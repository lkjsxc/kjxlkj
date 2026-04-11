# Resource Kind Contract

## Shared Resource Model

- Every live resource has `id`, optional `alias`, `body`, derived `title`, derived `summary`, visibility, favorite state, analytics, timestamps, and immutable saved snapshots.
- `id` is one 26-character lowercase Base32 identifier shared across all resource kinds.
- `alias` is globally unique across all live notes and media.
- `/{ref}` resolves alias first and then live-resource `id`.
- `/{ref}/history` opens the history index for that live resource.

## Resource Kinds

- `note`: Markdown-first document with no primary binary file.
- `media`: binary-backed resource with one current file plus Markdown body.
- `media_family` is `image` or `video`.

## Media-Specific Fields

- Media keeps the preserved raw-original `file_href`, plus `content_type`, `byte_size`, `sha256_hex`, and `original_filename`.
- Images may also keep `width` and `height`.
- Videos may also keep `width`, `height`, and `duration_ms`.
- Media may keep WebP derivative metadata for card, display, and poster contexts.
- `card` variants serve repeated cards, URL cards, and share previews for both image and video media.
- `display` variants serve inline image display on media pages and Markdown output.
- `poster` variants serve video player posters rather than list cards.
- `/{ref}/file` returns the preserved current raw-original binary.
- `/{snapshot_id}/file` returns the immutable raw-original binary stored on that snapshot.
- `/{ref}/file?variant=card|display|poster` returns the requested derivative when present.
- `/{snapshot_id}/file?variant=card|display|poster` returns the saved derivative when present.
- Derivatives accelerate inline delivery but do not replace the raw-original file routes.

## Shared Display Rules

- Public routes prefer `alias` when present.
- Normal UI does not show raw IDs as visible chrome.
- Browse, favorites, popularity, and search treat notes and media as peer resources.
- Timeline navigation is shared across notes and media.
