# Live Page States and Controls

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

## Viewer States

- When no broadcast is active, viewers see an idle waiting state.
- When a broadcast starts, viewers connect to the active stream without page reload.
- When a broadcast ends or disconnects, viewers return to the idle waiting state.
- Viewers do not need an account to watch.
- Viewers see native video controls for volume, fullscreen, and playback UI.
- Viewers do not see viewer count.
- Video must remain contained inside its wrapper and preserve aspect ratio; overflow is hidden.

## Non-Goals

- `/live` does not save stream replays.
- `/live` does not persist media to object storage.
- `/live` does not create resources or saved snapshots.
- `/live` does not provide chat, rooms, replay, or scheduling in v1.
- `/live` does not combine screen and camera into one composited scene.
