# Live Capture Contract

## Sources

- Admins choose exactly one active video source before starting or while broadcasting.
- Supported video sources are `screen` and `camera`.
- Screen capture uses `navigator.mediaDevices.getDisplayMedia`.
- Camera capture uses `navigator.mediaDevices.getUserMedia`.
- Camera mode exposes a device picker populated from `enumerateDevices()`.
- The camera picker may show browser-provided labels only after permission is granted.
- Device selection is per-page state and is not persisted.

## Quality

- Quality is expressed as target video height plus target frame rate.
- Height presets are `360`, `480`, `720`, `1080`, `1440`, and `2160`.
- Frame-rate presets are `15`, `30`, `45`, `60`, and `120`.
- The default quality is `1080p` at `60 fps`.
- Capture constraints are browser best-effort.
- If a new runtime constraint fails, the active stream remains unchanged when possible.

## Microphone

- Microphone capture is optional.
- The persisted default microphone state is off.
- When microphone is on, the app requests `getUserMedia({ audio: true })`.
- When microphone is off, no new audio track is requested.
- Turning microphone off while live stops or disables the local audio track and removes audio from future peer negotiation.

## Runtime Changes

- Admins may change source, camera device, quality, frame rate, and microphone state while live.
- Source or camera-device changes reacquire the video track and replace the outgoing WebRTC video sender track.
- Quality and frame-rate changes first try `MediaStreamTrack.applyConstraints`.
- If applying constraints is not enough, the app may reacquire the selected video source and replace the sender track.
- Microphone changes may add, replace, stop, or disable audio tracks and then renegotiate peers.
- Failed runtime changes surface a visible admin status message.
- A failed runtime change must not leave viewers stuck on an active black stream.
