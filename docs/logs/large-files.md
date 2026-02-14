# Source Files Exceeding 200 Lines

Back: [/docs/logs/README.md](/docs/logs/README.md)

Files listed here exceed the 200-line guideline from
[/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md).

## Current Inventory

| File | Lines | Reason | Action |
|---|---|---|---|
| `src/crates/http/kjxlkj-http/src/notes.rs` | 410 | Full CRUD + history + version conflict + search handlers | Split into sub-modules if adding more routes |
| `src/crates/db/kjxlkj-db/src/migrate.rs` | 294 | V1 (18 tables) + V2 (librarian tables) embedded SQL | Acceptable — raw SQL blocks inflate count |
| `src/crates/automation/kjxlkj-automation/src/xml_parser.rs` | 287 | XML-attrless parser with tests | Acceptable — parser + inline tests |
| `src/crates/http/kjxlkj-http/src/automation.rs` | 275 | Automation rule/run CRUD + librarian trigger | Split into sub-modules if adding more routes |
| `src/crates/ws/kjxlkj-ws/src/handlers.rs` | 259 | WS message handler implementations | Acceptable — each handler is small; grouped by concern |
| `src/crates/db/kjxlkj-db/src/repo/notes.rs` | 243 | Notes repository (create/update/list/get/history/search) | Acceptable — query functions are flat |
| `src/frontend/app/src/index.css` | 242 | Global stylesheet (app + palette + librarian + a11y) | Split into CSS modules if adding more components |
| `src/crates/domain/kjxlkj-domain/src/types.rs` | 207 | Shared domain types and enums | Acceptable — type-only file |
| `src/crates/ws/kjxlkj-ws/src/session_mgr.rs` | 206 | WS connection/subscription manager + tests | Acceptable — tests inflate count |
| `src/crates/ws/kjxlkj-ws/src/handler.rs` | 202 | WS upgrade + actor + heartbeat | Borderline — monitor |
