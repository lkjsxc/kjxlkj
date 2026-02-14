// WebSocket session manager per /docs/spec/api/websocket.md
// Tracks active connections and their stream subscriptions.
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Unique identifier for a WebSocket connection.
pub type ConnId = Uuid;

/// Stream key: either "note:{id}" or "workspace:{id}".
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StreamKey(pub String);

impl StreamKey {
    pub fn note(id: Uuid) -> Self {
        Self(format!("note:{id}"))
    }
    pub fn workspace(id: Uuid) -> Self {
        Self(format!("workspace:{id}"))
    }
}

/// Per-connection subscription state.
#[derive(Debug, Default)]
pub struct ConnState {
    pub user_id: Uuid,
    pub subscriptions: HashSet<StreamKey>,
    /// Ack cursors: stream_key → last acked event_seq.
    pub ack_cursors: HashMap<String, i64>,
}

/// Shared state across all WS connections.
#[derive(Debug, Clone, Default)]
pub struct SessionManager {
    inner: Arc<RwLock<ManagerInner>>,
}

#[derive(Debug, Default)]
struct ManagerInner {
    /// conn_id → ConnState
    conns: HashMap<ConnId, ConnState>,
    /// stream_key → set of conn_ids subscribed
    stream_subs: HashMap<StreamKey, HashSet<ConnId>>,
    /// Idempotency cache: (note_id, idempotency_key) → (version, event_seq)
    idem_cache: HashMap<(Uuid, String), (i64, i64)>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register(&self, conn_id: ConnId, user_id: Uuid) {
        let mut inner = self.inner.write().await;
        inner.conns.insert(conn_id, ConnState {
            user_id,
            subscriptions: HashSet::new(),
            ack_cursors: HashMap::new(),
        });
    }

    pub async fn unregister(&self, conn_id: ConnId) {
        let mut inner = self.inner.write().await;
        if let Some(state) = inner.conns.remove(&conn_id) {
            for key in &state.subscriptions {
                if let Some(subs) = inner.stream_subs.get_mut(key) {
                    subs.remove(&conn_id);
                    if subs.is_empty() {
                        inner.stream_subs.remove(key);
                    }
                }
            }
        }
    }

    pub async fn subscribe(
        &self,
        conn_id: ConnId,
        key: StreamKey,
    ) -> bool {
        let mut inner = self.inner.write().await;
        if let Some(state) = inner.conns.get_mut(&conn_id) {
            state.subscriptions.insert(key.clone());
            inner.stream_subs.entry(key).or_default().insert(conn_id);
            true
        } else {
            false
        }
    }

    pub async fn unsubscribe(
        &self,
        conn_id: ConnId,
        key: &StreamKey,
    ) {
        let mut inner = self.inner.write().await;
        if let Some(state) = inner.conns.get_mut(&conn_id) {
            state.subscriptions.remove(key);
        }
        if let Some(subs) = inner.stream_subs.get_mut(key) {
            subs.remove(&conn_id);
            if subs.is_empty() {
                inner.stream_subs.remove(key);
            }
        }
    }

    /// Get all conn_ids subscribed to a stream, excluding a given conn.
    pub async fn subscribers_except(
        &self,
        key: &StreamKey,
        exclude: ConnId,
    ) -> Vec<ConnId> {
        let inner = self.inner.read().await;
        inner
            .stream_subs
            .get(key)
            .map(|subs| subs.iter().copied().filter(|c| *c != exclude).collect())
            .unwrap_or_default()
    }

    /// Get all conn_ids subscribed to a stream.
    pub async fn subscribers(&self, key: &StreamKey) -> Vec<ConnId> {
        let inner = self.inner.read().await;
        inner
            .stream_subs
            .get(key)
            .map(|subs| subs.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Update ack cursor. Returns Err if cursor regresses.
    pub async fn update_ack(
        &self,
        conn_id: ConnId,
        stream_id: &str,
        event_seq: i64,
    ) -> Result<(), i64> {
        let mut inner = self.inner.write().await;
        if let Some(state) = inner.conns.get_mut(&conn_id) {
            let current = state.ack_cursors.get(stream_id).copied().unwrap_or(0);
            if event_seq < current {
                return Err(current); // stale cursor
            }
            state.ack_cursors.insert(stream_id.to_string(), event_seq);
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Check idempotency cache for an existing commit.
    pub async fn check_idempotency(
        &self,
        note_id: Uuid,
        key: &str,
    ) -> Option<(i64, i64)> {
        let inner = self.inner.read().await;
        inner.idem_cache.get(&(note_id, key.to_string())).copied()
    }

    /// Record an idempotency entry.
    pub async fn record_idempotency(
        &self,
        note_id: Uuid,
        key: String,
        version: i64,
        event_seq: i64,
    ) {
        let mut inner = self.inner.write().await;
        inner.idem_cache.insert((note_id, key), (version, event_seq));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscribe_unsubscribe() {
        let mgr = SessionManager::new();
        let conn = Uuid::now_v7();
        let user = Uuid::now_v7();
        mgr.register(conn, user).await;

        let key = StreamKey::note(Uuid::now_v7());
        assert!(mgr.subscribe(conn, key.clone()).await);

        let subs = mgr.subscribers(&key).await;
        assert_eq!(subs.len(), 1);

        mgr.unsubscribe(conn, &key).await;
        let subs = mgr.subscribers(&key).await;
        assert!(subs.is_empty());
    }

    #[tokio::test]
    async fn test_stale_cursor() {
        let mgr = SessionManager::new();
        let conn = Uuid::now_v7();
        mgr.register(conn, Uuid::now_v7()).await;
        assert!(mgr.update_ack(conn, "note:abc", 5).await.is_ok());
        assert_eq!(mgr.update_ack(conn, "note:abc", 3).await, Err(5));
    }
}
