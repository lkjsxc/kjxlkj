# Live Relay ICE

## Source Of Truth

- The live relay is app-hosted.
- Browser clients connect only to the app relay, never to each other.
- Deployment environment owns the app relay ICE socket.
- `LIVE_ICE_BIND_IP` controls the UDP bind address inside the app runtime.
- `LIVE_ICE_UDP_PORT` controls the app relay UDP port.
- `LIVE_ICE_PUBLIC_IPS` controls NAT 1:1 addresses advertised by the app.
- Admin settings do not provide browser ICE servers for the core live path.

## Browser Behavior

- The live page embeds only capture defaults in `#live-config`.
- Browser `RTCPeerConnection` instances use an empty `iceServers` list.
- Browser ICE candidates are scoped to their own app WebSocket session.
- Browser SDP and ICE are never forwarded to another browser.
- Viewer media tracks originate from app-owned relay tracks.

## Production Addresses

- Public viewers need a public app relay address in `LIVE_ICE_PUBLIC_IPS`.
- LAN viewers need a LAN app relay address in `LIVE_ICE_PUBLIC_IPS`.
- Multiple addresses are comma-separated.
- Docker and Incus must forward `${LIVE_ICE_UDP_PORT}/udp` to the app.

## Production Example Value

```env
LIVE_ICE_BIND_IP=0.0.0.0
LIVE_ICE_UDP_PORT=8189
LIVE_ICE_PUBLIC_IPS=92.202.56.95,192.168.1.2
```
