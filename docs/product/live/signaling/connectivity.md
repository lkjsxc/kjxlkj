# Live Connectivity and NAT Traversal

## Why Remote Streaming Fails Without TURN

- WebRTC media is peer-to-peer; signaling goes through the WebSocket server.
- STUN discovers public IP addresses so peers can attempt direct connection.
- STUN fails when either peer is behind symmetric NAT, carrier-grade NAT, or a firewall that blocks UDP.
- When STUN fails, browsers cannot establish a direct peer-to-peer route.
- TURN relays all media through the TURN server, providing a guaranteed path when direct connection is impossible.
- Remote viewers behind corporate or mobile firewalls almost always require TURN.

## Connection Fallback Order

1. Direct peer-to-peer attempt (host candidates).
2. STUN-based public address discovery (server-reflexive candidates).
3. TURN over UDP (relay candidates).
4. TURN over TCP (relay candidates with TCP transport).
5. TURN over TLS on port 443 (relay candidates inside TLS).

The ICE agent tries candidates in parallel, but the priority ranking prefers direct over relay and UDP over TCP.

## Minimum vs Recommended Deployment

- **Minimum** (strict firewalls): Only TCP 80 and TCP 443 open.
  - HTTPS on 443 for the web app.
  - TURN over TLS on 443 for media relay.
  - No STUN or TURN UDP required.
  - Slightly higher latency because all media traverses TLS over TCP.
- **Recommended** (best performance): Also open UDP 3478 and UDP relay range 49152-65535.
  - STUN and TURN UDP allow direct discovery and lower-latency relay.
  - TCP 443 TURN TLS remains as a fallback.

## TLS Requirement for Capture

- Browsers require HTTPS (or localhost) for `getUserMedia` and `getDisplayMedia`.
- The edge reverse proxy terminates TLS for the web app.
- TURN TLS uses end-to-end TLS passthrough so coturn owns the TURN protocol inside TLS.
