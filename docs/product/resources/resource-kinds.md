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

- Media keeps `file_href`, `content_type`, `byte_size`, `sha256_hex`, and `original_filename`.
- Images may also keep `width` and `height`.
- Videos may also keep `width`, `height`, and `duration_ms`.
- `/{ref}/file` returns the current media binary.
- `/{snapshot_id}/file` returns the immutable media binary stored on that snapshot.

## Shared Display Rules

- Public routes prefer `alias` when present.
- Normal UI does not show raw IDs as visible chrome.
- Browse, favorites, popularity, and search treat notes and media as peer resources.
- Timeline navigation is shared across notes and media.
