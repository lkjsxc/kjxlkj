use super::ice_config::IceAddresses;
use std::net::IpAddr;
use tokio::net::UdpSocket;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::setting_engine::SettingEngine;
use webrtc::api::APIBuilder;
use webrtc::ice::udp_mux::{UDPMuxDefault, UDPMuxParams};
use webrtc::ice::udp_network::UDPNetwork;
use webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType;
use webrtc::interceptor::registry::Registry;

#[derive(Clone)]
pub struct LiveRtc {
    udp_network: UDPNetwork,
    addresses: IceAddresses,
}

impl LiveRtc {
    pub fn client_nat_ip(&self, client_ip: Option<IpAddr>) -> Option<String> {
        self.addresses.candidate_for(client_ip)
    }

    pub fn api(&self, nat_ip: Option<&str>) -> Result<webrtc::api::API, String> {
        build_api(self.udp_network.clone(), nat_ip)
    }
}

pub async fn build_rtc(
    addr: &str,
    public_ips: Vec<String>,
    lan_ips: Vec<String>,
) -> Result<LiveRtc, String> {
    let socket = UdpSocket::bind(addr)
        .await
        .map_err(|error| format!("failed to bind live ICE UDP socket: {error}"))?;
    Ok(LiveRtc {
        udp_network: UDPNetwork::Muxed(UDPMuxDefault::new(UDPMuxParams::new(socket))),
        addresses: IceAddresses::new(public_ips, lan_ips),
    })
}

fn build_api(udp_network: UDPNetwork, nat_ip: Option<&str>) -> Result<webrtc::api::API, String> {
    let mut settings = SettingEngine::default();
    settings.set_udp_network(udp_network);
    if let Some(nat_ip) = nat_ip {
        settings.set_nat_1to1_ips(vec![nat_ip.to_string()], RTCIceCandidateType::Host);
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
