use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Supersedes,
    References,
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipType::Supersedes => write!(f, "supersedes"),
            RelationshipType::References => write!(f, "references"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub from_document_id: String,
    pub to_document_id: String,
    pub relationship_type: RelationshipType,
    pub created_at: DateTime<Utc>,
}

impl Relationship {
    pub fn new(
        from_document_id: String,
        to_document_id: String,
        relationship_type: RelationshipType,
    ) -> Self {
        let id = format!(
            "{}-{}-{}",
            from_document_id, relationship_type, to_document_id
        );
        Self {
            id,
            from_document_id,
            to_document_id,
            relationship_type,
            created_at: Utc::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        // Prevent self-relationships
        if self.from_document_id == self.to_document_id {
            return false;
        }

        // Additional validation rules can be added here
        true
    }
}

/// Collection of relationships for a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentRelationships {
    pub supersedes: Vec<String>,       // Documents this one supersedes
    pub references: Vec<String>,       // Documents this one references
    pub superseded_by: Option<String>, // Document that supersedes this one
    pub referenced_by: Vec<String>,    // Documents that reference this one
}

impl DocumentRelationships {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_outgoing(&mut self, doc_id: String, rel_type: RelationshipType) {
        match rel_type {
            RelationshipType::Supersedes => {
                if !self.supersedes.contains(&doc_id) {
                    self.supersedes.push(doc_id);
                }
            }
            RelationshipType::References => {
                if !self.references.contains(&doc_id) {
                    self.references.push(doc_id);
                }
            }
        }
    }

    pub fn add_incoming(&mut self, doc_id: String, rel_type: RelationshipType) {
        match rel_type {
            RelationshipType::Supersedes => {
                self.superseded_by = Some(doc_id);
            }
            RelationshipType::References => {
                if !self.referenced_by.contains(&doc_id) {
                    self.referenced_by.push(doc_id);
                }
            }
        }
    }

    pub fn remove_outgoing(&mut self, doc_id: &str, rel_type: RelationshipType) {
        match rel_type {
            RelationshipType::Supersedes => {
                self.supersedes.retain(|id| id != doc_id);
            }
            RelationshipType::References => {
                self.references.retain(|id| id != doc_id);
            }
        }
    }

    pub fn remove_incoming(&mut self, doc_id: &str, rel_type: RelationshipType) {
        match rel_type {
            RelationshipType::Supersedes => {
                if self.superseded_by.as_deref() == Some(doc_id) {
                    self.superseded_by = None;
                }
            }
            RelationshipType::References => {
                self.referenced_by.retain(|id| id != doc_id);
            }
        }
    }
}
