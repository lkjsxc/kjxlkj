# Live Page

Contracts for `/{user}/live` access, visible states, controls, and page lifetime.

## Child Index

- [states-and-controls.md](states-and-controls.md): access, viewer states, video frame, and broadcaster controls
- [lifecycle.md](lifecycle.md): start, stop, disconnect, and page-leave cleanup

## Rules

- `GET /{user}/live` is the live broadcast page for one personal space.
- The page is public-viewable.
- Authorized members can start and stop the broadcast.
- One active broadcast is allowed per personal space.
