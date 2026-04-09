# Purpose Contract

## Goal

`kjxlkj` is an LLM-operated resource system for Markdown notes and uploaded media.

## Product Intent

- Treat `note` and `media` as peer live resources with one shared identity model.
- Serve live resources at root paths such as `/{alias}` or `/{id}`.
- Keep immutable saved-snapshot history for both notes and media.
- Let Markdown notes embed current or saved media files directly.
- Keep admin editing fast for one operator with autosave, preview, and direct file replacement.
- Keep home, search, favorites, popularity, and analytics workable for thousands of mixed resources.
- Keep deployment single-host friendly through Docker Compose, PostgreSQL, and S3-compatible object storage.

## Non-Goals

- No multi-user collaboration.
- No backward compatibility for old routes, old schema names, or old payload shapes.
- No browser-only storage path outside the canonical runtime state stores.
