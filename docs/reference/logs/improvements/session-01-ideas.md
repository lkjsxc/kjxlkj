# Improvement Ideas — Captured During Reconstruction

## High Priority

- **IMP-ARC-01**: SQLx compile-time query checking needs offline sqlx-data.json cache
  once PostgreSQL is integrated. Track in S10/W100.

- **IMP-STRUCT-01**: Route splitting complete. routes.rs went from 284→50 lines.
  Future route additions should follow the per-resource module pattern.

- **IMP-TEST-03**: Integration test harness needs ephemeral PostgreSQL or SQLite
  fallback for CI. Currently all tests are unit-level.

## Medium Priority

- **IMP-FE-02**: Rich markdown editor (CodeMirror/ProseMirror) is critical for
  production-quality editing. Currently using plain HTML.

- **IMP-SEC-02**: Auth endpoint rate limiting not yet implemented. Must add before
  any public deployment.

- **IMP-OPS-01**: Replace println! with tracing crate once runtime is fully wired.

## Observations

- The wiki-link parser had a subtle borrow-checker issue with Peekable iterators.
  The byte-based approach is cleaner and avoids the double-mutable-borrow problem.

- include_str! paths are fragile — they're relative to the source file location.
  Consider a build.rs approach for embedding data files.

- The hybrid search score formula (0.5*lex + 0.5*sem) should be configurable
  per workspace or via config.json.
