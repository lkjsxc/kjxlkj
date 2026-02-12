use std::{collections::HashMap, sync::Arc};

use sqlx::PgPool;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::{config::Config, ws::ServerEvent};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: PgPool,
    pub topics: Arc<RwLock<HashMap<String, broadcast::Sender<ServerEvent>>>>,
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        Self {
            config,
            pool,
            topics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn note_stream_id(note_id: Uuid) -> String {
        format!("note:{note_id}")
    }

    pub fn workspace_stream_id(workspace_id: Uuid) -> String {
        format!("workspace:{workspace_id}")
    }

    pub async fn topic_sender(&self, stream_id: &str) -> broadcast::Sender<ServerEvent> {
        let mut guard = self.topics.write().await;
        if let Some(existing) = guard.get(stream_id) {
            return existing.clone();
        }
        let (tx, _rx) = broadcast::channel(512);
        guard.insert(stream_id.to_string(), tx.clone());
        tx
    }

    pub async fn publish(&self, note_id: Uuid, event: ServerEvent) {
        let sender = self.topic_sender(&Self::note_stream_id(note_id)).await;
        let _ = sender.send(event);
    }

    pub async fn publish_workspace(&self, workspace_id: Uuid, event: ServerEvent) {
        let sender = self
            .topic_sender(&Self::workspace_stream_id(workspace_id))
            .await;
        let _ = sender.send(event);
    }
}
