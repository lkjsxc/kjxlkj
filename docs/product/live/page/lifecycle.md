# Live Lifecycle Contract

## Active Broadcast Owner

- The `/live` page owns the admin capture session.
- The broadcast is active only while the admin broadcaster WebSocket is connected.
- The server does not persist broadcast state across process restart.
- The server does not store media bytes.

## Admin Leave Behavior

- Leaving `/live` during an active broadcast ends the broadcast.
- Same-origin partial shell navigation away from `/live` ends the broadcast before the page shell is replaced.
- Full navigation, reload, browser back or forward, tab close, and `pagehide` end the broadcast.
- Ending the broadcast closes peer connections, stops local media tracks, clears the local preview, and closes the broadcaster WebSocket.
- Viewers receive `stream_ended` and return to the waiting state.

## Safety Rules

- The app must not leave the server marked live after the admin capture page is gone.
- The app must not leave viewers attached to a black stream after admin navigation.
- A second admin broadcaster must not replace the active broadcaster.
- If cleanup is invoked more than once, it remains harmless.

## Viewer Count

- Viewer count means connected viewer WebSocket registrations.
- The broadcaster admin preview is not counted as a viewer.
- Viewer count is visible only to the signed-in admin broadcaster.
- Guests and viewer-role admins do not receive viewer-count messages.
