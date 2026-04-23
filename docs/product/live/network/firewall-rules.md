# Live Firewall Rules

## Minimum Ports (Strict Firewalls)

Open these ports when only TCP is allowed:

- TCP 80 — HTTP redirect to HTTPS.
- TCP 443 — HTTPS web app and TURN over TLS.

With only these ports:
- Web app works over HTTPS.
- TURN relays media over TLS inside TCP 443.
- STUN and TURN UDP are unavailable.
- Direct peer-to-peer may still work if both peers can reach each other via TCP, but this is rare.

## Recommended Ports (Best Performance)

Open these ports for lower latency and higher throughput:

- TCP 80 — HTTP redirect.
- TCP 443 — HTTPS web app and TURN TLS fallback.
- UDP 3478 — STUN and TURN UDP.
- UDP 49152-65535 — TURN UDP relay ports.

With these ports:
- STUN discovers public addresses for direct peer-to-peer.
- TURN UDP relays media with lower latency than TCP.
- TURN TLS on TCP 443 remains as a fallback for strict networks.

## Port Summary Table

| Port | Protocol | Service | Purpose |
|------|----------|---------|---------|
| 80 | TCP | nginx | HTTP redirect |
| 443 | TCP | nginx | HTTPS + TURN TLS passthrough |
| 3478 | UDP | coturn | STUN + TURN UDP |
| 3478 | TCP | coturn | TURN TCP |
| 5349 | TCP | coturn | TURN TLS (internal only, via nginx) |
| 49152-65535 | UDP | coturn | TURN UDP relay range |
