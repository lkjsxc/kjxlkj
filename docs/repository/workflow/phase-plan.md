# Phase Plan Contract

## Order

1. Update the docs canon.
2. Move authored tests, browser verification, and vendored assets under `src/`.
3. Refactor shared layout, summary rendering, autosave, and note responsiveness.
4. Add CI publishing and artifact upload.
5. Update tests, visual verification, and line-limit enforcement.
6. Commit only after the active batch passes its gates.

## Intent

- Keep documentation decisions ahead of code changes.
- Prefer small coherent batches over one massive unverified diff.
- Treat compose verification as the acceptance boundary for each major batch.
