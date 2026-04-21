//! In-memory live broadcast signaling hub

use axum::extract::ws::Message;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::{mpsc, Mutex};

pub type LiveTx = mpsc::UnboundedSender<Message>;

#[derive(Clone, Default)]
pub struct LiveHub {
    inner: Arc<Mutex<LiveState>>,
    next_id: Arc<AtomicU64>,
}

#[derive(Default)]
struct LiveState {
    broadcaster: Option<Connection>,
    viewers: HashMap<String, LiveTx>,
}

struct Connection {
    tx: LiveTx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveRole {
    Broadcaster,
    Viewer(String),
}

impl LiveHub {
    pub async fn register_broadcaster(&self, tx: LiveTx) -> Result<LiveRole, String> {
        let mut state = self.inner.lock().await;
        if state.broadcaster.is_some() {
            return Err("A live broadcast is already active.".to_string());
        }
        state.broadcaster = Some(Connection { tx: tx.clone() });
        notify_all_viewers(&state.viewers, json!({ "type": "stream_started" }));
        for viewer_id in state.viewers.keys() {
            send(
                &tx,
                json!({ "type": "viewer_ready", "viewer_id": viewer_id }),
            );
        }
        Ok(LiveRole::Broadcaster)
    }

    pub async fn register_viewer(&self, tx: LiveTx) -> LiveRole {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed).to_string();
        let mut state = self.inner.lock().await;
        state.viewers.insert(id.clone(), tx.clone());
        if state.broadcaster.is_some() {
            send(&tx, json!({ "type": "stream_started" }));
            self.send_to_broadcaster_locked(
                &state,
                json!({ "type": "viewer_ready", "viewer_id": id }),
            );
        }
        LiveRole::Viewer(id)
    }

    pub async fn unregister(&self, role: &LiveRole) {
        let mut state = self.inner.lock().await;
        match role {
            LiveRole::Broadcaster => {
                state.broadcaster = None;
                notify_all_viewers(&state.viewers, json!({ "type": "stream_ended" }));
            }
            LiveRole::Viewer(id) => {
                state.viewers.remove(id);
            }
        }
    }

    pub async fn forward_from_viewer(&self, viewer_id: &str, mut message: Value) {
        set_viewer_id(&mut message, viewer_id);
        let state = self.inner.lock().await;
        self.send_to_broadcaster_locked(&state, message);
    }

    pub async fn forward_from_broadcaster(&self, message: Value) {
        let Some(viewer_id) = message.get("viewer_id").and_then(Value::as_str) else {
            return;
        };
        let state = self.inner.lock().await;
        if let Some(tx) = state.viewers.get(viewer_id) {
            send(tx, message);
        }
    }

    fn send_to_broadcaster_locked(&self, state: &LiveState, message: Value) {
        if let Some(broadcaster) = &state.broadcaster {
            send(&broadcaster.tx, message);
        }
    }
}

fn notify_all_viewers(viewers: &HashMap<String, LiveTx>, message: Value) {
    for tx in viewers.values() {
        send(tx, message.clone());
    }
}

fn send(tx: &LiveTx, message: Value) {
    let _ = tx.send(Message::Text(message.to_string().into()));
}

fn set_viewer_id(message: &mut Value, viewer_id: &str) {
    if let Some(object) = message.as_object_mut() {
        object.insert(
            "viewer_id".to_string(),
            Value::String(viewer_id.to_string()),
        );
    }
}
