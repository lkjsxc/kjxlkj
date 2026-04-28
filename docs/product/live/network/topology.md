# Live Network Topology

## Local Compose

- `app` owns `/live` and `/live/ws`.
- `app` listens on `0.0.0.0:8080` inside Compose.
- Compose publishes `app` to the host through the configured app port.
- `postgres` and `seaweedfs` stay internal to the Compose network.
- Local Compose does not own public TLS.
- Local Compose does not own public TURN.

## Production Edge

- Public DNS points `kjxlkj.lkjsxc.com` and `turn.lkjsxc.com` at `92.202.56.95`.
- Incus proxy devices for public ports belong on the `edge` container.
- Edge nginx terminates SSL for `kjxlkj.lkjsxc.com`.
- Edge nginx proxies HTTP and `/live/ws` to the Portainer-managed `kjxlkj` app stack.
- Edge coturn serves `turn.lkjsxc.com`.
- The Portainer stack continues to consume GitHub CI-built GHCR images.

## Flow

```text
viewer -> edge nginx HTTPS 443 -> kjxlkj app HTTP 8080 -> /live and /live/ws
viewer -> edge coturn 3478/tcp+udp and relay UDP range -> WebRTC relay
app -> postgres:5432
app -> seaweedfs:8333
```
