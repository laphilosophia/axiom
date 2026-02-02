use std::path::Path;

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use crate::core::document::{Document, DocumentStatus};
use crate::core::errors::{AxiomError, Result};
use crate::core::relationship::{DocumentRelationships, Relationship, RelationshipType};

pub struct SurrealDb {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl SurrealDb {
    pub async fn new(_path: &Path) -> Result<Self> {
        // Create in-memory database for now
        // In production, use file-based storage
        let db = Surreal::new::<Mem>(()).await?;

        // Use a namespace and database
        db.use_ns("axiom").use_db("documents").await?;

        // Initialize schema
        Self::initialize_schema(&db).await?;

        Ok(Self { db })
    }

    async fn initialize_schema(db: &Surreal<surrealdb::engine::local::Db>) -> Result<()> {
        // Define documents table
        db.query(
            r#"
            DEFINE TABLE documents SCHEMAFULL;
            DEFINE FIELD id ON documents TYPE string;
            DEFINE FIELD title ON documents TYPE string;
            DEFINE FIELD content ON documents TYPE string;
            DEFINE FIELD status ON documents TYPE string;
            DEFINE FIELD path ON documents TYPE string;
            DEFINE FIELD tags ON documents TYPE array;
            DEFINE FIELD created_at ON documents TYPE datetime;
            DEFINE FIELD updated_at ON documents TYPE datetime;
            DEFINE FIELD content_hash ON documents TYPE option<string>;
            DEFINE FIELD embedding ON documents TYPE option<array>;
            "#,
        )
        .await?;

        // Define relationships table
        db.query(
            r#"
            DEFINE TABLE relationships SCHEMAFULL;
            DEFINE FIELD id ON relationships TYPE string;
            DEFINE FIELD from_document_id ON relationships TYPE string;
            DEFINE FIELD to_document_id ON relationships TYPE string;
            DEFINE FIELD relationship_type ON relationships TYPE string;
            DEFINE FIELD created_at ON relationships TYPE datetime;
            "#,
        )
        .await?;

        Ok(())
    }

    pub async fn create_document(&self, document: &Document) -> Result<()> {
        let _: Option<Document> = self
            .db
            .create(("documents", &document.id))
            .content(document)
            .await?;
        Ok(())
    }

    pub async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        let doc: Option<Document> = self.db.select(("documents", id)).await?;
        Ok(doc)
    }

    pub async fn update_document(&self, document: &Document) -> Result<()> {
        let _: Option<Document> = self
            .db
            .update(("documents", &document.id))
            .content(document)
            .await?;
        Ok(())
    }

    pub async fn delete_document(&self, id: &str) -> Result<()> {
        let _: Option<Document> = self.db.delete(("documents", id)).await?;
        Ok(())
    }

    pub async fn list_documents(&self) -> Result<Vec<Document>> {
        let docs: Vec<Document> = self.db.select("documents").await?;
        Ok(docs)
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

        let sql = r#"
            UPDATE documents:$id SET status = $status, updated_at = time::now()
        "#;

        self.db
            .query(sql)
            .bind(("id", id))
            .bind(("status", status.to_string()))
            .await?;

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
