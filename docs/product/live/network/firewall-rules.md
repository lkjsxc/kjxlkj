# Live Firewall Rules

## HTTP and WebSocket

- `80/tcp` reaches edge nginx for redirect or certificate handling.
- `443/tcp` reaches edge nginx for HTTPS.
- `/live/ws` must preserve `Upgrade` and `Connection` headers.
- `/live/ws` should use long proxy read and send timeouts.

## App ICE

- `${LIVE_ICE_UDP_PORT}/udp` reaches the `app` container.
- The default live ICE UDP port is `8189`.
- The port must be reachable from remote broadcaster and viewer browsers.
- Docker, Incus, and edge NAT must preserve UDP traffic for this port.

## Optional TURN

- `3478/udp` may reach edge coturn.
- `3478/tcp` may reach edge coturn.
- `49152-65535/udp` may reach edge coturn relay ports.
- TURN TLS on `5349/tcp` is optional, not required for the target setup.

## Incus Ownership

- Public port proxy devices for live streaming belong on `edge`.
- Equivalent stale proxy devices should be removed from `lkjsxc`.
- Moving proxy devices must not alter the Portainer deployment flow.
