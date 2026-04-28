//! In-memory live WebRTC relay

mod model;
pub(crate) mod rtc;
mod state;
mod tracks;

use model::{Broadcaster, LiveState, Viewer};
pub use model::{LiveRole, LiveTx};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

#[derive(Clone)]
pub struct LiveHub {
    api: Arc<webrtc::api::API>,
    inner: Arc<Mutex<LiveState>>,
    next_id: Arc<AtomicU64>,
}

impl LiveHub {
    pub async fn new(addr: &str, public_ips: Vec<String>) -> Result<Self, String> {
        Ok(Self {
            api: Arc::new(rtc::build_api(addr, public_ips).await?),
            inner: Arc::new(Mutex::new(LiveState::default())),
            next_id: Arc::new(AtomicU64::new(1)),
        })
    }

    #[cfg(test)]
    pub async fn test() -> Self {
        Self::new("127.0.0.1:0", Vec::new()).await.unwrap()
    }

    pub async fn register_broadcaster(&self, tx: LiveTx) -> Result<LiveRole, String> {
        let mut state = self.inner.lock().await;
        if state.broadcaster.is_some() {
            return Err("A live broadcast is already active.".to_string());
        }
        state.broadcaster = Some(Broadcaster {
            tx,
            pc: None,
            tracks: None,
        });
        tracing::info!("live broadcaster registered");
        state::send_viewer_count(&state);
        Ok(LiveRole::Broadcaster)
    }

    pub async fn register_viewer(&self, tx: LiveTx) -> LiveRole {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed).to_string();
        let mut state = self.inner.lock().await;
        state.viewers.insert(
            id.clone(),
            Viewer {
                tx: tx.clone(),
                pc: None,
            },
        );
        if state.broadcasting {
            state::send(&tx, serde_json::json!({ "type": "stream_started" }));
        }
        tracing::info!(viewer_id = id, "live viewer registered");
        state::send_viewer_count(&state);
        LiveRole::Viewer(id)
    }

    pub async fn unregister(&self, role: &LiveRole) {
        let (pcs, ended) = self.unregister_locked(role).await;
        state::close_all(pcs).await;
        if ended {
            self.notify_stream_ended().await;
        }
    }

    pub async fn publish_offer(&self, sdp: RTCSessionDescription) {
        let Some(tx) = self.broadcaster_tx().await else {
            return;
        };
        let tracks = rtc::RelayTracks::from_offer(&sdp);
        tracing::info!(
            video = tracks.video.is_some(),
            audio = tracks.audio.is_some(),
            "live publisher offer parsed"
        );
        let pc = match rtc::publisher(&self.api, sdp, tx, tracks.clone()).await {
            Ok(pc) => pc,
            Err(error) => {
                tracing::warn!(%error, "live publisher offer failed");
                if let Some(tx) = self.broadcaster_tx().await {
                    state::send_error(&tx, &error);
                }
                return;
            }
        };
        let old = self.install_publisher(pc, tracks).await;
        state::close_all(old).await;
        tracing::info!("live publisher installed");
    }

    pub async fn view_offer(&self, id: &str, sdp: RTCSessionDescription) {
        let Some((tx, tracks)) = self.viewer_parts(id).await else {
            tracing::debug!(
                viewer_id = id,
                "live viewer offer ignored without active stream"
            );
            return;
        };
        let pc = match rtc::viewer(&self.api, sdp, tx.clone(), tracks).await {
            Ok(pc) => pc,
            Err(error) => {
                tracing::warn!(viewer_id = id, %error, "live viewer offer failed");
                state::send_error(&tx, &error);
                return;
            }
        };
        let old = self.install_viewer(id, pc).await;
        state::close_all(old).await;
        tracing::info!(viewer_id = id, "live viewer installed");
    }

    pub async fn add_ice(&self, role: &LiveRole, candidate: RTCIceCandidateInit) {
        if let Some(pc) = self.peer(role).await {
            rtc::add_ice(&pc, candidate).await;
        }
    }
}
