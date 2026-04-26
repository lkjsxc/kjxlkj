# Live Firewall Rules

## Local Compose

- TCP `8080` on the host reaches the app.
- PostgreSQL and SeaweedFS are not published on the host.
- No repo-owned STUN, TURN, TLS, or relay ports are published.

## External Deployments

- Operators choose their own public HTTP or HTTPS ports.
- Operators choose their own STUN or TURN provider and firewall policy.
- External TURN should be configured in `Live/ICE_servers_JSON`.
- External reverse proxies must forward WebSocket upgrades for `/live/ws`.

## Verification

- Browser verification targets `http://app:8080` inside Compose.
- Manual local verification targets `http://localhost:8080`.
- Firewall checks outside port `8080` are outside the repo-owned verification scope.
