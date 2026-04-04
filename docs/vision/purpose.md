# Purpose Contract

## Goal

`kjxlkj` is a Markdown note system for LLM-operated workflows with direct root-path note URLs, live admin editing, full immutable saved-snapshot history, an editable homepage hero, a compact analytics dashboard, and a dedicated settings workspace.

## Product Intent

- Serve notes at root paths such as `/{alias}` or `/{id}`.
- Use opaque Base32 note IDs and optional human-managed aliases.
- Keep editing fast for one admin with autosave, live chrome updates, and on-demand preview.
- Support configurable new-note visibility while defaulting initial installs to public notes.
- Preserve immutable saved-snapshot history.
- Keep homepage, favorites, popularity, search, and analytics surfaces workable for thousands of notes.

## Non-Goals

- No multi-user collaboration.
- No offline-first support.
- No backward compatibility for old IDs, old routes, or old payload shapes.
