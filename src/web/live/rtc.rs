use axum::extract::ws::Message;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::setting_engine::SettingEngine;
use webrtc::api::APIBuilder;
use webrtc::ice::udp_mux::{UDPMuxDefault, UDPMuxParams};
use webrtc::ice::udp_network::UDPNetwork;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTPCodecType;
use webrtc::rtp_transceiver::rtp_transceiver_direction::RTCRtpTransceiverDirection;
use webrtc::rtp_transceiver::RTCRtpTransceiverInit;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocal;

use super::tracks::attach_track_reader;
pub use super::tracks::RelayTracks;

pub async fn build_api(addr: &str, public_ips: Vec<String>) -> Result<webrtc::api::API, String> {
    let socket = UdpSocket::bind(addr)
        .await
        .map_err(|error| format!("failed to bind live ICE UDP socket: {error}"))?;
    let mux = UDPMuxDefault::new(UDPMuxParams::new(socket));
    let mut settings = SettingEngine::default();
    settings.set_udp_network(UDPNetwork::Muxed(mux));
    if !public_ips.is_empty() {
        settings.set_nat_1to1_ips(public_ips, RTCIceCandidateType::Host);
    }
    let mut media = MediaEngine::default();
    media.register_default_codecs().map_err(|e| e.to_string())?;
    let registry =
        register_default_interceptors(Registry::new(), &mut media).map_err(|e| e.to_string())?;
    Ok(APIBuilder::new()
        .with_setting_engine(settings)
        .with_media_engine(media)
        .with_interceptor_registry(registry)
        .build())
}

pub fn ice_servers(value: &Value) -> Result<Vec<RTCIceServer>, String> {
    match value.as_array() {
        Some(servers) => servers.iter().map(ice_server).collect(),
        None => Ok(Vec::new()),
    }
}

pub async fn publisher(
    api: &webrtc::api::API,
    ice_servers: Vec<RTCIceServer>,
    offer: RTCSessionDescription,
    tx: mpsc::UnboundedSender<Message>,
    tracks: RelayTracks,
) -> Result<Arc<RTCPeerConnection>, String> {
    let pc = Arc::new(new_peer(api, ice_servers).await?);
    add_recvonly(&pc, RTPCodecType::Video).await?;
    if tracks.audio.is_some() {
        add_recvonly(&pc, RTPCodecType::Audio).await?;
    }
    attach_ice_sender(&pc, tx.clone());
    attach_track_reader(&pc, tracks);
    answer(&pc, offer, tx).await?;
    Ok(pc)
}

pub async fn viewer(
    api: &webrtc::api::API,
    ice_servers: Vec<RTCIceServer>,
    offer: RTCSessionDescription,
    tx: mpsc::UnboundedSender<Message>,
    tracks: RelayTracks,
) -> Result<Arc<RTCPeerConnection>, String> {
    let pc = Arc::new(new_peer(api, ice_servers).await?);
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
    let _ = pc.add_ice_candidate(candidate).await;
}

fn ice_server(value: &Value) -> Result<RTCIceServer, String> {
    Ok(RTCIceServer {
        urls: urls(value.get("urls")).ok_or_else(|| "invalid ICE server urls".to_string())?,
        username: value
            .get("username")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        credential: value
            .get("credential")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
    })
}

fn urls(value: Option<&Value>) -> Option<Vec<String>> {
    match value {
        Some(Value::String(url)) => Some(vec![url.to_string()]),
        Some(Value::Array(urls)) => urls
            .iter()
            .map(|url| url.as_str().map(str::to_string))
            .collect(),
        _ => None,
    }
}

async fn new_peer(
    api: &webrtc::api::API,
    ice_servers: Vec<RTCIceServer>,
) -> Result<RTCPeerConnection, String> {
    api.new_peer_connection(RTCConfiguration {
        ice_servers,
        ..Default::default()
    })
    .await
    .map_err(|error| error.to_string())
}

async fn add_recvonly(pc: &RTCPeerConnection, kind: RTPCodecType) -> Result<(), String> {
    pc.add_transceiver_from_kind(
        kind,
        Some(RTCRtpTransceiverInit {
            direction: RTCRtpTransceiverDirection::Recvonly,
            send_encodings: Vec::new(),
        }),
    )
    .await
    .map(|_| ())
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
        send(&tx, json!({ "type": "answer", "sdp": sdp }));
    }
    Ok(())
}

fn attach_ice_sender(pc: &RTCPeerConnection, tx: mpsc::UnboundedSender<Message>) {
    pc.on_ice_candidate(Box::new(move |candidate| {
        let tx = tx.clone();
        Box::pin(async move {
            if let Some(candidate) = candidate.and_then(|value| value.to_json().ok()) {
                send(&tx, json!({ "type": "ice", "candidate": candidate }));
            }
        })
    }));
}

fn send(tx: &mpsc::UnboundedSender<Message>, message: Value) {
    let _ = tx.send(Message::Text(message.to_string().into()));
}
