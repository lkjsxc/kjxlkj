# Screenshot Checks Contract

## Verification Command

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Required Captures

- Desktop homepage shell with mixed resource cards.
- Desktop search page with `kind` filtering.
- Desktop live page idle viewer state.
- Desktop live page admin broadcast controls with source, camera-facing, device, quality, frame-rate, microphone, and viewer-count UI.
- Desktop admin dashboard shell.
- Desktop admin note page.
- Desktop guest media page.
- Desktop history index page.
- Desktop settings page with settings search, site icon controls, and favorite ordering.
- Compact homepage shell closed and opened.
- Compact admin note page.
- Compact admin note page with preview overlay opened.

## Required Assertions

- Admin rails keep `New note` above `Open GitHub`.
- Side menus show `Live` directly after `Home` and `Search`.
- Search, home, and dashboard cards can render notes and media in one shared visual language.
- Image media cards use a fixed-height cropped thumbnail of `128px` without breaking shared card height rules.
- Video media cards use a first-frame WebP poster when one was generated.
- Guest note preview and guest note display both render inline images from Markdown image syntax.
- Guest note preview and guest note display both render safe inline video embeds.
- Guest and admin Markdown render deterministic external URL embeds without server-side metadata fetching.
- Guest and admin Markdown render allowlisted rich external embeds without overflowing the prose column.
- Local note and media URL cards render the same resource-card language used by Home and Search.
- Markdown video embeds stay contained inside the prose surface.
- Live and media page videos expose native controls and stay contained inside their frames.
- Local media URLs in Markdown may render as thumbnail cards.
- Admin note pages expose `Upload media` beside `Show preview`.
- Uploading media from a note creates direct embeds plus media resources.
- Uploading media from a multibyte cursor position restores the caret at the server-confirmed insertion end.
- Dashboard Favorites remain read-only while settings owns favorite reordering.
- Settings search can filter visible settings items without leaving the page.
- Settings page renders one flat list without boxed setting groups.
- Settings page exposes slash-path row labels for ordinary settings.
- Nostr discovery settings validate and persist from settings.
- Live relay ICE environment is not exposed as a persisted browser setting.
- Live default source, camera-facing, quality, frame-rate, and microphone settings validate and persist from settings.
- Home-section ordering on settings uses drag-only controls.
- Site icon upload and reset work from `/admin/settings` without leaving the page.
- Guest media pages expose `Download original` while inline image display may still prefer WebP.
- Live note pages and live media pages both place `Prev`, `History`, and `Next` in the main pane rather than the rail.
- The three live-resource cards keep visibly matched widths and heights.
- Live-resource Created, Updated, and pill metadata stay left-aligned.
- Root, admin, settings, search, resource, and history main content align to the same non-rail width.
- Authenticated admin resource-page opens do not increment counted view totals.
- Leaving `/live` as an active admin broadcaster ends the stream and returns viewers to waiting.
- Resource-to-resource shell transitions preserve rail scroll state.
- Private file URLs do not leak to guest verification flows.
- History list pages require resource write permission, while known public snapshot URLs remain guest-readable.
