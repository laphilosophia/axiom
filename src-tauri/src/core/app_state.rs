use std::path::PathBuf;

use crate::db::surreal_db::SurrealDb;
use crate::search::tantivy_engine::TantivyEngine;

pub struct AppState {
    pub workspace_path: Option<PathBuf>,
    pub db: Option<SurrealDb>,
    pub search_engine: Option<TantivyEngine>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            workspace_path: None,
            db: None,
            search_engine: None,
        }
    }

    pub async fn initialize(&mut self, workspace_path: PathBuf) -> anyhow::Result<()> {
        use tracing::info;

        self.workspace_path = Some(workspace_path.clone());

        // Initialize SurrealDB (in-memory)
        let db = SurrealDb::new(&workspace_path.join(".axiom/db")).await?;

        // Load existing documents from sidecar files for persistence
        let loaded = db.load_documents_from_workspace(&workspace_path).await?;
        if loaded > 0 {
            info!("Loaded {} documents from workspace", loaded);
        }

        self.db = Some(db);

        // Initialize Tantivy
        let search_engine = TantivyEngine::new(&workspace_path.join(".axiom/search"))?;
        self.search_engine = Some(search_engine);

        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
