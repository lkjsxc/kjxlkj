# File-Family Media Contract

## Scope

- `file` is a media family for binary uploads that should not be treated as inline images or inline videos.
- `HEIC` and `HEIF` belong to this family in `kjxlkj`.
- Broader arbitrary-file support is out of scope for now. The direct contract change here is the `HEIC` and `HEIF` move.

## Create Rules

- Direct media create still accepts the current image and video formats plus `HEIC` and `HEIF`.
- `HEIC` and `HEIF` no longer count as image uploads for rendering or derivative purposes.
- File-family media still seed `body` from the uploaded filename stem as a `# Heading`.

## Display Rules

- File-family media pages are download-first rather than preview-first.
- Live file-family pages reuse the same top-row `Prev` / `History` / `Next` shell used by live notes and other live media pages.
- Guest and admin file-family pages expose `Download original` and the Markdown body.
- File-family list cards and local URL cards use text metadata rather than generated thumbnails.
- File-family resources remain first-class peers in search, favorites, popularity, history, and paging.

## Markdown Rules

- Auto-inserted note attachments for file-family media use page links such as `[report.heic](/alias-or-id)`.
- Direct raw-file links such as `/alias-or-id/file` remain valid when download-first Markdown is desired.
- File-family links may render as local file cards when the target media page is accessible.
