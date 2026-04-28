use axum::extract::ws::Message;
use std::collections::HashMap;
use std::sync::Arc;
use webrtc::peer_connection::RTCPeerConnection;

use super::tracks::RelayTracks;

pub type LiveTx = tokio::sync::mpsc::UnboundedSender<Message>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveRole {
    Broadcaster,
    Viewer(String),
}

#[derive(Default)]
pub struct LiveState {
    pub broadcaster: Option<Broadcaster>,
    pub viewers: HashMap<String, Viewer>,
    pub broadcasting: bool,
}

pub struct Broadcaster {
    pub tx: LiveTx,
    pub pc: Option<Arc<RTCPeerConnection>>,
    pub tracks: Option<RelayTracks>,
}

pub struct Viewer {
    pub tx: LiveTx,
    pub pc: Option<Arc<RTCPeerConnection>>,
}
