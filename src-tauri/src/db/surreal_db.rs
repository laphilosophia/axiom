use std::path::Path;

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use crate::core::document::{Document, DocumentStatus};
use crate::core::errors::{AxiomError, Result};
use crate::core::relationship::{DocumentRelationships, Relationship};

pub struct SurrealDb {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl SurrealDb {
    pub async fn new(_path: &Path) -> Result<Self> {
        // Using in-memory database
        // Data persistence is handled via sidecar JSON files
        // which are loaded on startup via scan_workspace
        let db = Surreal::new::<Mem>(())
            .await
            .map_err(|e| AxiomError::Database(format!("Failed to create database: {}", e)))?;

        // Use a namespace and database
        db.use_ns("axiom").use_db("documents").await?;

        // Initialize schema
        Self::initialize_schema(&db).await?;

        Ok(Self { db })
    }

    async fn initialize_schema(db: &Surreal<surrealdb::engine::local::Db>) -> Result<()> {
        // Define documents table as SCHEMALESS for flexibility
        db.query(
            r#"
            DEFINE TABLE documents SCHEMALESS;
            "#,
        )
        .await?;

        // Define relationships table as SCHEMALESS
        db.query(
            r#"
            DEFINE TABLE relationships SCHEMALESS;
            "#,
        )
        .await?;

        Ok(())
    }

