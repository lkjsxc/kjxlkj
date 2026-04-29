# Live Connectivity

## Connection Order

- Browsers connect WebRTC only to the app-hosted live relay.
- The app advertises host candidates on its configured static ICE UDP port.
- The app selects one advertised NAT 1:1 address per peer connection.
- Public clients receive public relay candidates.
- LAN clients receive LAN relay candidates when configured.
- Browser clients do not receive STUN or TURN servers for the core path.

## Remote Viewers

- Local-network viewing can work with the app host candidate alone.
- Remote viewing requires the app ICE UDP port to be reachable from the public edge.
- Remote production must proxy `/live/ws` with WebSocket upgrade headers.
- Remote production must forward client IP headers from trusted edge proxies.
- Remote production must serve `/live` over HTTPS for capture APIs.
- Remote production should configure public and LAN relay addresses separately.

## Failure Behavior

- Viewers show a connecting state while negotiation is pending.
- Viewers show a visible failure state when server negotiation or ICE reaches `failed`.
- The app closes failed peer connections.
- A viewer failure must not end the broadcaster stream for other viewers.
- A broadcaster WebSocket failure ends the stream for all viewers.

## Media Diagnostics

- ICE `Connected` means the peer path is viable, not that media is flowing.
- The app must install a process-level Rustls crypto provider before WebRTC receives RTP.
- Server logs must show `live publisher track received` after the publish answer is applied.
- Server logs must show `live publisher first RTP packet received` before any viewer can decode video.
- Browser viewer logs must show inbound RTP stats with bytes or decoded frames.
- Multiple publisher media sections require RID-tagged RTP so the relay can map tracks.
- If viewer `ontrack` fires but inbound RTP stats stay zero, inspect publisher track reception first.
- If publisher RTP arrives but viewer stats stay zero, inspect relay track writes and viewer answer SDP.
