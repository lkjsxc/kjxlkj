use super::model::{LiveState, LiveTx};
use super::tracks::RelayTracks;
use super::{LiveHub, LiveRole};
use axum::extract::ws::Message;
use serde_json::{json, Value};
use std::sync::Arc;
use webrtc::peer_connection::RTCPeerConnection;

impl LiveHub {
    pub(super) async fn unregister_locked(
        &self,
        role: &LiveRole,
    ) -> (Vec<Arc<RTCPeerConnection>>, bool) {
        let mut state = self.inner.lock().await;
        match role {
            LiveRole::Broadcaster => unregister_broadcaster(&mut state),
            LiveRole::Viewer(id) => unregister_viewer(&mut state, id),
        }
    }

    pub(super) async fn notify_stream_ended(&self) {
        let state = self.inner.lock().await;
        for viewer in state.viewers.values() {
            send(&viewer.tx, json!({ "type": "stream_ended" }));
        }
    }

    pub(super) async fn broadcaster_parts(&self) -> Option<(LiveTx, Option<String>)> {
        self.inner
            .lock()
            .await
            .broadcaster
            .as_ref()
            .map(|broadcaster| (broadcaster.tx.clone(), broadcaster.nat_ip.clone()))
    }

    pub(super) async fn install_publisher(
        &self,
        pc: Arc<RTCPeerConnection>,
        tracks: RelayTracks,
    ) -> Vec<Arc<RTCPeerConnection>> {
        let mut state = self.inner.lock().await;
        let mut old = Vec::new();
        if let Some(broadcaster) = &mut state.broadcaster {
            push_pc(&mut old, broadcaster.pc.replace(pc));
            broadcaster.tracks = Some(tracks);
            state.broadcasting = true;
        }
        for viewer in state.viewers.values_mut() {
            push_pc(&mut old, viewer.pc.take());
            send(&viewer.tx, json!({ "type": "stream_started" }));
        }
        old
    }

    pub(super) async fn viewer_parts(
        &self,
        id: &str,
    ) -> Option<(LiveTx, RelayTracks, Option<String>)> {
        let state = self.inner.lock().await;
        let viewer = state.viewers.get(id)?;
        let tracks = state.broadcaster.as_ref()?.tracks.clone()?;
        state
            .broadcasting
            .then(|| (viewer.tx.clone(), tracks, viewer.nat_ip.clone()))
    }

    pub(super) async fn install_viewer(
        &self,
        id: &str,
        pc: Arc<RTCPeerConnection>,
    ) -> Vec<Arc<RTCPeerConnection>> {
        let mut state = self.inner.lock().await;
        let mut old = Vec::new();
        if let Some(viewer) = state.viewers.get_mut(id) {
            push_pc(&mut old, viewer.pc.replace(pc));
        }
        old
    }

    pub(super) async fn peer(&self, role: &LiveRole) -> Option<Arc<RTCPeerConnection>> {
        let state = self.inner.lock().await;
        match role {
            LiveRole::Broadcaster => state.broadcaster.as_ref()?.pc.clone(),
            LiveRole::Viewer(id) => state.viewers.get(id)?.pc.clone(),
        }
    }
}

pub(super) fn send_viewer_count(state: &LiveState) {
    if let Some(broadcaster) = &state.broadcaster {
        send(
            &broadcaster.tx,
            json!({ "type": "viewer_count", "count": state.viewers.len() }),
        );
    }
}

pub(super) async fn close_all(pcs: Vec<Arc<RTCPeerConnection>>) {
    for pc in pcs {
        let _ = pc.close().await;
    }
}

pub(super) fn send(tx: &LiveTx, message: Value) {
    let _ = tx.send(Message::Text(message.to_string().into()));
}

pub(super) fn send_error(tx: &LiveTx, message: &str) {
    send(tx, json!({ "type": "error", "message": message }));
}

fn unregister_broadcaster(state: &mut LiveState) -> (Vec<Arc<RTCPeerConnection>>, bool) {
    let mut pcs = Vec::new();
    if let Some(broadcaster) = state.broadcaster.take() {
        push_pc(&mut pcs, broadcaster.pc);
    }
    for viewer in state.viewers.values_mut() {
        push_pc(&mut pcs, viewer.pc.take());
    }
    state.broadcasting = false;
    (pcs, true)
}

fn unregister_viewer(state: &mut LiveState, id: &str) -> (Vec<Arc<RTCPeerConnection>>, bool) {
    let mut pcs = Vec::new();
    if let Some(viewer) = state.viewers.remove(id) {
        push_pc(&mut pcs, viewer.pc);
    }
    send_viewer_count(state);
    (pcs, false)
}

fn push_pc(target: &mut Vec<Arc<RTCPeerConnection>>, pc: Option<Arc<RTCPeerConnection>>) {
    if let Some(pc) = pc {
        target.push(pc);
    }
}
