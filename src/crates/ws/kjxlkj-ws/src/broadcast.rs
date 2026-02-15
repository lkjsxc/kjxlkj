//! WebSocket broadcast registry per /docs/spec/api/websocket.md.
//! Maintains session registry for cross-actor event broadcasting.
//! Thread-safe via Arc<RwLock<...>>.

use std::collections::HashMap;
use std::sync::{mpsc, Arc, RwLock};
use uuid::Uuid;

/// Handle for sending text messages to a connected session.
pub type SessionSender = mpsc::Sender<String>;

/// Shared registry of active WebSocket sessions.
#[derive(Clone, Default)]
pub struct BroadcastRegistry {
    inner: Arc<RwLock<RegistryInner>>,
}

#[derive(Default)]
struct RegistryInner {
    sessions: HashMap<Uuid, SessionEntry>,
}

struct SessionEntry {
    user_id: Uuid,
    workspace_ids: Vec<Uuid>,
    sender: SessionSender,
}

impl BroadcastRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new session. Returns a receiver for the actor.
    pub fn register(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel();
        let entry = SessionEntry {
            user_id,
            workspace_ids: Vec::new(),
            sender: tx,
        };
        let mut inner = self.inner.write().unwrap();
        inner.sessions.insert(session_id, entry);
        rx
    }

    /// Remove a session from the registry.
    pub fn unregister(&self, session_id: &Uuid) {
        let mut inner = self.inner.write().unwrap();
        inner.sessions.remove(session_id);
    }

    /// Subscribe a session to workspace events.
    pub fn subscribe_workspace(
        &self,
        session_id: &Uuid,
        workspace_id: Uuid,
    ) {
        let mut inner = self.inner.write().unwrap();
        if let Some(entry) = inner.sessions.get_mut(session_id) {
            if !entry.workspace_ids.contains(&workspace_id) {
                entry.workspace_ids.push(workspace_id);
            }
        }
    }

    /// Broadcast to all sessions subscribed to a workspace.
    pub fn broadcast_to_workspace(
        &self,
        workspace_id: &Uuid,
        message: &str,
    ) -> usize {
        let inner = self.inner.read().unwrap();
        let mut count = 0;
        for entry in inner.sessions.values() {
            if entry.workspace_ids.contains(workspace_id) {
                if entry.sender.send(message.to_owned()).is_ok() {
                    count += 1;
                }
            }
        }
        count
    }

    /// Broadcast session revocation to a specific user's sessions.
    /// Per /docs/spec/security/sessions.md revocation broadcast.
    pub fn revoke_user_sessions(
        &self,
        user_id: &Uuid,
        revoke_message: &str,
    ) -> usize {
        let inner = self.inner.read().unwrap();
        let mut count = 0;
        for entry in inner.sessions.values() {
            if &entry.user_id == user_id {
                let _ = entry.sender.send(revoke_message.to_owned());
                count += 1;
            }
        }
        count
    }

    /// Return count of active sessions.
    pub fn session_count(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_broadcast() {
        let reg = BroadcastRegistry::new();
        let sid = Uuid::now_v7();
        let uid = Uuid::now_v7();
        let wid = Uuid::now_v7();

        let rx = reg.register(sid, uid);
        reg.subscribe_workspace(&sid, wid);
        let sent = reg.broadcast_to_workspace(&wid, "hello");
        assert_eq!(sent, 1);
        assert_eq!(rx.try_recv().unwrap(), "hello");
    }

    #[test]
    fn unregister_removes_session() {
        let reg = BroadcastRegistry::new();
        let sid = Uuid::now_v7();
        let uid = Uuid::now_v7();
        let _rx = reg.register(sid, uid);
        assert_eq!(reg.session_count(), 1);
        reg.unregister(&sid);
        assert_eq!(reg.session_count(), 0);
    }

    #[test]
    fn revoke_targets_user() {
        let reg = BroadcastRegistry::new();
        let uid1 = Uuid::now_v7();
        let uid2 = Uuid::now_v7();
        let rx1 = reg.register(Uuid::now_v7(), uid1);
        let rx2 = reg.register(Uuid::now_v7(), uid2);

        let count = reg.revoke_user_sessions(&uid1, "revoked");
        assert_eq!(count, 1);
        assert_eq!(rx1.try_recv().unwrap(), "revoked");
        assert!(rx2.try_recv().is_err());
    }
}
