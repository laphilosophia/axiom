use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::core::app_state::AppState;
use crate::core::document::Document;
use crate::core::errors::Result;

#[derive(serde::Serialize)]
pub struct SimilarityResult {
    pub document: Document,
    pub similarity: f32,
    pub reason: String,
}

#[tauri::command]
pub async fn search_documents(
    state: State<'_, Arc<Mutex<AppState>>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<Document>> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let limit = limit.unwrap_or(10);

    // Use Tantivy for full-text search
    if let Some(search_engine) = state.search_engine.as_ref() {
        let results = search_engine.search(&query, limit)?;

        let mut documents = Vec::new();
        for result in results {
            // Parse the JSON result to get document ID
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&result.doc_json) {
                if let Some(id) = json.get("id").and_then(|v| v.as_str()) {
                    if let Some(doc) = db.get_document(id).await? {
                        documents.push(doc);
                    }
                }
            }
        }

        return Ok(documents);
    }

    // Fallback: Return all documents if search is not available
    let all_docs = db.list_documents().await?;
    let filtered: Vec<Document> = all_docs
        .into_iter()
        .filter(|d| {
            d.title.to_lowercase().contains(&query.to_lowercase())
                || d.content.to_lowercase().contains(&query.to_lowercase())
        })
        .take(limit)
        .collect();

    Ok(filtered)
}

#[tauri::command]
pub async fn find_similar_documents(
    state: State<'_, Arc<Mutex<AppState>>>,
    id: String,
    threshold: Option<f32>,
    limit: Option<usize>,
) -> Result<Vec<SimilarityResult>> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let threshold = threshold.unwrap_or(0.75);
    let limit = limit.unwrap_or(5);

    // Get the source document
    let source_doc =
        db.get_document(&id)
            .await?
            .ok_or(crate::core::errors::AxiomError::DocumentNotFound(
                id.clone(),
            ))?;

    // If document has no embedding, return empty results
    let source_embedding = match source_doc.embedding {
        Some(emb) => emb,
        None => return Ok(Vec::new()),
    };

    // Find similar documents
    let similar = db
        .find_similar_by_embedding(&id, &source_embedding, limit * 2)
        .await?;

    let mut results = Vec::new();
    for (doc_id, similarity) in similar {
        if similarity >= threshold {
            if let Some(doc) = db.get_document(&doc_id).await? {
                let reason = if similarity > 0.9 {
                    "Very high semantic similarity".to_string()
                } else if similarity > 0.8 {
                    "High semantic similarity".to_string()
                } else {
                    "Moderate semantic similarity".to_string()
                };

                results.push(SimilarityResult {
                    document: doc,
                    similarity,
                    reason,
                });
            }

            if results.len() >= limit {
                break;
            }
        }
    }

    Ok(results)
}
