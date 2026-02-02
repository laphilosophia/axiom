use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::core::app_state::AppState;
use crate::core::document::Document;
use crate::core::errors::Result;
use crate::core::relationship::{DocumentRelationships, Relationship, RelationshipType};

#[tauri::command]
pub async fn get_document_relationships(
    state: State<'_, Arc<Mutex<AppState>>>,
    id: String,
) -> Result<DocumentRelationships> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let relationships = db.get_document_relationships(&id).await?;
    Ok(relationships)
}

#[tauri::command]
pub async fn create_relationship(
    state: State<'_, Arc<Mutex<AppState>>>,
    from_id: String,
    to_id: String,
    relationship: String,
) -> Result<()> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    // Verify both documents exist
    if db.get_document(&from_id).await?.is_none() {
        return Err(crate::core::errors::AxiomError::DocumentNotFound(from_id));
    }
    if db.get_document(&to_id).await?.is_none() {
        return Err(crate::core::errors::AxiomError::DocumentNotFound(to_id));
    }

    let rel_type = match relationship.as_str() {
        "supersedes" => RelationshipType::Supersedes,
        "references" => RelationshipType::References,
        _ => {
            return Err(crate::core::errors::AxiomError::InvalidStatusTransition {
                from: relationship,
                to: "valid type".to_string(),
            })
        }
    };

    let rel = Relationship::new(from_id, to_id, rel_type);
    db.create_relationship(&rel).await?;

    Ok(())
}
