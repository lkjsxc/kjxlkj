use std::sync::Arc;
use webrtc::api::media_engine::{MIME_TYPE_OPUS, MIME_TYPE_VP8};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTPCodecType};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocalWriter;

#[derive(Clone)]
pub struct RelayTracks {
    pub video: Option<Arc<TrackLocalStaticRTP>>,
    pub audio: Option<Arc<TrackLocalStaticRTP>>,
}

impl RelayTracks {
    pub fn from_offer(offer: &RTCSessionDescription) -> Self {
        Self {
            video: has_media(&offer.sdp, "video").then(video_track),
            audio: has_media(&offer.sdp, "audio").then(audio_track),
        }
    }

    fn for_kind(&self, kind: RTPCodecType) -> Option<Arc<TrackLocalStaticRTP>> {
        match kind {
            RTPCodecType::Video => self.video.clone(),
            RTPCodecType::Audio => self.audio.clone(),
            _ => None,
        }
    }
}

pub fn attach_track_reader(pc: &Arc<RTCPeerConnection>, tracks: RelayTracks) {
    let pc_weak = Arc::downgrade(pc);
    pc.on_track(Box::new(move |track, _, _| {
        let tracks = tracks.clone();
        let pc_weak = pc_weak.clone();
        tokio::spawn(async move {
            let Some(output) = tracks.for_kind(track.kind()) else {
                return;
            };
            let media_ssrc = track.ssrc();
            tokio::spawn(async move {
                while let Some(pc) = pc_weak.upgrade() {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    let _ = pc
                        .write_rtcp(&[Box::new(PictureLossIndication {
                            sender_ssrc: 0,
                            media_ssrc,
                        })])
                        .await;
                }
            });
            while let Ok((rtp, _)) = track.read_rtp().await {
                if output.write_rtp(&rtp).await.is_err() {
                    break;
                }
            }
        });
        Box::pin(async {})
    }));
}

fn video_track() -> Arc<TrackLocalStaticRTP> {
    Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_VP8.to_string(),
            clock_rate: 90000,
            channels: 0,
            sdp_fmtp_line: String::new(),
            rtcp_feedback: Vec::new(),
        },
        "video".to_string(),
        "live".to_string(),
    ))
}

fn audio_track() -> Arc<TrackLocalStaticRTP> {
    Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_OPUS.to_string(),
            clock_rate: 48000,
            channels: 2,
            sdp_fmtp_line: String::new(),
            rtcp_feedback: Vec::new(),
        },
        "audio".to_string(),
        "live".to_string(),
    ))
}

fn has_media(sdp: &str, kind: &str) -> bool {
    sdp.lines()
        .any(|line| line.starts_with(&format!("m={kind} ")))
}
