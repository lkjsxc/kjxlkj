# Media Derivatives Contract

## Original First

- Every media upload keeps the original binary, content type, byte size, checksum, and filename.
- Original file URLs stay at `/{ref}/file` and `/{snapshot_id}/file` without query parameters.
- Derivatives are optional accelerators and never replace the original stored binary.

## Image Variants

- Image uploads may create WebP derivatives for card and display contexts.
- Card variants target repeated list thumbnails.
- Display variants target primary media-page and Markdown display.
- Variant quality uses the persisted `media_webp_quality` setting at upload time.
- Variant generation failure must not make the original upload fail when the original can be stored.

## Video Posters

- Video uploads keep the original video untouched.
- Video media may create a deterministic WebP poster from media metadata.
- The poster is not extracted from the first frame.
- Video poster generation must be deterministic under browser verification.

## Selection

- List cards prefer `variant=card` for image thumbnails and video posters.
- Media pages and Markdown output prefer `variant=display` for images when present.
- Video players use the original video as `src` and the WebP poster as `poster` when present.
- Missing variants fall back to the original file for images and to no poster for videos.

## Settings

- `media_webp_quality` is an integer setting from `1` through `100`.
- Default `media_webp_quality` is `82`.
- Changing `media_webp_quality` affects future uploads only.
