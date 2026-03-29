# Purpose Contract

## Goal

`kjxlkj` is a Markdown note system for LLM-operated workflows with direct root-path note URLs, live admin editing, full revision history, a homepage-like public root, and a compact admin dashboard.

## Product Intent

- Serve notes at root paths such as `/{alias}` or `/{id}`.
- Use opaque Base32 note IDs and optional human-managed aliases.
- Keep editing fast for one admin with autosave and live chrome updates.
- Support private-by-default notes with explicit `Public` control.
- Preserve immutable revision history.
- Keep recent-note, favorite, search, and stats surfaces workable for thousands of notes.

## Non-Goals

- No multi-user collaboration.
- No offline-first support.
- No backward compatibility for old IDs, old routes, or old payload shapes.
