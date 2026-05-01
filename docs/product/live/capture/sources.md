# Live Sources

## Source Rules

- Supported video sources are `screen` and `camera`.
- Fresh installs default new live pages to `camera`.
- Screen capture uses `navigator.mediaDevices.getDisplayMedia`.
- Camera capture uses `navigator.mediaDevices.getUserMedia`.
- Admins may choose a source before starting or while broadcasting.
- Device selection is per-page state and is not persisted.

## Camera Facing

- Camera mode exposes a facing selector with `Rear` and `Front`.
- `Rear` maps to browser `facingMode: environment`.
- `Front` maps to browser `facingMode: user`.
- Fresh installs default camera facing to `Rear`.
- Facing selection may be persisted as the default through `/admin/settings`.
- Facing changes on `/live` are current-page overrides until saved in settings.
- Facing controls are disabled when source is `screen`.
- Facing constraints are browser best-effort.
- If the selected facing mode is unavailable, the app falls back to any available camera.

## Camera Picker

- Camera mode exposes a device picker populated from `enumerateDevices()`.
- The blank camera-device value means `Auto by facing`.
- A non-blank camera-device value uses exact `deviceId` and overrides facing mode.
- The camera picker may show browser-provided labels only after permission is granted.
- The camera picker updates automatically without a manual refresh button.
- Automatic refresh runs on admin page load, `devicechange`, camera source selection, successful camera capture, and page visibility return.
- Automatic refresh preserves the selected camera when that `deviceId` is still available.
- Automatic refresh resets the picker to `Auto by facing` when the selected `deviceId` disappears.
- Browsers without `devicechange` support still refresh when the page becomes visible or receives focus.
- Camera picker controls are disabled when source is `screen`.

## Browser Permissions

- Capture prompts are controlled by the browser.
- Permission denial keeps the broadcast stopped when start fails.
- Permission denial keeps the old stream when a runtime source change fails.
- Remote production capture requires HTTPS.
