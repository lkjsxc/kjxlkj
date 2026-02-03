# Decisions (2026-02-03)

Back: [/docs/log/2026/02/03/README.md](/docs/log/2026/02/03/README.md)

## Decisions

| Decision | Rationale |
|---|---|
| Implement in-repo Cargo workspace under `src/crates/` | Aligns with `docs/spec/architecture/crates.md` topology and enables continuous verification via `cargo test`/`cargo clippy`. |
| Prefer minimal but end-to-end editor slices | Maximizes correctness and testability while iterating toward the full spec surface. |
| Group workspace crates by concern under `src/crates/*/` | Keeps directories small and navigable while still satisfying “crates under `src/crates/`”. |
| Add `--headless --script` for deterministic E2E | Enables automated tests to drive modal editing and Ex commands without relying on terminal rendering capture. |
