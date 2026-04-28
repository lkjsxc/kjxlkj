# Live Quality and Audio

## Quality

- Quality is expressed as target video height plus target frame rate.
- Height presets are `360`, `480`, `720`, `1080`, `1440`, and `2160`.
- Frame-rate presets are `15`, `30`, `45`, `60`, and `120`.
- The default quality is `1080p` at `60 fps`.
- Capture constraints are browser best-effort.

## Microphone

- Microphone capture is optional.
- The persisted default microphone state is off.
- When microphone is on, the app requests `getUserMedia({ audio: true })`.
- When microphone is off, no new audio track is requested.
- Turning microphone off while live stops the local audio track and removes audio from future peer negotiation.

## Runtime Changes

- Admins may change source, camera device, quality, frame rate, and microphone state while live.
- Source or camera-device changes reacquire video and replace the outgoing WebRTC video sender track.
- Quality and frame-rate changes first try `MediaStreamTrack.applyConstraints`.
- If constraints fail, the app may reacquire the selected video source and replace the sender track.
- Microphone changes may add, stop, or disable audio tracks and then renegotiate peers.
- Failed runtime changes surface a visible admin status message.
- A failed runtime change must not leave viewers stuck on an active black stream.
