use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone)]
#[serde(tag = "type", content = "message")]
pub enum AxiomError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Search engine error: {0}")]
    Search(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Workspace not initialized")]
    WorkspaceNotInitialized,

    #[allow(dead_code)]
    #[error("ML error: {0}")]
    ML(String),

    #[allow(dead_code)]
    #[error("ML inference error: {0}")]
    MlInference(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<surrealdb::Error> for AxiomError {
    fn from(err: surrealdb::Error) -> Self {
        AxiomError::Database(err.to_string())
    }
}

impl From<std::io::Error> for AxiomError {
    fn from(err: std::io::Error) -> Self {
        AxiomError::FileSystem(err.to_string())
    }
}

impl From<serde_json::Error> for AxiomError {
    fn from(err: serde_json::Error) -> Self {
        AxiomError::Serialization(err.to_string())
    }
}

impl From<anyhow::Error> for AxiomError {
    fn from(err: anyhow::Error) -> Self {
        AxiomError::General(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AxiomError>;
