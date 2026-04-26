# Live Network Topology

## Local Compose

- `app` owns `/live` and `/live/ws`.
- `app` listens on `0.0.0.0:8080` inside Compose.
- Compose publishes `app` as `localhost:8080` on the host.
- `postgres` and `seaweedfs` stay internal to the Compose network.
- No repo-owned edge proxy or bundled TURN service is part of the canonical stack.

## External Edge Ownership

- Operators may place any reverse proxy in front of `app`.
- Operators may terminate TLS outside the repo-owned Compose stack.
- Operators may configure external STUN or TURN providers in `/admin/settings`.
- The repo does not generate certificates or manage public DNS.

## Browser Constraints

- Capture APIs work on `localhost` during local development.
- Remote deployments need HTTPS for camera and screen capture.
- Remote deployments need externally reachable signaling through `/live/ws`.
- Difficult NAT environments may need external TURN configured through `Live/ICE_servers_JSON`.

## Service Map

```text
Host localhost:8080 -> app:8080 -> /live and /live/ws
app -> postgres:5432
app -> seaweedfs:8333
```
