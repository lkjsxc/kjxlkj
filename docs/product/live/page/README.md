# Live Page

Contracts for `/live` access, visible states, controls, and page lifetime.

## Child Index

- [states-and-controls.md](states-and-controls.md): access, viewer states, video frame, and admin controls
- [lifecycle.md](lifecycle.md): start, stop, disconnect, and page-leave cleanup

## Rules

- `GET /live` is the single site-wide live broadcast page.
- The page is public-viewable.
- Signed-in admins can start and stop the broadcast.
- v1 supports exactly one active broadcast and no named rooms.
