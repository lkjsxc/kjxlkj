# Live ICE Servers Contract

## Settings Source

- `Live/ICE_servers_JSON` is the persisted settings source of truth.
- The setting stores a JSON array compatible with browser `RTCIceServer[]`.
- Admins may replace the array or clear it.
- An empty array disables configured ICE servers.
- Fresh installs default to an empty array.

## Object Format

- Each element MUST be an object.
- Each object MUST have `urls`.
- `urls` is a non-empty string or a non-empty array of non-empty strings.
- TURN objects SHOULD have `username` and `credential` for authentication.
- `username` and `credential` are strings when present.

## Supported URL Schemes

- `stun:host:port` identifies a STUN service.
- `turn:host:port` identifies a TURN relay.
- `turn:host:port?transport=tcp` identifies TURN over TCP.
- `turns:host:port` identifies TURN over TLS.
- Unknown schemes are rejected during validation.

## External Ownership

- The repo does not bundle a STUN or TURN service.
- Compose does not derive ICE entries from environment variables.
- Operators configure external providers through `/admin/settings`.

## Validation Rules

- The JSON must parse as an array.
- Each element must be an object with `urls`.
- Each URL must be a non-empty string with a known scheme.
- The validator rejects arrays that contain non-objects, objects without `urls`, or empty URL strings.
