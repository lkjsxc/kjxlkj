# Purpose Contract

## Goal

`kjxlkj` is a Markdown note system for LLM-operated workflows with opaque note URLs, live admin editing, full revision history, and scalable public/admin indexes.

## Product Intent

- Serve notes at direct opaque URLs such as `/{id}`.
- Keep editing fast for a single admin with auto-save and live chrome updates.
- Support private-by-default notes with explicit `Public` control.
- Preserve immutable revision history.
- Keep public and admin note browsing workable for thousands of notes.

## Non-Goals

- No multi-user collaboration.
- No offline-first support.
- No backward compatibility for old datetime slug URLs or field names.
