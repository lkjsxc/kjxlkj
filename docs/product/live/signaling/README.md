# Live Signaling

Contracts for `/live/ws`, browser-to-app WebRTC negotiation, ICE servers, and live connectivity failure behavior.

## Child Index

- [messages.md](messages.md): WebSocket message shapes, forwarding, and lifetime rules
- [ice-servers.md](ice-servers.md): `Live/ICE_servers_JSON` format and validation
- [connectivity.md](connectivity.md): NAT traversal, TURN relay, and visible failure states

## Rules

- `GET /live/ws` is the WebSocket signaling endpoint.
- WebRTC media terminates at the `app` process.
- The `app` process fans broadcaster RTP out to connected viewers.
- Browsers never negotiate WebRTC directly with each other.
