#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

mod commands;
mod core;
mod db;
mod fs;
mod ml;
mod search;

use crate::core::app_state::AppState;

fn main() {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Axiom - Document Orchestrator");

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(AppState::new())))
        .invoke_handler(tauri::generate_handler![
            // Document commands
            commands::documents::get_documents,
            commands::documents::create_document,
            commands::documents::update_document,
            commands::documents::update_document_status,
            commands::documents::delete_document,
            // Search commands
            commands::search::search_documents,
            commands::search::find_similar_documents,
            commands::search::search_with_snippets,
            // Relationship commands
            commands::relationships::get_document_relationships,
            commands::relationships::create_relationship,
            // System commands
            commands::system::get_workspace_path,
            commands::system::set_workspace_path,
            commands::system::get_home_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
