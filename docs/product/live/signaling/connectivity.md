# Live Connectivity

## Connection Order

- Browsers first try host candidates on the local network.
- Browsers then try server-reflexive candidates from configured STUN.
- Browsers then try relay candidates from configured TURN over UDP.
- Browsers may try relay candidates from configured TURN over TCP or TLS.

## Remote Viewers

- Local-network viewing can work without TURN.
- Remote viewing across NAT commonly requires TURN.
- Remote production must proxy `/live/ws` with WebSocket upgrade headers.
- Remote production must serve `/live` over HTTPS for capture APIs.
- Remote production should configure operator-owned TURN in `Live/ICE_servers_JSON`.

## Failure Behavior

- Viewers show a connecting state while negotiation is pending.
- Viewers show a visible failure state when ICE reaches `failed`.
- The app closes failed peer connections.
- A viewer failure must not end the broadcaster stream for other viewers.
- A broadcaster WebSocket failure ends the stream for all viewers.
