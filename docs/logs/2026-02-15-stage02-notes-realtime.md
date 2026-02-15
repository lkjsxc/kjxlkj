# Stage 02: Notes and Realtime Core — Session Log

Date: 2026-02-15
Stage: 02 (Notes and Realtime Core)
Status: COMPLETE

## Summary

Implemented the notes lifecycle, WebSocket realtime patch/replay protocol,
metadata/tags, search/backlinks, and chunked attachment support.

## New Files Created

### Domain Crate
- `backlink.rs` — wiki-link `[[target]]` parser with deduplicated output
- `metadata.rs` — key/value validation (max 64 chars, reserved prefix, slug)

### DB Crate
- `repo_metadata.rs` — upsert/delete/list metadata, metadata_to_json
- `repo_tag.rs` — list_tags, find_or_create_tag, replace_note_tags
- `repo_backlink.rs` — sync_backlinks (delete+reinsert by title lookup)
- `repo_attachment.rs` — full CRUD for attachments and chunks
- `repo_workspace_event.rs` — append/list workspace events with seq
- `repo_idempotency.rs` — idempotency key store/find/cleanup
- `repo_note_snapshot.rs` — snapshot store/find functions (split from repo_note)

### HTTP Crate
- `routes_notes_rollback.rs` — POST /notes/{id}/rollback with snapshot replay
- `routes_notes_read.rs` — GET list/get/history (split from routes_notes)
- `routes_metadata.rs` — PUT/DELETE metadata, PUT tags, GET /tags
- `routes_search.rs` — GET /search (FTS), GET /notes/{id}/backlinks
- `routes_attachments.rs` — POST upload (chunked SHA-256)
- `routes_attachments_dl.rs` — GET download, DELETE (split from routes_attachments)

### WS Crate
- `messages.rs` — ClientMessage (7 variants) / ServerMessage (8 variants)
- `subscriptions.rs` — SubscriptionState with cursor tracking
- `protocol.rs` — message dispatcher with subscribe/ack handlers
- `protocol_patch.rs` — apply_patch handler (split from protocol)

### Migration
- `007_idempotency_keys.sql` — idempotency_keys + note_metadata tables

## Modified Files

- `domain/error.rs` — added WorkspaceNotFound, StaleCursor variants
- `domain/lib.rs` — exports backlink, metadata modules
- `db/repo_note.rs` — re-exports from event/snapshot splits; create_note_stream now accepts project_id + access_scope
- `db/repo_note_event.rs` — split module for event functions
- `db/lib.rs` — exports all new repo modules
- `http/dto.rs` — added Rollback/Metadata/Tag/Search/Backlink/Attachment DTOs; CreateNoteRequest has project_id + access_scope
- `http/routes_notes.rs` — VALID_KINDS/VALID_SCOPES validation; re-exports read/rollback
- `http/error_response.rs` — details field for VersionConflict/StaleCursor
- `http/lib.rs` — exports all new route modules
- `ws/session_actor.rs` — holds PgPool, SubscriptionState; JSON dispatch via protocol module
- `ws/route.rs` — passes pool+replay_batch to WsSession::new
- `ws/lib.rs` — exports new modules + apply_patch_ops helper
- `server/startup.rs` — wired all new routes

## 200-Line Splits

| Original File | Split Into | Lines |
|---|---|---|
| repo_note.rs (291) | repo_note.rs + repo_note_event.rs + repo_note_snapshot.rs | 168 + 60 + 83 |
| protocol.rs (254) | protocol.rs + protocol_patch.rs | 148 + 122 |
| routes_notes.rs (239) | routes_notes.rs + routes_notes_read.rs | 137 + 120 |
| routes_attachments.rs (215) | routes_attachments.rs + routes_attachments_dl.rs | 115 + 130 |
| startup.rs (201) | startup.rs (removed blank line) | 200 |

## Compilation Status

Zero errors, zero warnings across all 10 crates.
