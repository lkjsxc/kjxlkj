use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub revision: u64,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordInput {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}
