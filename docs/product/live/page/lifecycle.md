# Live Lifecycle

## Ownership

- The `/live` page owns the admin capture session.
- The active admin WebSocket owns the server-side broadcaster role.
- The app process owns the in-memory live relay.
- The server stores no live media bytes.
- Server restart clears all live state.

## Start

- Starting a broadcast captures selected local tracks first.
- The app opens the broadcaster WebSocket after capture succeeds.
- The broadcaster publishes one WebRTC peer connection to the app.
- A second broadcaster receives an error and does not replace the active stream.
- Existing viewers receive `stream_started` after the broadcaster registers.

## Stop

- Stopping the broadcast closes the broadcaster peer connection.
- Stopping the broadcast closes all viewer relay peer connections.
- Stopping the broadcast stops all local capture tracks.
- Stopping the broadcast closes the broadcaster WebSocket.
- Viewers receive `stream_ended` and return to idle.

## Leave

- Leaving `/live` during an active broadcast ends the broadcast.
- Same-origin partial shell navigation away from `/live` ends the broadcast before the page shell is replaced.
- Browser refresh closes the old broadcast and may reconnect as a new broadcaster or viewer.
- Viewer disconnect removes only that viewer.

## Failure

- Capture failure leaves the server with no active broadcaster.
- WebSocket close by the broadcaster ends the active stream.
- Broadcaster ICE failure ends the active stream.
- Viewer ICE failure closes only the affected viewer peer connection.
- The app must not leave the server marked live after the admin capture page is gone.
