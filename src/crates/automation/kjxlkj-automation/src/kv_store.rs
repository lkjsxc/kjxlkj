/// Agent KV memory store per /docs/spec/technical/librarian-agent.md
///
/// Memory model: KV store persisted between loops.
/// Agent must be able to freely create/update/delete KV entries.
use kjxlkj_domain::DomainError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// In-memory KV store that can be serialized to disk.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KvStore {
    pub entries: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load from file path.
    pub fn load(path: &str) -> Result<Self, DomainError> {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| DomainError::AgentMemoryStoreError(format!("parse: {e}"))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::new()),
            Err(e) => Err(DomainError::AgentMemoryStoreError(format!("read: {e}"))),
        }
    }

    /// Save to file path.
    pub fn save(&self, path: &str) -> Result<(), DomainError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| DomainError::AgentMemoryStoreError(format!("serialize: {e}")))?;
        std::fs::write(path, content)
            .map_err(|e| DomainError::AgentMemoryStoreError(format!("write: {e}")))
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_02_kv_memory_persists() {
        // Acceptance: AGENT-02
        let mut store = KvStore::new();
        store.set("think_log".into(), "initial".into());
        store.set("plan".into(), "step1".into());
        assert_eq!(store.get("think_log"), Some(&"initial".to_string()));

        // Simulate persistence round-trip
        let json = serde_json::to_string(&store).unwrap();
        let loaded: KvStore = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.get("think_log"), Some(&"initial".to_string()));
        assert_eq!(loaded.get("plan"), Some(&"step1".to_string()));
    }

    #[test]
    fn test_kv_delete() {
        let mut store = KvStore::new();
        store.set("key".into(), "val".into());
        assert!(store.delete("key"));
        assert!(store.get("key").is_none());
        assert!(!store.delete("nonexistent"));
    }

    #[test]
    fn test_kv_load_missing_file() {
        let store = KvStore::load("/nonexistent/path.json").unwrap();
        assert!(store.entries.is_empty());
    }
}
