use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::document::DocumentStatus;

/// Sidecar metadata file format for portability
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidecarMetadata {
    pub version: String,
    pub id: String,
    pub status: DocumentStatus,
    pub tags: Vec<String>,
    pub relationships: RelationshipMetadata,
    pub embedding_checksum: Option<String>,
    pub metadata_checksum: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipMetadata {
    pub supersedes: Vec<String>,
    pub references: Vec<String>,
}

impl SidecarMetadata {
    pub fn new(document_id: String) -> Self {
        let now = Utc::now();
        Self {
            version: "1.0".to_string(),
            id: document_id,
            status: DocumentStatus::Draft,
            tags: Vec::new(),
            relationships: RelationshipMetadata::default(),
            embedding_checksum: None,
            metadata_checksum: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_checksum(&mut self) {
        // Simple checksum based on metadata fields
        let data = format!("{}-{}-{:?}", self.id, self.status, self.tags);
        self.metadata_checksum = Some(calculate_simple_hash(&data));
        self.updated_at = Utc::now();
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

fn calculate_simple_hash(data: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(&hasher.finalize()[..8])
}

/// In-memory metadata cache for performance
pub struct MetadataCache {
    cache: HashMap<String, SidecarMetadata>,
}

impl MetadataCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, id: &str) -> Option<&SidecarMetadata> {
        self.cache.get(id)
    }

    pub fn insert(&mut self, id: String, metadata: SidecarMetadata) {
        self.cache.insert(id, metadata);
    }

    pub fn remove(&mut self, id: &str) -> Option<SidecarMetadata> {
        self.cache.remove(id)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for MetadataCache {
    fn default() -> Self {
        Self::new()
    }
}
