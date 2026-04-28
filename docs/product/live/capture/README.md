# Live Capture

Contracts for capture sources, quality presets, microphone behavior, and runtime changes.

## Child Index

- [sources.md](sources.md): screen, camera, device picker, and browser permission rules
- [quality-and-audio.md](quality-and-audio.md): height, frame-rate, microphone, and runtime changes

## Rules

- Admins choose exactly one active video source.
- Capture APIs require `localhost` or HTTPS in browsers.
- Runtime control changes should keep the existing stream alive when possible.
