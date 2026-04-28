# Live Relay ICE

## Source Of Truth

- The live relay is app-hosted.
- Browser clients connect only to the app relay, never to each other.
- Deployment environment owns the app relay ICE socket.
- `LIVE_ICE_BIND_IP` controls the UDP bind address inside the app runtime.
- `LIVE_ICE_UDP_PORT` controls the app relay UDP port.
- `LIVE_ICE_PUBLIC_IPS` controls public NAT 1:1 addresses advertised by the app.
- `LIVE_ICE_LAN_IPS` controls LAN NAT 1:1 addresses advertised to LAN clients.
- `LIVE_TRUSTED_PROXY_IPS` controls which proxy peers may supply forwarded client IP headers.
- Each address list may contain one IPv4 and one IPv6 candidate.
- A candidate may be a bare IP address or an `external/local` NAT mapping.
- Extra same-family entries are ignored before WebRTC setup.
- Admin settings do not provide browser ICE servers for the core live path.

## Browser Behavior

- The live page embeds only capture defaults in `#live-config`.
- Browser `RTCPeerConnection` instances use an empty `iceServers` list.
- Browser ICE candidates are scoped to their own app WebSocket session.
- Browser SDP and ICE are never forwarded to another browser.
- Viewer media tracks originate from app-owned relay tracks.

## Production Addresses

- Public clients receive a public relay candidate matching their address family.
- LAN clients receive a LAN relay candidate matching their address family when configured.
- Forwarded client IP headers are ignored unless the direct peer is trusted.
- Address lists are comma-separated.
- Docker and Incus must forward `${LIVE_ICE_UDP_PORT}/udp` to the app.

## Production Example Value

```env
LIVE_ICE_BIND_IP=0.0.0.0
LIVE_ICE_UDP_PORT=8189
LIVE_ICE_PUBLIC_IPS=<public-edge-ip>
LIVE_ICE_LAN_IPS=<lan-edge-ip>
LIVE_TRUSTED_PROXY_IPS=<edge-nginx-container-ip>
```
