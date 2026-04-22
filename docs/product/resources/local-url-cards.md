# Local URL Card Contract

## Scope

- URL cards apply to local resource URLs served by this app.
- Root-relative local URLs and absolute URLs under configured `public_base_url` are equivalent.
- External URL embed rules are owned by [external-url-embeds.md](external-url-embeds.md).
- The renderer must not perform server-side network requests to build local or external cards.

## Recognized URLs

- `/{ref}` renders as a card when `ref` resolves to an accessible live resource.
- `/{snapshot_id}` renders as a card when it resolves to an accessible saved snapshot.
- `/{ref}/file` and `/{snapshot_id}/file` may render as cards when the renderer can safely derive the owning resource page.
- Absolute public-origin versions of the same paths follow the same rules.

## Rendering

- Page-link cards link to the media page or snapshot page, not directly to a variant.
- Note cards reuse the same title, summary, kind badge, and timestamp language as Home and Search cards.
- Image cards use the best available WebP `variant=card` thumbnail.
- Video cards use the generated WebP `variant=card` thumbnail when available.
- File-family cards use text metadata only and do not request thumbnail variants.
- Cards show title, media kind, and concise metadata.
- Inaccessible URLs render as ordinary Markdown links for guests.
- Private local resources render as rich cards only for signed-in admins.
- Local image embeds are not URL cards and use the owner-note click rules from [embed-rules.md](embed-rules.md).
- Local card rendering must not inline another rendered note body.
