# Decisions (2026-02-03)

Back: [/docs/log/2026/02/03/README.md](/docs/log/2026/02/03/README.md)

## Decisions

| Decision | Rationale |
|---|---|
| Implement in-repo Cargo workspace under `src/crates/` | Aligns with `docs/spec/architecture/crates.md` topology and enables continuous verification via `cargo test`/`cargo clippy`. |
| Prefer minimal but end-to-end editor slices | Maximizes correctness and testability while iterating toward the full spec surface. |
| Group workspace crates by concern under `src/crates/*/` | Keeps directories small and navigable while still satisfying “crates under `src/crates/`”. |
| Add `--headless --script` for deterministic E2E | Enables automated tests to drive modal editing and Ex commands without relying on terminal rendering capture. || Split large files to comply with 200-line policy | Files split: `buffer.rs` → `buffer.rs` + `buffer_ops.rs`; `apply.rs` → `apply.rs` + `apply_edit.rs`; `tests.rs` → `tests.rs` + `tests_e2e.rs`; `frame.rs` → `frame.rs` + `frame_widgets.rs`. |
| Mode-aware cursor clamping | Insert mode allows cursor at line_len (past last char) for correct text insertion; normal mode clamps to last character. |
| Fix redo to return original transaction | Undo returns inverse ops; redo returns original ops to replay the edit. |