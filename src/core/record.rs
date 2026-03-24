//! Record domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A record stored in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub revision: u32,
    pub updated_at: DateTime<Utc>,
}

impl Record {
    /// Create a new record with revision 1
    pub fn new(id: String, title: String, body: String, tags: Vec<String>) -> Self {
        Self {
            id,
            title,
            body,
            tags,
            revision: 1,
            updated_at: Utc::now(),
        }
    }

    /// Create an updated version of this record with incremented revision
    pub fn update(&self, title: String, body: String, tags: Vec<String>) -> Self {
        Self {
            id: self.id.clone(),
            title,
            body,
            tags,
            revision: self.revision + 1,
            updated_at: Utc::now(),
        }
    }
}

/// Input data for creating or updating a record
#[derive(Debug, Clone, Deserialize)]
pub struct RecordInput {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_record_has_revision_1() {
        let record = Record::new(
            "test-id".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            vec!["tag1".to_string()],
        );
        assert_eq!(record.revision, 1);
    }

    #[test]
    fn update_increments_revision() {
        let record = Record::new(
            "test-id".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            vec![],
        );
        let updated = record.update("New".to_string(), "New body".to_string(), vec![]);
        assert_eq!(updated.revision, 2);
        assert_eq!(updated.id, "test-id");
    }
}
