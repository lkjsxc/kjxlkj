# Live Signaling Messages

## Roles

- `broadcaster`: signed-in admin that owns the active live capture stream.
- `viewer`: any public client watching `/live`.
- Only one `broadcaster` may be active at a time.

## Message Shapes

- Messages are JSON objects with a `type` string.
- Client hello: `{ "type": "hello", "role": "broadcaster" | "viewer" }`.
- Viewer ready: `{ "type": "viewer_ready", "viewer_id": "..." }`.
- Stream state: `{ "type": "stream_started" }` and `{ "type": "stream_ended" }`.
- Viewer count: `{ "type": "viewer_count", "count": 3 }`.
- Session descriptions: `{ "type": "offer" | "answer", "viewer_id": "...", "sdp": { ... } }`.
- ICE candidates: `{ "type": "ice", "viewer_id": "...", "candidate": { ... } }`.
- Errors: `{ "type": "error", "message": "..." }`.

## Forwarding Rules

- Viewer `answer` and `ice` messages forward only to the active broadcaster.
- Broadcaster `offer` and `ice` messages forward only to the addressed viewer.
- A newly joined viewer receives `stream_started` when a broadcaster is active.
- The active broadcaster receives `viewer_ready` for each connected viewer.
- The active broadcaster receives `viewer_count` when it registers and whenever viewer count changes.
- Viewer clients never receive `viewer_count`.

## Lifetime Rules

- Broadcaster disconnect ends the active stream.
- Admin partial navigation or page unload closes the broadcaster connection.
- Viewer disconnect removes only that viewer.
- Viewer joins and disconnects update the admin-only viewer count.
- Server restart clears all live state.
- Browser refresh reconnects as a new viewer or broadcaster.
