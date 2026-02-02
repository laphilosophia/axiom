use thiserror::Error;

#[derive(Error, Debug)]
pub enum AxiomError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("Search engine error: {0}")]
    Search(String),

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Workspace not initialized")]
    WorkspaceNotInitialized,

    #[error("ML inference error: {0}")]
    MlInference(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AxiomError>;
