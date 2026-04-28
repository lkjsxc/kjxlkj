# Live Sources

## Source Rules

- Supported video sources are `screen` and `camera`.
- Screen capture uses `navigator.mediaDevices.getDisplayMedia`.
- Camera capture uses `navigator.mediaDevices.getUserMedia`.
- Admins may choose a source before starting or while broadcasting.
- Device selection is per-page state and is not persisted.

## Camera Picker

- Camera mode exposes a device picker populated from `enumerateDevices()`.
- The camera picker may show browser-provided labels only after permission is granted.
- Refreshing cameras updates the picker without persisting a default.
- Camera picker controls are disabled when source is `screen`.

## Browser Permissions

- Capture prompts are controlled by the browser.
- Permission denial keeps the broadcast stopped when start fails.
- Permission denial keeps the old stream when a runtime source change fails.
- Remote production capture requires HTTPS.
