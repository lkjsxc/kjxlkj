# Live Signaling Messages

## Roles

- `broadcaster`: signed-in admin that publishes the active live capture stream to the app.
- `viewer`: any public client receiving the app-relayed stream.
- Only one `broadcaster` may be active at a time.

## Message Shapes

- Messages are JSON objects with a `type` string.
- Client hello: `{ "type": "hello", "role": "broadcaster" | "viewer" }`.
- Stream state: `{ "type": "stream_started" }` and `{ "type": "stream_ended" }`.
- Viewer count: `{ "type": "viewer_count", "count": 3 }`.
- Publish offer: `{ "type": "publish_offer", "sdp": { ... } }`.
- View offer: `{ "type": "view_offer", "sdp": { ... } }`.
- Server answer: `{ "type": "answer", "sdp": { ... } }`.
- ICE candidates: `{ "type": "ice", "candidate": { ... } }`.
- Errors: `{ "type": "error", "message": "..." }`.

## Negotiation Rules

- The broadcaster sends one `publish_offer` to the server.
- The browser publisher gives each sent RTP encoding a stable RID.
- The server registers `on_track` before applying the broadcaster offer.
- The server answers the broadcaster from the transceivers created by that offer.
- The server must not add synthetic publisher recvonly transceivers before reading the offer.
- Each viewer sends one `view_offer` to the server after `stream_started`.
- The server answers each viewer with recv media sections for active tracks.
- ICE candidates are scoped to the connected WebSocket session.
- Candidate messages never include a viewer id.
- The active broadcaster receives `viewer_count` when it registers and whenever viewer count changes.
- Viewer clients never receive `viewer_count`.
- Broadcaster shutdown sends `stream_ended` before the server closes viewer peer connections.

## Media Relay Rules

- The server accepts one VP8 video track from the broadcaster.
- The server accepts one optional Opus audio track from the broadcaster.
- The server forwards RTP packets from broadcaster tracks to viewer tracks.
- Viewer `ontrack` only proves SDP negotiation; playback requires inbound RTP stats and advancing video time.
- The server does not persist, inspect, transcode, or save media bytes.
- Browser-to-browser RTP, SDP, and ICE exchange is forbidden.

## Lifetime Rules

- Broadcaster disconnect ends the active stream.
- Admin partial navigation or page unload closes the broadcaster connection.
- Viewer disconnect removes only that viewer.
- Viewer joins and disconnects update the broadcaster-only viewer count.
- Server restart clears all live state.
- Browser refresh reconnects as a new viewer or broadcaster.