    /// Load existing documents from sidecar files in the workspace
    /// This provides persistence by reading .sidecar.json files on startup
    pub async fn load_documents_from_workspace(&self, workspace_path: &Path) -> Result<usize> {
        use crate::core::metadata::SidecarMetadata;
        use std::fs;

        let sidecars_dir = workspace_path.join(".axiom").join("sidecars");
        let documents_dir = workspace_path.join("documents");

        if !sidecars_dir.exists() || !documents_dir.exists() {
            return Ok(0);
        }

        let mut loaded_count = 0;

        // Read all sidecar files
        if let Ok(entries) = fs::read_dir(&sidecars_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(sidecar) = serde_json::from_str::<SidecarMetadata>(&content) {
                            // Try to load the corresponding document content
                            let doc_path = documents_dir.join(format!("{}.md", sidecar.id));
                            let doc_content = fs::read_to_string(&doc_path).unwrap_or_default();

                            // Create Document from sidecar + content
                            let document = Document {
                                id: sidecar.id.clone(),
                                title: path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Untitled")
                                    .replace(".sidecar", "")
                                    .to_string(),
                                content: doc_content,
                                status: sidecar.status,
                                path: doc_path.to_string_lossy().to_string(),
                                tags: sidecar.tags,
                                created_at: sidecar.created_at,
                                updated_at: sidecar.updated_at,
                                content_hash: None,
                                embedding: None,
                            };

                            // Insert into database
                            if self.create_document(&document).await.is_ok() {
                                loaded_count += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(loaded_count)
    }

    pub async fn create_document(&self, document: &Document) -> Result<()> {
        // Remove id from content (SurrealDB uses tuple ID)
        let mut json_value =
            serde_json::to_value(document).map_err(|e| AxiomError::Serialization(e.to_string()))?;
        if let Some(obj) = json_value.as_object_mut() {
            obj.remove("id");
        }

        let _: Option<serde_json::Value> = self
            .db
            .create(("documents", &document.id))
            .content(json_value)
            .await?;
        Ok(())
    }

    pub async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        // Get as JSON and inject id back
        let result: Option<serde_json::Value> = self.db.select(("documents", id)).await?;

        match result {
            Some(mut json) => {
                if let Some(obj) = json.as_object_mut() {
                    obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
                }
                let doc: Document = serde_json::from_value(json)
                    .map_err(|e| AxiomError::Serialization(e.to_string()))?;
                Ok(Some(doc))
            }
            None => Ok(None),
        }
    }

    pub async fn update_document(&self, document: &Document) -> Result<()> {
        // Remove id from content (SurrealDB uses tuple ID)
        let mut json_value =
            serde_json::to_value(document).map_err(|e| AxiomError::Serialization(e.to_string()))?;
        if let Some(obj) = json_value.as_object_mut() {
            obj.remove("id");
        }

        let _: Option<serde_json::Value> = self
            .db
            .update(("documents", &document.id))
            .content(json_value)
            .await?;
        Ok(())
    }

    pub async fn delete_document(&self, id: &str) -> Result<()> {
        let _: Option<serde_json::Value> = self.db.delete(("documents", id)).await?;
        Ok(())
    }

    pub async fn list_documents(&self) -> Result<Vec<Document>> {
        // Get all as JSON and inject IDs back
        let results: Vec<serde_json::Value> = self.db.select("documents").await?;

        let mut documents = Vec::new();
        for mut json in results {
            // Extract id from SurrealDB's internal id field
            // SurrealDB may return id as: {"tb": "documents", "id": {"String": "doc_xxx"}}
            // or as a string directly, depending on version
            let id = if let Some(id_val) = json.get("id") {
                if let Some(s) = id_val.as_str() {
                    // Could be "documents:doc_xxx" format - extract the id part
                    if s.contains(':') {
                        s.split(':').last().unwrap_or(s).to_string()
                    } else {
                        s.to_string()
                    }
                } else if let Some(obj) = id_val.as_object() {
                    // Thing object case - try different structures
                    obj.get("id")
                        .and_then(|v| {
                            // Could be {"String": "doc_xxx"} or just "doc_xxx"
                            v.as_str().map(|s| s.to_string()).or_else(|| {
                                v.get("String")
                                    .and_then(|s| s.as_str())
                                    .map(|s| s.to_string())
                            })
                        })
                        .or_else(|| {
                            obj.get("Id")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        })
                        .unwrap_or_default()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            // Skip documents without valid IDs
            if id.is_empty() {
                continue;
            }

            if let Some(obj) = json.as_object_mut() {
                obj.insert("id".to_string(), serde_json::Value::String(id));
            }

            if let Ok(doc) = serde_json::from_value::<Document>(json) {
                documents.push(doc);
            }
        }
        Ok(documents)
    }

    pub async fn update_status(&self, id: &str, status: DocumentStatus) -> Result<()> {
        let doc = self
            .get_document(id)
            .await?
            .ok_or_else(|| AxiomError::DocumentNotFound(id.to_string()))?;

        if !doc.can_transition_to(&status) {
            return Err(AxiomError::InvalidStatusTransition {
                from: doc.status.to_string(),
                to: status.to_string(),
            });
        }

        // Update status using update_document to ensure consistent serialization
        let mut updated_doc = doc;
        updated_doc.status = status;
        updated_doc.updated_at = chrono::Utc::now();

        self.update_document(&updated_doc).await?;

        Ok(())
    }

    pub async fn create_relationship(&self, relationship: &Relationship) -> Result<()> {
        if !relationship.is_valid() {
            return Err(AxiomError::InvalidStatusTransition {
                from: relationship.from_document_id.clone(),
                to: relationship.to_document_id.clone(),
            });
        }

        let _: Option<Relationship> = self
            .db
            .create(("relationships", &relationship.id))
            .content(relationship)
            .await?;

        Ok(())
    }

    pub async fn get_document_relationships(&self, id: &str) -> Result<DocumentRelationships> {
        let mut rels = DocumentRelationships::new();

        // Get outgoing relationships (this document -> others)
        let outgoing: Vec<Relationship> = self
            .db
            .query("SELECT * FROM relationships WHERE from_document_id = $id")
            .bind(("id", id))
            .await?
            .take(0)?;

        for rel in outgoing {
            rels.add_outgoing(rel.to_document_id.clone(), rel.relationship_type);
        }

        // Get incoming relationships (others -> this document)
        let incoming: Vec<Relationship> = self
            .db
            .query("SELECT * FROM relationships WHERE to_document_id = $id")
            .bind(("id", id))
            .await?
            .take(0)?;

        for rel in incoming {
            rels.add_incoming(rel.from_document_id.clone(), rel.relationship_type);
        }

        Ok(rels)
    }

    pub async fn find_similar_by_embedding(
        &self,
        id: &str,
        _embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<(String, f32)>> {
        // Simplified similarity search - in production, use vector similarity
        let sql = r#"
            SELECT id, embedding FROM documents
            WHERE id != $id AND embedding IS NOT NULL
            LIMIT $limit
        "#;

        let results: Vec<(String, Option<Vec<f32>>)> = self
            .db
            .query(sql)
            .bind(("id", id))
            .bind(("limit", limit as i64))
            .await?
            .take(0)?;

        // Placeholder similarity calculation
        let similar: Vec<(String, f32)> = results
            .into_iter()
            .map(|(id, _)| (id, 0.8)) // Placeholder score
            .collect();

        Ok(similar)
    }
}
