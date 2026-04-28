# Live Signaling

Contracts for WebSocket signaling, ICE servers, and live connectivity failure behavior.

## Child Index

- [messages.md](messages.md): WebSocket message shapes, forwarding, and lifetime rules
- [ice-servers.md](ice-servers.md): `Live/ICE_servers_JSON` format and validation
- [connectivity.md](connectivity.md): NAT traversal, TURN relay, and visible failure states

## Rules

- `GET /live/ws` is the WebSocket signaling endpoint.
- WebRTC carries media directly or through TURN relay.
- The server relays signaling messages only.
