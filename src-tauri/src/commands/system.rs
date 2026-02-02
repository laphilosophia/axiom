use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::core::app_state::AppState;
use crate::core::errors::Result;

#[tauri::command]
pub async fn get_workspace_path(state: State<'_, Arc<Mutex<AppState>>>) -> Result<Option<String>> {
    let state = state.lock().await;
    Ok(state
        .workspace_path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn set_workspace_path(
    state: State<'_, Arc<Mutex<AppState>>>,
    path: String,
) -> Result<()> {
    let mut state = state.lock().await;

    let path = PathBuf::from(path);

    // Initialize the workspace
    state.initialize(path).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_home_dir() -> Result<String> {
    let home = dirs::home_dir().ok_or_else(|| {
        crate::core::errors::AxiomError::FileSystem("Could not find home directory".to_string())
    })?;
    Ok(home.to_string_lossy().to_string())
}
