use axum::extract::ws::Message;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocal;

use super::tracks::attach_track_reader;
pub use super::tracks::RelayTracks;

pub async fn publisher(
    api: &webrtc::api::API,
    offer: RTCSessionDescription,
    tx: mpsc::UnboundedSender<Message>,
    tracks: RelayTracks,
) -> Result<Arc<RTCPeerConnection>, String> {
    let pc = Arc::new(new_peer(api).await?);
    attach_state_logs(&pc, "broadcaster");
    attach_ice_sender(&pc, tx.clone());
    attach_track_reader(&pc, tracks);
    answer(&pc, offer, tx).await?;
    Ok(pc)
}

pub async fn viewer(
    api: &webrtc::api::API,
    offer: RTCSessionDescription,
    tx: mpsc::UnboundedSender<Message>,
    tracks: RelayTracks,
) -> Result<Arc<RTCPeerConnection>, String> {
    let pc = Arc::new(new_peer(api).await?);
    attach_state_logs(&pc, "viewer");
    if let Some(track) = &tracks.video {
        add_track(&pc, track).await?;
    }
    if let Some(track) = &tracks.audio {
        add_track(&pc, track).await?;
    }
    attach_ice_sender(&pc, tx.clone());
    answer(&pc, offer, tx).await?;
    Ok(pc)
}

pub async fn add_ice(pc: &RTCPeerConnection, candidate: RTCIceCandidateInit) {
    if let Err(error) = pc.add_ice_candidate(candidate).await {
        tracing::warn!(%error, "live add ICE candidate failed");
    }
}

async fn new_peer(api: &webrtc::api::API) -> Result<RTCPeerConnection, String> {
    api.new_peer_connection(RTCConfiguration::default())
        .await
        .map_err(|error| error.to_string())
}

async fn add_track(
    pc: &Arc<RTCPeerConnection>,
    track: &Arc<TrackLocalStaticRTP>,
) -> Result<(), String> {
    let sender = pc
        .add_track(Arc::clone(track) as Arc<dyn TrackLocal + Send + Sync>)
        .await
        .map_err(|error| error.to_string())?;
    tokio::spawn(async move {
        let mut buf = vec![0u8; 1500];
        while sender.read(&mut buf).await.is_ok() {}
    });
    Ok(())
}

async fn answer(
    pc: &RTCPeerConnection,
    offer: RTCSessionDescription,
    tx: mpsc::UnboundedSender<Message>,
) -> Result<(), String> {
    pc.set_remote_description(offer)
        .await
        .map_err(|error| error.to_string())?;
    let answer = pc
        .create_answer(None)
        .await
        .map_err(|error| error.to_string())?;
    pc.set_local_description(answer)
        .await
        .map_err(|error| error.to_string())?;
    if let Some(sdp) = pc.local_description().await {
        tracing::info!("live answer created");
        send(&tx, json!({ "type": "answer", "sdp": sdp }));
    }
    Ok(())
}

fn attach_ice_sender(pc: &RTCPeerConnection, tx: mpsc::UnboundedSender<Message>) {
    pc.on_ice_candidate(Box::new(move |candidate| {
        let tx = tx.clone();
        Box::pin(async move {
            if let Some(candidate) = candidate.and_then(|value| value.to_json().ok()) {
                tracing::debug!("live ICE candidate sent");
                send(&tx, json!({ "type": "ice", "candidate": candidate }));
            }
        })
    }));
}

fn attach_state_logs(pc: &RTCPeerConnection, role: &'static str) {
    pc.on_ice_connection_state_change(Box::new(move |state| {
        tracing::info!(role, state = ?state, "live ICE connection state changed");
        Box::pin(async {})
    }));
    pc.on_peer_connection_state_change(Box::new(move |state| {
        tracing::info!(role, state = ?state, "live peer connection state changed");
        Box::pin(async {})
    }));
}

fn send(tx: &mpsc::UnboundedSender<Message>, message: Value) {
    let _ = tx.send(Message::Text(message.to_string().into()));
}
