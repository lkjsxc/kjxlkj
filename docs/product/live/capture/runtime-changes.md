# Live Capture Runtime Changes

## Change Types

- Admins may change source, camera device, quality, frame rate, and microphone state while live.
- Source or camera-device changes reacquire the video track and replace the outgoing WebRTC video sender track.
- Quality and frame-rate changes first try `MediaStreamTrack.applyConstraints`.
- If applying constraints is not enough, the app may reacquire the selected video source and replace the sender track.
- Microphone changes may add, replace, stop, or disable audio tracks and then renegotiate peers.
- Failed runtime changes surface a visible admin status message.
- A failed runtime change must not leave viewers stuck on an active black stream.

## Track Replacement Rules

- When replacing a video track, stop the old track after removing it from the local stream.
- Update `RTCRtpSender.replaceTrack` for each peer before stopping the old track.
- When adding an audio track, call `peer.addTrack` for peers that do not yet have an audio sender.
- When removing an audio track, call `sender.replaceTrack(null)` rather than removing the sender, to preserve the transceiver for renegotiation.
