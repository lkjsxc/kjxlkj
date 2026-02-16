# Source Files Exceeding 200 Lines

Back: [/docs/logs/README.md](README.md)

Per [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md), source files exceeding 200 lines
are noted here for potential future splitting.

## Current Audit

| File | Lines | Reason | Split Candidate |
|---|---:|---|---|
| `src/frontend/app/src/components/app-shell.ts` | 422 | Lit component with all views inline | yes — extract per-view components |
| `src/crates/http/kjxlkj-http/src/routes_note.rs` | 306 | 6 route handlers + types | yes — split into routes_note_read.rs / routes_note_write.rs |
| `src/crates/db/kjxlkj-db/src/repo_note.rs` | 302 | note CRUD + projection + patching | yes — split into repo_note_read.rs / repo_note_write.rs |
| `src/crates/ws/kjxlkj-ws/src/session.rs` | 229 | WebSocket actor with handlers | moderate — could extract message dispatch |
| `src/crates/db/kjxlkj-db/migrations/001_initial_schema.sql` | 218 | single migration with 16 tables | no — must remain single for migration ordering |
| `src/crates/db/kjxlkj-db/src/repo_automation.rs` | 205 | automation rules + runs + KV store | moderate — could extract kv_store.rs |

## Policy

Files listed here are functional and correct. Splitting is a future hardening
activity tracked in [/docs/todo/waves/stage-10-hardening-and-investigation/wave-101.md](/docs/todo/waves/stage-10-hardening-and-investigation/wave-101.md).
