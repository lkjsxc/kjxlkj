# Live Connectivity

## Connection Order

- Browsers connect WebRTC only to the app-hosted live relay.
- The app advertises host candidates on its configured static ICE UDP port.
- The app may advertise configured public NAT 1:1 addresses.
- Browsers may use configured STUN or TURN servers from `Live/ICE_servers_JSON`.
- TURN is a fallback for restrictive networks, not the primary target path.

## Remote Viewers

- Local-network viewing can work with the app host candidate alone.
- Remote viewing requires the app ICE UDP port to be reachable from the public edge.
- Remote viewing behind restrictive networks may require TURN.
- Remote production must proxy `/live/ws` with WebSocket upgrade headers.
- Remote production must serve `/live` over HTTPS for capture APIs.
- Remote production should configure `LIVE_ICE_PUBLIC_IPS` when Docker or NAT hides the public address.

## Failure Behavior

- Viewers show a connecting state while negotiation is pending.
- Viewers show a visible failure state when server negotiation or ICE reaches `failed`.
- The app closes failed peer connections.
- A viewer failure must not end the broadcaster stream for other viewers.
- A broadcaster WebSocket failure ends the stream for all viewers.
