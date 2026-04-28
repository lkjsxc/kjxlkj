# Live Signaling

Contracts for `/live/ws`, browser-to-app WebRTC negotiation, app relay ICE, and live connectivity failure behavior.

## Child Index

- [messages.md](messages.md): WebSocket message shapes, forwarding, and lifetime rules
- [ice-servers.md](ice-servers.md): app relay ICE environment and address rules
- [connectivity.md](connectivity.md): NAT traversal and visible failure states

## Rules

- `GET /live/ws` is the WebSocket signaling endpoint.
- WebRTC media terminates at the `app` process.
- The `app` process fans broadcaster RTP out to connected viewers.
- Browsers never negotiate WebRTC directly with each other.
