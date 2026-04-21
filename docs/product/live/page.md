# Live Page Contract

## Route

- `GET /live` is the single site-wide live broadcast page.
- The page is public-viewable.
- Signed-in admins can start and stop the broadcast.
- Guests and admins can watch the current broadcast.
- v1 supports exactly one active broadcast and no named rooms.

## Navigation

- The side menu includes `Live` directly after `Home` and `Search`.
- `Live` is visible to guests and admins.
- The active state is `Live` when the current page is `/live`.

## Admin Broadcast Controls

- Admins can start a broadcast from `/live`.
- Starting a broadcast requests browser screen share and microphone audio.
- The app uses `getDisplayMedia` for the screen track.
- The app uses `getUserMedia` for the microphone track.
- The browser may require HTTPS or localhost for capture APIs.
- Stopping the broadcast ends all local tracks and notifies viewers.

## Viewer States

- When no broadcast is active, viewers see an idle waiting state.
- When a broadcast starts, viewers connect to the active stream without page reload.
- When a broadcast ends or disconnects, viewers return to the idle waiting state.
- Viewers do not need an account to watch.

## Non-Goals

- `/live` does not save stream replays.
- `/live` does not persist media to object storage.
- `/live` does not create resources or saved snapshots.
- `/live` does not provide chat, rooms, replay, or scheduling in v1.
