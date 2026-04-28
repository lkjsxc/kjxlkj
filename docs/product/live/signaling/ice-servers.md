# Live ICE Servers

## Settings Source

- `Live/ICE_servers_JSON` is the persisted settings source of truth.
- The setting stores a JSON array compatible with browser `RTCIceServer[]`.
- Fresh installs default to one public Google STUN server.
- Production should replace the default with operator-owned STUN/TURN entries.
- Admins may replace the array or clear it.
- An empty array disables configured ICE servers.

## Object Format

- Each element must be an object.
- Each object must have `urls`.
- `urls` is a non-empty string or a non-empty array of non-empty strings.
- TURN objects should have `username` and `credential`.
- A TURN object with `username` must also have `credential`.

## Supported URL Schemes

- `stun:host:port` identifies a STUN service.
- `turn:host:port` identifies a TURN relay.
- `turn:host:port?transport=tcp` identifies TURN over TCP.
- `turns:host:port` identifies TURN over TLS.
- Unknown schemes are rejected during validation.

## Production Example

```json
[
  { "urls": ["stun:turn.lkjsxc.com:3478"] },
  { "urls": ["turn:turn.lkjsxc.com:3478"], "username": "kjxlkj", "credential": "secret" },
  { "urls": ["turn:turn.lkjsxc.com:3478?transport=tcp"], "username": "kjxlkj", "credential": "secret" }
]
```
