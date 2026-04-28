# Live Network Topology

## Local Compose

- `app` owns `/live` and `/live/ws`.
- `app` listens on `0.0.0.0:8080` inside Compose.
- `app` listens on the configured live ICE UDP port inside Compose.
- Compose publishes `app` to the host through the configured app port.
- Compose publishes the live ICE UDP port to the host.
- `postgres` and `seaweedfs` stay internal to the Compose network.
- Local Compose does not own public TLS.
- Local Compose does not require TURN.

## Production Edge

- Public DNS points the site hostname at the production edge address.
- Incus proxy devices for public ports belong on the `edge` container.
- Edge nginx terminates SSL for `kjxlkj.lkjsxc.com`.
- Edge nginx proxies HTTP and `/live/ws` to the Portainer-managed `kjxlkj` app stack.
- Edge nginx forwards `X-Real-IP` and `X-Forwarded-For` to the app stack.
- Edge forwards the configured live ICE UDP port to the `kjxlkj` app stack.
- `LIVE_ICE_PUBLIC_IPS` advertises the public edge address for external clients.
- `LIVE_ICE_LAN_IPS` advertises the LAN edge address for LAN clients.
- `LIVE_TRUSTED_PROXY_IPS` contains only trusted edge proxy source addresses.
- The Portainer stack continues to consume GitHub CI-built GHCR images.

## Flow

```text
viewer -> edge nginx HTTPS 443 -> kjxlkj app HTTP 8080 -> /live and /live/ws
broadcaster -> edge live ICE UDP port -> kjxlkj app in-process relay
viewer -> edge live ICE UDP port -> kjxlkj app in-process relay
app -> postgres:5432
app -> seaweedfs:8333
```
