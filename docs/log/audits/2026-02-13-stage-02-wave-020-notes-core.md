# Audit: Stage 02 Wave 020 Notes Core

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for Wave 020:

- notes CRUD baseline
- note history and rollback
- optimistic version conflict semantics

## Implementation Summary

- added Stage 02 DB migration for notes/event-sourcing projections and indices
- implemented note repositories for create/list/get/patch/title/delete/history/rollback
- implemented patch application semantics (`retain`/`insert`/`delete`) with validation
- implemented soft-delete exclusion from default note list
- implemented version conflict detection with deterministic conflict payload context
- wired HTTP routes:
  - `POST /api/notes`
  - `GET /api/notes`
  - `GET /api/notes/{id}`
  - `PATCH /api/notes/{id}`
  - `PATCH /api/notes/{id}/title`
  - `DELETE /api/notes/{id}`
  - `GET /api/notes/{id}/history`
  - `POST /api/notes/{id}/rollback`

## Deterministic Checks

### Check 1: compile

```bash
cargo check --workspace --tests
```

Result: pass.

Proof:

```text
Checking kjxlkj-db v0.1.0
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
```

### Check 2: Wave 020 note lifecycle/conflict test

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-db --test notes_flow -- --nocapture
```

Result: pass.

Proof:

```text
running 1 test
test notes_crud_history_rollback_and_conflict_flow ... ok

test result: ok. 1 passed; 0 failed
```

## Conclusion

Wave 020 note CRUD/history/rollback/versioning requirements are implemented with deterministic integration evidence. Wave 021 (WS replay/idempotency/conflict protocol) is the next ordered implementation step.