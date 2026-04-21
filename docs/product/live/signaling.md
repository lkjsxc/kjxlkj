# Live Signaling Contract

## Transport

- `GET /live/ws` is the WebSocket signaling endpoint.
- WebRTC carries media directly between browser peers.
- The server relays signaling messages only.
- The server stores no live media bytes.

## Roles

- `broadcaster`: signed-in admin that owns the active screen and microphone stream.
- `viewer`: any public client watching `/live`.
- Only one `broadcaster` may be active at a time.
- A second broadcaster receives an error and does not replace the active stream.

## Message Shapes

- Messages are JSON objects with a `type` string.
- Client hello: `{ "type": "hello", "role": "broadcaster" | "viewer" }`.
- Viewer ready: `{ "type": "viewer_ready", "viewer_id": "..." }`.
- Stream state: `{ "type": "stream_started" }` and `{ "type": "stream_ended" }`.
- Session descriptions: `{ "type": "offer" | "answer", "viewer_id": "...", "sdp": { ... } }`.
- ICE candidates: `{ "type": "ice", "viewer_id": "...", "candidate": { ... } }`.
- Errors: `{ "type": "error", "message": "..." }`.

## Forwarding Rules

- Viewer `answer` and `ice` messages forward only to the active broadcaster.
- Broadcaster `offer` and `ice` messages forward only to the addressed viewer.
- A newly joined viewer receives `stream_started` when a broadcaster is active.
- The active broadcaster receives `viewer_ready` for each connected viewer.

## Lifetime Rules

- Broadcaster disconnect ends the active stream.
- Viewer disconnect removes only that viewer.
- Server restart clears all live state.
- Browser refresh reconnects as a new viewer or broadcaster.

## ICE Servers

- `Live/ICE_servers_JSON` is the persisted settings source of truth.
- The setting stores a JSON array compatible with browser `RTCIceServer[]`.
- The default is one public STUN server.
- Admins may replace the array or clear it.
