# 2026-02-15 Autosave Conflict Hardening

## Scope

- Eliminate `PATCH /api/notes/{id}` `500` errors caused by concurrent autosave updates.
- Ensure frontend autosave does not issue overlapping mutation requests for the same note.

## Changes

### Backend

- File: `src/crates/http/kjxlkj-http/src/routes_notes_patch.rs`
- Behavior change:
  - On `note_events(note_id, seq)` unique violations (`23505`), return `VERSION_CONFLICT` (`409`) instead of `INTERNAL_ERROR` (`500`).
  - Conflict `found` version now resolves to at least `stream.current_version + 1` to avoid stale same-version responses during concurrent writers.

### Frontend

- File: `src/frontend/app/src/hooks/useEditor.ts`
- Behavior change:
  - Added in-flight autosave lock and queued-save flag to serialize saves.
  - Autosave now runs at most one mutation cycle at a time and performs follow-up save only when needed.
  - Save result dispatch (`saved`/`conflict`/`error`) is ignored if the user switched to a different note mid-request.
  - Added `autosave` option so non-editor consumers can read actions without spawning an autosave loop.

### Frontend Follow-Up Root Cause

- Files:
  - `src/frontend/app/src/views/NotesLayout.tsx`
  - `src/frontend/app/src/views/NoteDetail.tsx`
- Root cause:
  - `useEditor()` was mounted in both layout and detail components, creating two independent autosave timers and duplicate PATCH attempts.
- Fix:
  - `NotesLayout` now uses `useEditor({ autosave: false })` because it only needs `clear()`.
  - `NoteDetail` remains the single autosave owner.

## Verification

- Reproduced old behavior before rebuild:
  - Concurrent PATCH with same `base_version` returned multiple `500 INTERNAL_ERROR` duplicate-key responses.
- Verified after backend rebuild:
  - Same race now returns one `200` + remaining `409 VERSION_CONFLICT`.
- Verified after final rebuild:
  - Conflict details report progressed version (`expected: 9, found: 10`) under race.
- Verified bundle rollout:
  - app now serves `index-CWmzYSIP.js` containing the autosave-owner fix.
- Local checks:
  - `cargo check -p kjxlkj-http` passed.
  - `npm run build` in `src/frontend/app` passed.
- TODO scan command:
  - `rg -n "\[ \]" docs/todo` returned no matches.

## File Length Audit

- Source file over 200 lines:
  - `src/crates/http/kjxlkj-http/src/routes_notes_patch.rs` (228 lines)

## Improvement Ideas

- Add integration test that concurrently PATCHes the same note and asserts `409` (never `500`).
- Move note event append + projection update into one transactional repository method.
- Add structured request logging for note mutation endpoints with note/version correlation.
