# Local URL Card Contract

## Scope

- URL cards apply only to local media resource URLs served by this app.
- External URL metadata fetching is out of scope.
- The renderer must not perform server-side network requests to build URL cards.

## Recognized URLs

- `/{ref}` renders as a card when `ref` resolves to accessible live media.
- `/{snapshot_id}` renders as a card when it resolves to an accessible media saved snapshot.
- `/{ref}/file` and `/{snapshot_id}/file` may render as cards when the renderer can safely derive the owning media page.

## Rendering

- Page-link cards link to the media page or snapshot page, not directly to a variant.
- Image cards use the best available WebP `variant=card` thumbnail.
- Video cards use the generated WebP `variant=card` thumbnail when available.
- File-family cards use text metadata only and do not request thumbnail variants.
- Cards show title, media kind, and concise metadata.
- Inaccessible or non-media URLs render as ordinary Markdown links.
