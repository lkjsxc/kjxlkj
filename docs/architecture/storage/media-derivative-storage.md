# Media Derivative Storage Contract

## Object Rules

- Original media objects and derivative objects live in the same SeaweedFS S3 bucket.
- Derivative keys are opaque and scoped under the media resource id.
- Derivative objects use `image/webp` content type.
- Derivative metadata is stored on the resource and copied to saved snapshots.

## Variants

- `card`: small WebP for repeated list thumbnails.
- `display`: larger WebP for media pages and rendered local Markdown image embeds.
- `poster`: deterministic WebP poster for video media.

## Retention

- Saved snapshots keep derivative metadata from the post-save resource state.
- Object cleanup may delete derivative objects only when no live resource or saved snapshot references them.
- Missing derivative objects must degrade to original media or no poster without breaking the page.
