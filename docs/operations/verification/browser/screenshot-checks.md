# Screenshot Checks Contract

## Verification Command

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Required Captures

- Desktop homepage shell with mixed resource cards.
- Desktop search page with `kind` filtering.
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
- Search, home, and dashboard cards can render notes and media in one shared visual language.
- Image media cards use a fixed-height cropped thumbnail of `128px` without breaking shared card height rules.
- Video media cards use a first-frame WebP poster when one was generated.
- Guest note preview and guest note display both render inline images from Markdown image syntax.
- Guest note preview and guest note display both render safe inline video embeds.
- Markdown video embeds stay contained inside the prose surface.
- Local media URLs in Markdown may render as thumbnail cards.
- Admin note pages expose `Upload media` beside `Show preview`.
- Uploading media from a note creates direct embeds plus media resources.
- Uploading media from a multibyte cursor position restores the caret at the server-confirmed insertion end.
- Dashboard Favorites remain read-only while settings owns favorite reordering.
- Settings search can filter visible settings items without leaving the page.
- Home-section ordering on settings uses drag-only controls.
- Site icon upload and reset work from `/admin/settings` without leaving the page.
- Guest media pages expose `Download original` while inline image display may still prefer WebP.
- Authenticated admin resource-page opens do not increment counted view totals.
- Resource-to-resource shell transitions preserve rail scroll state.
- Private file URLs do not leak to guest verification flows.
- History list pages are admin-only, while known public snapshot URLs remain guest-readable.
