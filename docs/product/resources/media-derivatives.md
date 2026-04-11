# Media Derivatives Contract

## Original First

- Every media upload keeps the exact original binary, content type, byte size, checksum, and filename.
- Original file URLs stay at `/{ref}/file` and `/{snapshot_id}/file` without query parameters.
- Original file URLs are the canonical raw-download surface for both current media and saved snapshots.
- Browser-hostile originals such as `HEIC` and `HEIF` still remain preserved and downloadable at those raw URLs.
- Derivatives are optional accelerators and never replace the original stored binary.

## Image Variants

- Image uploads may create WebP derivatives for card and display contexts.
- Image derivative generation should still be attempted for browser-unfriendly originals such as `HEIC` and `HEIF` when any server decoder path can read them.
- Vector images such as `SVG` should rasterize into WebP derivatives when the server can safely render them.
- Card variants target repeated list thumbnails.
- Display variants target primary media-page and Markdown display.
- Variant quality uses the persisted `media_webp_quality` setting at upload time.
- Variant generation failure must not make the original upload fail when the original can be stored.

## Video Still Images

- Video uploads keep the original video untouched.
- Video media may create both a WebP card variant and a WebP poster from the first decodable video frame.
- Still-image extraction runs server-side during upload.
- Poster generation failure must not make the original upload fail when the original can be stored.
- Existing media derivatives are immutable; first-frame card and poster generation applies to future uploads only.

## Selection

- List cards prefer `variant=card` for both image and video thumbnails.
- Media pages and Markdown output prefer `variant=display` for images when present.
- Video players use the original video as `src` and the WebP poster as `poster` when present.
- Public share cards prefer absolute WebP derivative URLs when present.
- Missing image variants may fall back to the original file only when browsers can reasonably render that original inline.
- Inline display should prefer stored WebP over raw `HEIC` or `HEIF` whenever a display or card derivative exists.

## Original Downloads

- Live media pages expose `Download original` for the current raw file.
- Saved-snapshot media pages expose `Download original` for the snapshot raw file.
- The visible download control points at the unchanged raw file URL rather than at a separate export route.

## Settings

- `media_webp_quality` is an integer setting from `1` through `100`.
- Default `media_webp_quality` is `82`.
- Changing `media_webp_quality` affects future uploads only.
