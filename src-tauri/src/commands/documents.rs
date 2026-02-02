use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::core::app_state::AppState;
use crate::core::document::{Document, DocumentStatus};
use crate::core::errors::Result;

#[tauri::command]
pub async fn get_documents(state: State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<Document>> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let documents = db.list_documents().await?;
    Ok(documents)
}

#[tauri::command]
pub async fn create_document(
    state: State<'_, Arc<Mutex<AppState>>>,
    title: String,
    content: String,
) -> Result<Document> {
    let mut state = state.lock().await;

    let workspace = state
        .workspace_path
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let db = state
        .db
        .as_mut()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    // Generate ID
    let id = format!(
        "doc_{}",
        Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .get(0..16)
            .unwrap_or("")
    );

    // Create document path
    let path = workspace
        .join("documents")
        .join(format!("{}.md", id))
        .to_string_lossy()
        .to_string();

    let mut document = Document::new(id.clone(), title.clone(), path);
    document.content = content;

    // Save to database
    db.create_document(&document).await?;

    // Save to filesystem
    let file_manager = crate::fs::file_manager::FileManager::new(workspace.clone());
    file_manager.write_document(&document).await?;

    // Index for search
    if let Some(search_engine) = state.search_engine.as_mut() {
        search_engine.add_document(&document)?;
        search_engine.commit()?;
    }

    Ok(document)
}

#[tauri::command]
pub async fn update_document(
    state: State<'_, Arc<Mutex<AppState>>>,
    id: String,
    title: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<()> {
    let mut state = state.lock().await;

    let workspace = state
        .workspace_path
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let db = state
        .db
        .as_mut()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    // Get existing document
    let mut document =
        db.get_document(&id)
            .await?
            .ok_or(crate::core::errors::AxiomError::DocumentNotFound(
                id.clone(),
            ))?;

    // Check if document is read-only
    if document.is_readonly() {
        return Err(crate::core::errors::AxiomError::InvalidStatusTransition {
            from: document.status.to_string(),
            to: "modified".to_string(),
        });
    }

    // Update fields
    if let Some(title) = title {
        document.update_title(title);
    }
    if let Some(content) = content {
        document.update_content(content);
    }
    if let Some(tags) = tags {
        document.update_tags(tags);
    }

    // Save to database
    db.update_document(&document).await?;

    // Save to filesystem
    let file_manager = crate::fs::file_manager::FileManager::new(workspace.clone());
    file_manager.write_document(&document).await?;

    // Update search index
    if let Some(search_engine) = state.search_engine.as_mut() {
        search_engine.update_document(&document)?;
        search_engine.commit()?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_document_status(
    state: State<'_, Arc<Mutex<AppState>>>,
    id: String,
    status: String,
) -> Result<()> {
    let state = state.lock().await;

    let db = state
        .db
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let status = status.parse::<DocumentStatus>().map_err(|e| {
        crate::core::errors::AxiomError::InvalidStatusTransition {
            from: id.clone(),
            to: e,
        }
    })?;

    db.update_status(&id, status).await?;

    Ok(())
}

#[tauri::command]
pub async fn delete_document(state: State<'_, Arc<Mutex<AppState>>>, id: String) -> Result<()> {
    let mut state = state.lock().await;

    let workspace = state
        .workspace_path
        .as_ref()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    let db = state
        .db
        .as_mut()
        .ok_or(crate::core::errors::AxiomError::WorkspaceNotInitialized)?;

    // Delete from filesystem
    let file_manager = crate::fs::file_manager::FileManager::new(workspace.clone());
    file_manager.delete_document(&id).await?;

    // Delete from search index
    if let Some(search_engine) = state.search_engine.as_mut() {
        search_engine.delete_document(&id)?;
        search_engine.commit()?;
    }

    // Delete from database
    db.delete_document(&id).await?;

    Ok(())
}
