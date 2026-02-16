/// WebSocket broadcast registry per /docs/spec/api/websocket.md (IMP-ARC-02)
///
/// Cross-session broadcast for note events, workspace events, and
/// automation events. Each connected WS client registers its sender;
/// the registry fans out server messages to all subscribers of a stream.
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Broadcast channel capacity per stream.
const CHANNEL_CAPACITY: usize = 256;

/// A serialized server message ready for broadcast.
pub type BroadcastPayload = String;

/// Registry of broadcast channels keyed by stream (note or workspace ID).
///
/// Per /docs/spec/api/websocket.md: events are delivered to all
/// subscribers of a note or workspace stream in order.
#[derive(Clone)]
pub struct BroadcastRegistry {
    note_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<BroadcastPayload>>>>,
    workspace_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<BroadcastPayload>>>>,
}

impl BroadcastRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            note_channels: Arc::new(RwLock::new(HashMap::new())),
            workspace_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe to note events. Returns a receiver that will get
    /// all future broadcasts for this note stream.
    pub async fn subscribe_note(
        &self,
        note_id: Uuid,
    ) -> broadcast::Receiver<BroadcastPayload> {
        let mut channels = self.note_channels.write().await;
        let sender = channels
            .entry(note_id)
            .or_insert_with(|| broadcast::channel(CHANNEL_CAPACITY).0);
        sender.subscribe()
    }

    /// Subscribe to workspace events.
    pub async fn subscribe_workspace(
        &self,
        workspace_id: Uuid,
    ) -> broadcast::Receiver<BroadcastPayload> {
        let mut channels = self.workspace_channels.write().await;
        let sender = channels
            .entry(workspace_id)
            .or_insert_with(|| broadcast::channel(CHANNEL_CAPACITY).0);
        sender.subscribe()
    }

    /// Broadcast a payload to all subscribers of a note stream.
    /// Returns the number of receivers that received the message.
    pub async fn broadcast_note(
        &self,
        note_id: Uuid,
        payload: BroadcastPayload,
    ) -> usize {
        let channels = self.note_channels.read().await;
        if let Some(sender) = channels.get(&note_id) {
            sender.send(payload).unwrap_or(0)
        } else {
            0
        }
    }

    /// Broadcast a payload to all subscribers of a workspace stream.
    pub async fn broadcast_workspace(
        &self,
        workspace_id: Uuid,
        payload: BroadcastPayload,
    ) -> usize {
        let channels = self.workspace_channels.read().await;
        if let Some(sender) = channels.get(&workspace_id) {
            sender.send(payload).unwrap_or(0)
        } else {
            0
        }
    }

    /// Remove a note channel when no subscribers remain.
    /// Per /docs/spec/api/websocket.md: GC stale streams.
    pub async fn gc_note(&self, note_id: Uuid) {
        let mut channels = self.note_channels.write().await;
        if let Some(sender) = channels.get(&note_id) {
            if sender.receiver_count() == 0 {
                channels.remove(&note_id);
            }
        }
    }

    /// Remove a workspace channel when no subscribers remain.
    pub async fn gc_workspace(&self, workspace_id: Uuid) {
        let mut channels = self.workspace_channels.write().await;
        if let Some(sender) = channels.get(&workspace_id) {
            if sender.receiver_count() == 0 {
                channels.remove(&workspace_id);
            }
        }
    }

    /// Count active note streams (for diagnostics).
    pub async fn note_stream_count(&self) -> usize {
        self.note_channels.read().await.len()
    }

    /// Count active workspace streams (for diagnostics).
    pub async fn workspace_stream_count(&self) -> usize {
        self.workspace_channels.read().await.len()
    }
}

impl Default for BroadcastRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn subscribe_and_broadcast_note() {
        let reg = BroadcastRegistry::new();
        let note_id = Uuid::new_v4();
        let mut rx = reg.subscribe_note(note_id).await;
        let sent = reg
            .broadcast_note(note_id, r#"{"type":"note_event"}"#.into())
            .await;
        assert_eq!(sent, 1);
        let msg = rx.recv().await.unwrap();
        assert!(msg.contains("note_event"));
    }

    #[tokio::test]
    async fn subscribe_and_broadcast_workspace() {
        let reg = BroadcastRegistry::new();
        let ws_id = Uuid::new_v4();
        let mut rx = reg.subscribe_workspace(ws_id).await;
        let sent = reg
            .broadcast_workspace(ws_id, r#"{"type":"workspace_event"}"#.into())
            .await;
        assert_eq!(sent, 1);
        let msg = rx.recv().await.unwrap();
        assert!(msg.contains("workspace_event"));
    }

    #[tokio::test]
    async fn broadcast_no_subscribers_returns_zero() {
        let reg = BroadcastRegistry::new();
        let note_id = Uuid::new_v4();
        let sent = reg.broadcast_note(note_id, "test".into()).await;
        assert_eq!(sent, 0);
    }

    #[tokio::test]
    async fn gc_removes_empty_channel() {
        let reg = BroadcastRegistry::new();
        let note_id = Uuid::new_v4();
        let rx = reg.subscribe_note(note_id).await;
        assert_eq!(reg.note_stream_count().await, 1);
        drop(rx);
        reg.gc_note(note_id).await;
        assert_eq!(reg.note_stream_count().await, 0);
    }

    #[tokio::test]
    async fn multiple_subscribers_receive() {
        let reg = BroadcastRegistry::new();
        let note_id = Uuid::new_v4();
        let mut rx1 = reg.subscribe_note(note_id).await;
        let mut rx2 = reg.subscribe_note(note_id).await;
        let sent = reg.broadcast_note(note_id, "hello".into()).await;
        assert_eq!(sent, 2);
        assert_eq!(rx1.recv().await.unwrap(), "hello");
        assert_eq!(rx2.recv().await.unwrap(), "hello");
    }
}
