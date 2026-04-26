# Live Connectivity

## Local Behavior

- Local Compose does not configure ICE servers by default.
- Browser peers may connect directly on permissive local networks.
- `/live/ws` carries signaling messages through the app.
- Capture APIs work on `localhost` without repo-owned TLS.

## Remote Behavior

- Remote deployments usually need HTTPS for capture APIs.
- Remote viewers behind strict NAT or firewalls may need TURN.
- External STUN or TURN providers belong in `Live/ICE_servers_JSON`.
- The repo-owned Compose stack does not bundle relay infrastructure.

## Connection Fallback Order

1. Browser local host candidates.
2. Browser server-reflexive candidates from configured STUN.
3. Relay candidates from configured TURN over UDP.
4. Relay candidates from configured TURN over TCP or TLS.

## Operator Rule

- Configure only the ICE servers that the operator owns or trusts.
- Leave `Live/ICE_servers_JSON` empty when direct connectivity is sufficient.
