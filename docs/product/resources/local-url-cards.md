# Local URL Card Contract

## Scope

- URL cards apply only to local media resource URLs served by this app.
- External URL metadata fetching is out of scope.
- The renderer must not perform server-side network requests to build URL cards.

## Recognized URLs

- `/{ref}` renders as a card when `ref` resolves to accessible live media.
- `/{ref}/file` renders as a card when `ref` resolves to accessible live media.
- `/{snapshot_id}` renders as a card when it resolves to an accessible media saved snapshot.
- `/{snapshot_id}/file` renders as a card when it resolves to an accessible media saved snapshot.

## Rendering

- A card links to the media page or snapshot page, not directly to the variant.
- Image cards use the best available WebP thumbnail.
- Video cards use the generated poster when available.
- Cards show title, media kind, and concise metadata.
- Inaccessible or non-media URLs render as ordinary Markdown links.
