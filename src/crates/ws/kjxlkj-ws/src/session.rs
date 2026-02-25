//! WebSocket session management

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use kjxlkj_domain::{DomainEvent, NoteId};

/// Client cursor state
#[derive(Debug, Clone)]
pub struct ClientCursor {
    pub acknowledged_event_seq: u64,
    pub pending_event_seq: u64,
    pub base_version: u64,
}

impl Default for ClientCursor {
    fn default() -> Self {
        Self {
            acknowledged_event_seq: 0,
            pending_event_seq: 0,
            base_version: 1,
        }
    }
}

/// WebSocket session
#[derive(Debug, Clone)]
pub struct WsSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub cursor: ClientCursor,
}

impl WsSession {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            user_id,
            cursor: ClientCursor::default(),
        }
    }
}

/// Broadcast registry for note subscriptions
#[derive(Debug, Clone, Default)]
pub struct BroadcastRegistry {
    /// Note ID -> broadcast sender
    subscriptions: Arc<RwLock<HashMap<NoteId, broadcast::Sender<DomainEvent>>>>,
}

impl BroadcastRegistry {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, note_id: NoteId) -> broadcast::Receiver<DomainEvent> {
        let mut subscriptions = self.subscriptions.write().await;
        
        let sender = subscriptions
            .entry(note_id)
            .or_insert_with(|| broadcast::channel(100).0);
        
        sender.subscribe()
    }

    pub async fn broadcast(&self, note_id: NoteId, event: DomainEvent) {
        let subscriptions = self.subscriptions.read().await;
        
        if let Some(sender) = subscriptions.get(&note_id) {
            let _ = sender.send(event);
        }
    }

    pub async fn unsubscribe(&self, note_id: NoteId) {
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.remove(&note_id);
    }
}

/// Session registry
#[derive(Debug, Clone, Default)]
pub struct SessionRegistry {
    sessions: Arc<RwLock<HashMap<Uuid, WsSession>>>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&self, session: WsSession) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.session_id, session);
    }

    pub async fn get(&self, session_id: Uuid) -> Option<WsSession> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    pub async fn remove(&self, session_id: Uuid) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);
    }

    pub async fn update_cursor(&self, session_id: Uuid, event_seq: u64) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.cursor.acknowledged_event_seq = event_seq;
        }
    }
}
