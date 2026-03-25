//! Database row models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub slug: String,
    pub body: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A revision of a record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRevision {
    pub revision_number: i32,
    pub body: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}
