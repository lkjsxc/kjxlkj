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
