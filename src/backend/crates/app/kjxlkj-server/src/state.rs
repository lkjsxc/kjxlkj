use std::sync::Arc;

use serde_json::Value;
use sqlx::{Pool, Sqlite};
use tokio::sync::{broadcast, RwLock};

use crate::model::Store;

#[derive(Debug, Clone)]
pub struct WsEnvelope {
    pub stream_id: String,
    pub payload: Value,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
    pub store: Arc<RwLock<Store>>,
    pub ws_tx: broadcast::Sender<WsEnvelope>,
}

impl AppState {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        let (ws_tx, _) = broadcast::channel(512);
        Self {
            db_pool,
            store: Arc::new(RwLock::new(Store::default())),
            ws_tx,
        }
    }
}
