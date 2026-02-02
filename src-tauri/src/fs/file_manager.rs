use std::fs;
use std::path::{Path, PathBuf};

use crate::core::document::Document;
use crate::core::errors::{AxiomError, Result};
use crate::core::metadata::SidecarMetadata;

pub struct FileManager {
    workspace_path: PathBuf,
}

impl FileManager {
    pub fn new(workspace_path: PathBuf) -> Self {
        Self { workspace_path }
    }

    pub fn document_path(&self, document_id: &str) -> PathBuf {
        self.workspace_path
            .join("documents")
            .join(format!("{}.md", document_id))
    }

    pub fn sidecar_path(&self, document_id: &str) -> PathBuf {
        self.workspace_path
            .join("documents")
            .join(format!("{}.sidecar.json", document_id))
    }

    pub async fn write_document(&self, document: &Document) -> Result<()> {
        let doc_path = self.document_path(&document.id);
        let sidecar_path = self.sidecar_path(&document.id);

        // Ensure directory exists
        if let Some(parent) = doc_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write content atomically
        self.atomic_write(&doc_path, &document.content).await?;

        // Write sidecar metadata
        let mut sidecar = SidecarMetadata::new(document.id.clone());
        sidecar.status = document.status.clone();
        sidecar.tags = document.tags.clone();
        sidecar.created_at = document.created_at;
        sidecar.updated_at = document.updated_at;
        sidecar.update_checksum();

        let sidecar_json = sidecar.to_json()?;
        self.atomic_write(&sidecar_path, &sidecar_json).await?;

        Ok(())
    }

    pub async fn read_document(&self, document_id: &str) -> Result<String> {
        let doc_path = self.document_path(document_id);
        fs::read_to_string(&doc_path).map_err(|e| AxiomError::FileSystem(e.to_string()))
    }

    pub async fn read_sidecar(&self, document_id: &str) -> Result<Option<SidecarMetadata>> {
        let sidecar_path = self.sidecar_path(document_id);

        if !sidecar_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&sidecar_path)?;
        let metadata = SidecarMetadata::from_json(&content)?;
        Ok(Some(metadata))
    }

    pub async fn delete_document(&self, document_id: &str) -> Result<()> {
        let doc_path = self.document_path(document_id);
        let sidecar_path = self.sidecar_path(document_id);

        if doc_path.exists() {
            fs::remove_file(&doc_path)?;
        }

        if sidecar_path.exists() {
            fs::remove_file(&sidecar_path)?;
        }

        Ok(())
    }

    pub async fn list_documents(&self) -> Result<Vec<(String, PathBuf)>> {
        let docs_dir = self.workspace_path.join("documents");

        if !docs_dir.exists() {
            return Ok(Vec::new());
        }

        let mut documents = Vec::new();
        for entry in fs::read_dir(&docs_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| AxiomError::FileSystem("Invalid filename".to_string()))?;

                documents.push((stem, path));
            }
        }

        Ok(documents)
    }

    async fn atomic_write(&self, path: &Path, content: &str) -> Result<()> {
        // Write to temp file first
        let temp_path = path.with_extension("tmp");

        fs::write(&temp_path, content)?;

        // Atomic rename
        fs::rename(&temp_path, path)?;

        Ok(())
    }

    pub fn check_disk_space(&self) -> Result<u64> {
        // Platform-specific disk space check
        // For now, return a placeholder value
        Ok(1024 * 1024 * 1024) // 1GB placeholder
    }
}
