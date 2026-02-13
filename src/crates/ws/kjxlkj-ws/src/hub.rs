use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::protocol::ServerMessage;

/// Broadcast hub for WebSocket event distribution.
/// Each stream (note or workspace) has its own broadcast channel.
pub struct WsHub {
    note_channels: RwLock<HashMap<Uuid, broadcast::Sender<Arc<ServerMessage>>>>,
    workspace_channels: RwLock<HashMap<Uuid, broadcast::Sender<Arc<ServerMessage>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            note_channels: RwLock::new(HashMap::new()),
            workspace_channels: RwLock::new(HashMap::new()),
        }
    }

    pub async fn subscribe_note(
        &self,
        note_id: Uuid,
    ) -> broadcast::Receiver<Arc<ServerMessage>> {
        let mut channels = self.note_channels.write().await;
        let sender = channels
            .entry(note_id)
            .or_insert_with(|| broadcast::channel(256).0);
        sender.subscribe()
    }

    pub async fn subscribe_workspace(
        &self,
        workspace_id: Uuid,
    ) -> broadcast::Receiver<Arc<ServerMessage>> {
        let mut channels = self.workspace_channels.write().await;
        let sender = channels
            .entry(workspace_id)
            .or_insert_with(|| broadcast::channel(256).0);
        sender.subscribe()
    }

    pub async fn broadcast_note_event(&self, note_id: Uuid, msg: ServerMessage) {
        let channels = self.note_channels.read().await;
        if let Some(sender) = channels.get(&note_id) {
            let _ = sender.send(Arc::new(msg));
        }
    }

    pub async fn broadcast_workspace_event(
        &self,
        workspace_id: Uuid,
        msg: ServerMessage,
    ) {
        let channels = self.workspace_channels.read().await;
        if let Some(sender) = channels.get(&workspace_id) {
            let _ = sender.send(Arc::new(msg));
        }
    }
}

impl Default for WsHub {
    fn default() -> Self {
        Self::new()
    }
}
