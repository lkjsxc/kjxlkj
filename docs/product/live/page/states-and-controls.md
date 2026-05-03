# Live States and Controls

## Route

- `GET /live` is public.
- Guests and members can watch public broadcasts.
- Members with broadcast permission see broadcast controls.
- `/live` video elements expose browser-native controls.
- Live video stays contained inside its frame and preserves aspect ratio.
- The video frame must not expose colored browser or codec edge artifacts.

## Navigation

- The side menu includes `Live` directly after `Home` and `Search`.
- `Live` is visible to guests and members.
- The active state is `Live` when the current page is `/live`.

## Admin Controls

- Broadcasters choose screen or camera as the active video source.
- Admins can choose camera facing, exact camera device, target quality, target frame rate, and microphone state.
- Admins never need a manual camera-refresh action.
- Camera device options update automatically as devices appear, disappear, or receive labels after permission.
- The default source is camera.
- The default camera facing is rear.
- Persisted defaults come from `/{user}/settings`.
- `/live` controls may override persisted defaults for the current page session.
- Viewer count is visible only to the admin broadcaster.

## Viewer States

- When no broadcast is active, viewers see an idle waiting state.
- When a broadcast starts, viewers connect without page reload.
- While media negotiation is pending, viewers see a connecting state.
- When media arrives, viewers see a playing state.
- When server negotiation, connection, or ICE negotiation fails, viewers see a visible failure state.
- When a broadcast ends or disconnects, viewers return to the idle waiting state.
- Viewers do not need an account to watch.
- Viewers do not see viewer count.

## Non-Goals

- `/live` does not save stream replays.
- `/live` does not persist media to object storage.
- `/live` does not create resources or saved snapshots.
- `/live` does not provide chat, rooms, replay, or scheduling in v1.
- `/live` does not combine screen and camera into one composited scene.
