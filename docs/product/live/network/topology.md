# Live Network Topology

## Local Compose

- `app` owns `/live` and `/live/ws`.
- `app` listens on `0.0.0.0:8080` inside Compose.
- `app` listens on the configured live ICE UDP port inside Compose.
- Compose publishes `app` to the host through the configured app port.
- Compose publishes the live ICE UDP port to the host.
- `postgres` and `seaweedfs` stay internal to the Compose network.
- Local Compose does not own public TLS.
- Local Compose does not require public TURN.

## Production Edge

- Public DNS points `kjxlkj.lkjsxc.com` and `turn.lkjsxc.com` at `92.202.56.95`.
- Incus proxy devices for public ports belong on the `edge` container.
- Edge nginx terminates SSL for `kjxlkj.lkjsxc.com`.
- Edge nginx proxies HTTP and `/live/ws` to the Portainer-managed `kjxlkj` app stack.
- Edge forwards the configured live ICE UDP port to the `kjxlkj` app stack.
- Edge coturn may serve `turn.lkjsxc.com` for restrictive viewer networks.
- The Portainer stack continues to consume GitHub CI-built GHCR images.

## Flow

```text
viewer -> edge nginx HTTPS 443 -> kjxlkj app HTTP 8080 -> /live and /live/ws
broadcaster -> edge live ICE UDP port -> kjxlkj app in-process relay
viewer -> edge live ICE UDP port -> kjxlkj app in-process relay
viewer -> optional edge coturn 3478/tcp+udp and relay UDP range -> app relay
app -> postgres:5432
app -> seaweedfs:8333
```
