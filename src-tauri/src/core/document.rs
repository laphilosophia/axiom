use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DocumentStatus {
    Draft,
    Active,
    Superseded,
    Archived,
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentStatus::Draft => write!(f, "draft"),
            DocumentStatus::Active => write!(f, "active"),
            DocumentStatus::Superseded => write!(f, "superseded"),
            DocumentStatus::Archived => write!(f, "archived"),
        }
    }
}

impl std::str::FromStr for DocumentStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(DocumentStatus::Draft),
            "active" => Ok(DocumentStatus::Active),
            "superseded" => Ok(DocumentStatus::Superseded),
            "archived" => Ok(DocumentStatus::Archived),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub status: DocumentStatus,
    pub path: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content_hash: Option<String>,
    pub embedding: Option<Vec<f32>>,
}

impl Document {
    pub fn new(id: String, title: String, path: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            content: String::new(),
            status: DocumentStatus::Draft,
            path,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            content_hash: None,
            embedding: None,
        }
    }

    pub fn can_transition_to(&self, _new_status: &DocumentStatus) -> bool {
        // Allow all transitions - user has full control over document lifecycle
        true
    }

    pub fn update_content(&mut self, content: String) {
        self.content_hash = Some(calculate_hash(&content));
        self.content = content;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    pub fn update_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
        self.updated_at = Utc::now();
    }

    #[allow(dead_code)]
    pub fn set_status(&mut self, status: DocumentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn is_readonly(&self) -> bool {
        matches!(
            self.status,
            DocumentStatus::Superseded | DocumentStatus::Archived
        )
    }
}

fn calculate_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}
