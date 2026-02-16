use kjxlkj_db::repo_automation;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::debug;

/// In-memory KV store backed by database persistence.
/// Implements carry-over memory per docs/spec/technical/librarian-agent.md.
pub struct KvMemory {
    agent_name: String,
    workspace_id: Uuid,
}

impl KvMemory {
    pub fn new(agent_name: &str, workspace_id: Uuid) -> Self {
        Self {
            agent_name: agent_name.to_string(),
            workspace_id,
        }
    }

    pub async fn get(&self, pool: &PgPool, key: &str) -> Result<Option<serde_json::Value>, sqlx::Error> {
        repo_automation::kv_get(pool, &self.agent_name, self.workspace_id, key).await
    }

    pub async fn set(&self, pool: &PgPool, key: &str, value: &serde_json::Value) -> Result<(), sqlx::Error> {
        debug!("kv_set: {}={}", key, value);
        repo_automation::kv_set(pool, &self.agent_name, self.workspace_id, key, value).await
    }

    pub async fn delete(&self, pool: &PgPool, key: &str) -> Result<(), sqlx::Error> {
        debug!("kv_delete: {}", key);
        repo_automation::kv_delete(pool, &self.agent_name, self.workspace_id, key).await
    }

    pub async fn list_all(&self, pool: &PgPool) -> Result<Vec<(String, serde_json::Value)>, sqlx::Error> {
        let rows = repo_automation::kv_list(pool, &self.agent_name, self.workspace_id).await?;
        Ok(rows.into_iter().map(|r| (r.key, r.value)).collect())
    }
}
