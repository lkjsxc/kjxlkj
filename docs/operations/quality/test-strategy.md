# Test Strategy Contract

## Unit Tests

- ID, alias, and settings validation.
- Title and summary derivation for notes and media.
- Markdown sanitization, local embeds, and deterministic external URL embed rendering.
- Object-storage key handling and snapshot retention rules.
- Note-attachment upload preserves raw body text and falls back to append-at-end for stale selection ranges.
- Note-attachment upload returns a server-confirmed UTF-8 cursor after insertion.
- Media derivative metadata, first-frame video posters, WebP quality validation, variant selection, and file-family no-derivative rules.
- Setup code, password change, reset-token, and login return-path validation.
- History changed-excerpt generation.

## Integration Tests

- Health endpoint behavior.
- Public and admin resource fetch behavior.
- Session-protected write behavior.
- End-to-end note create, direct media upload, note-attachment upload, metadata update, and delete lifecycle.
- Admin-only history list behavior with guest direct public snapshot access.
- Multipart media failures return JSON instead of HTML, including oversized upload requests.

## Compose Verification

- Compose verify must execute the full docs, Rust, and line-limit bundle.
- The verification stack must include PostgreSQL, SeaweedFS, and the app service.

## Browser Verification

- Browser-rendered screenshots verify desktop and compact layouts.
- Visual checks assert mixed resource cards, note-editor upload flow, media page rendering, inline note embeds, and rail ordering.
- Browser checks cover note-editor upload against trailing blank lines and stale insertion ranges.
- Browser checks cover note-editor upload against multibyte cursor positions.
- Browser checks cover first-frame video posters, video card thumbnails, file-family cards, and prose-contained video embeds.
- Browser checks cover partial shell navigation, browser back/forward, rail scroll preservation, and remembered Home/Search state from the rail.
- Browser checks cover uploaded site icon delivery, icon reset, flat settings search, settings unsaved prompts, favorite reordering from settings, login return paths, local and external URL cards, delete arming, and password flows.
- Browser checks cover unified main-pane width and left-aligned live-resource metadata.
- Browser checks cover guest-counted views versus non-counted authenticated admin opens.
