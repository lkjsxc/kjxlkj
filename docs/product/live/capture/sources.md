# Live Capture Sources

## Supported Sources

- Admins choose exactly one active video source before starting or while broadcasting.
- Supported video sources are `screen` and `camera`.
- Screen capture uses `navigator.mediaDevices.getDisplayMedia`.
- Camera capture uses `navigator.mediaDevices.getUserMedia`.
- Camera mode exposes a device picker populated from `enumerateDevices()`.
- The camera picker may show browser-provided labels only after permission is granted.
- Device selection is per-page state and is not persisted.

## Microphone

- Microphone capture is optional.
- The persisted default microphone state is off.
- When microphone is on, the app requests `getUserMedia({ audio: true })`.
- When microphone is off, no new audio track is requested.
- Turning microphone off while live stops or disables the local audio track and removes audio from future peer negotiation.
