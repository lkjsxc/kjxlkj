# Media Derivative Storage Contract

## Object Rules

- Original media objects and derivative objects live in the same SeaweedFS S3 bucket.
- Derivative keys are opaque and scoped under the media resource id.
- Derivative objects use `image/webp` content type.
- Derivative metadata is stored on the resource and copied to saved snapshots.

## Variants

- `card`: small WebP for repeated list thumbnails.
- `display`: larger WebP for media pages and rendered local Markdown image embeds.
- `poster`: first-frame WebP poster for video media.

## Retention

- Saved snapshots keep derivative metadata from the post-save resource state.
- Object cleanup may delete derivative objects only when no live resource or saved snapshot references them.
- Missing image derivatives may degrade to the raw original only when that original is browser-safe for inline display.
- Missing posters may degrade to no poster without breaking the page.
- File-family media such as `HEIC` and `HEIF` keep only the raw original and therefore never participate in derivative selection.
