# Live ICE Servers Contract

## Settings Source

- `Live/ICE_servers_JSON` is the persisted settings source of truth.
- The setting stores a JSON array compatible with browser `RTCIceServer[]`.
- Admins may replace the array or clear it.
- An empty array disables ICE servers entirely.

## Object Format

- Each element MUST be an object.
- Each object MUST have `urls`.
- `urls` is a non-empty string or a non-empty array of non-empty strings.
- TURN objects SHOULD have `username` and `credential` for authentication.
- `username` and `credential` are strings when present.

## Supported URL Schemes

- `stun:host:port` — Session Traversal Utilities for NAT. UDP only. No credentials.
- `turn:host:port` — TURN relay. Defaults to UDP. May include `?transport=tcp` or `?transport=udp`.
- `turns:host:port` — TURN relay over TLS. Always uses TCP. May include `?transport=tcp`.
- Unknown schemes are rejected during validation.

## Default Array

- The default includes the local coturn instance:
  - `stun:$PUBLIC_HOST:3478`
  - `turn:$PUBLIC_HOST:3478` with static username and credential
  - `turn:$PUBLIC_HOST:3478?transport=tcp` with static username and credential
  - `turns:$PUBLIC_HOST:443` with static username and credential
- `$PUBLIC_HOST` and credentials come from the Compose environment.
- If local coturn is unavailable, admins may replace the array with public STUN/TURN services.

## Validation Rules

- The JSON must parse as an array.
- Each element must be an object with `urls`.
- Each URL must be a non-empty string with a known scheme.
- The validator rejects arrays that contain non-objects, objects without `urls`, or empty URL strings.
