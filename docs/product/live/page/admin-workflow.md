# Live Admin Workflow

## Broadcast Controls

- Admins can start a broadcast from `/live`.
- Admins choose screen or camera as the single active video source.
- Admins can choose camera device, target quality, target frame rate, and microphone state.
- Persisted defaults come from `/admin/settings`.
- `/live` controls may override the persisted defaults for the current page session.
- Default quality is `1080p` at `60 fps`.
- Default microphone state is off.
- Capture details are owned by [capture/quality-and-constraints.md](../capture/quality-and-constraints.md).
- The browser may require HTTPS or localhost for capture APIs.
- Stopping the broadcast ends all local tracks and notifies viewers.
- Navigating away from `/live` while broadcasting ends the stream.
- Leave and cleanup rules are owned by [lifecycle.md](lifecycle.md).
- Viewer count is visible only to the admin broadcaster.

## Source Selection

- Source options are `screen` and `camera`.
- Camera mode exposes a device picker populated from `enumerateDevices()`.
- The camera picker may show browser-provided labels only after permission is granted.
- Device selection is per-page state and is not persisted.
- Refreshing the camera list re-queries `enumerateDevices()`.

## Runtime Overrides

- Admins may change source, camera device, quality, frame rate, and microphone state while live.
- Source or camera-device changes reacquire the video track and replace the outgoing WebRTC video sender track.
- Quality and frame-rate changes first try `MediaStreamTrack.applyConstraints`.
- If applying constraints is not enough, the app may reacquire the selected video source and replace the sender track.
- Microphone changes may add, replace, stop, or disable audio tracks and then renegotiate peers.
- Failed runtime changes surface a visible admin status message.
- A failed runtime change must not leave viewers stuck on an active black stream.
