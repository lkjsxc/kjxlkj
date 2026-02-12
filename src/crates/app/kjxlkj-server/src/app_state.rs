use std::{collections::HashMap, sync::Arc};

use sqlx::PgPool;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::{config::Config, ws::ServerEvent};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: PgPool,
    pub topics: Arc<RwLock<HashMap<Uuid, broadcast::Sender<ServerEvent>>>>,
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        Self {
            config,
            pool,
            topics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn topic_sender(&self, note_id: Uuid) -> broadcast::Sender<ServerEvent> {
        let mut guard = self.topics.write().await;
        if let Some(existing) = guard.get(&note_id) {
            return existing.clone();
        }
        let (tx, _rx) = broadcast::channel(512);
        guard.insert(note_id, tx.clone());
        tx
    }

    pub async fn publish(&self, note_id: Uuid, event: ServerEvent) {
        let sender = self.topic_sender(note_id).await;
        let _ = sender.send(event);
    }
}
