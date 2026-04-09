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
- Desktop admin media page.
- Desktop guest media page.
- Desktop history index page.
- Compact homepage shell closed and opened.
- Compact admin note page.
- Compact admin media page.
- Compact admin note page with preview overlay opened.

## Required Assertions

- Admin rails keep `New note` above `New media`, and `New media` above `Open GitHub`.
- Search, home, and dashboard cards can render notes and media in one shared visual language.
- Image media cards may show a thumbnail without breaking shared card height rules.
- Guest note preview and guest note display both render inline images from Markdown image syntax.
- Guest note preview and guest note display both render safe inline video embeds.
- Admin media pages expose a file preview plus a Markdown body editor.
- Media replacement keeps current-page preview usable after the new file is stored.
- Private file URLs do not leak to guest verification flows.
