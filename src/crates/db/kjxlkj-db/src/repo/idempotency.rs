//! Idempotency key repository

use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Cached idempotency key entry
#[derive(Debug, Clone)]
pub struct IdempotencyEntry {
    pub key: String,
    pub response_hash: String,
    pub created_at: DateTime<Utc>,
}

/// In-memory idempotency key repository
#[derive(Debug, Clone)]
pub struct IdempotencyRepo {
    entries: Arc<RwLock<HashMap<String, IdempotencyEntry>>>,
    /// Keep last N keys for deduplication
    max_entries: usize,
}

impl IdempotencyRepo {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
        }
    }

    /// Check if key exists and return cached response hash
    pub async fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.read().await;
        entries.get(key).map(|e| e.response_hash.clone())
    }

    /// Store idempotency key with response hash
    pub async fn set(&self, key: &str, response_hash: &str) {
        let mut entries = self.entries.write().await;
        
        // Evict oldest if at capacity
        if entries.len() >= self.max_entries {
            if let Some(oldest_key) = entries
                .iter()
                .min_by_key(|(_, e)| e.created_at)
                .map(|(k, _)| k.clone())
            {
                entries.remove(&oldest_key);
            }
        }

        entries.insert(key.to_string(), IdempotencyEntry {
            key: key.to_string(),
            response_hash: response_hash.to_string(),
            created_at: Utc::now(),
        });
    }

    /// Check if key exists
    pub async fn exists(&self, key: &str) -> bool {
        let entries = self.entries.read().await;
        entries.contains_key(key)
    }
}

impl Default for IdempotencyRepo {
    fn default() -> Self {
        Self::new(100)
    }
}
