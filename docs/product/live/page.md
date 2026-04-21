# Live Page Contract

## Route

- `GET /live` is the single site-wide live broadcast page.
- The page is public-viewable.
- Signed-in admins can start and stop the broadcast.
- Guests and admins can watch the current broadcast.
- v1 supports exactly one active broadcast and no named rooms.
- `/live` video elements expose browser-native controls.
- Live video must stay contained inside its frame and preserve aspect ratio.

## Navigation

- The side menu includes `Live` directly after `Home` and `Search`.
- `Live` is visible to guests and admins.
- The active state is `Live` when the current page is `/live`.

## Admin Broadcast Controls

- Admins can start a broadcast from `/live`.
- Admins choose screen or camera as the single active video source.
- Admins can choose camera device, target quality, target frame rate, and microphone state.
- Persisted defaults come from `/admin/settings`.
- `/live` controls may override the persisted defaults for the current page session.
- Default quality is `1080p` at `60 fps`.
- Default microphone state is off.
- Capture details are owned by [capture.md](capture.md).
- The browser may require HTTPS or localhost for capture APIs.
- Stopping the broadcast ends all local tracks and notifies viewers.
- Navigating away from `/live` while broadcasting ends the stream.
- Leave and cleanup rules are owned by [lifecycle.md](lifecycle.md).
- Viewer count is visible only to the admin broadcaster.

## Viewer States

- When no broadcast is active, viewers see an idle waiting state.
- When a broadcast starts, viewers connect to the active stream without page reload.
- When a broadcast ends or disconnects, viewers return to the idle waiting state.
- Viewers do not need an account to watch.
- Viewers see native video controls for volume, fullscreen, and playback UI.
- Viewers do not see viewer count.

## Non-Goals

- `/live` does not save stream replays.
- `/live` does not persist media to object storage.
- `/live` does not create resources or saved snapshots.
- `/live` does not provide chat, rooms, replay, or scheduling in v1.
- `/live` does not combine screen and camera into one composited scene.
