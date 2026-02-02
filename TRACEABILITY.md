# Traceability

This document maps normative documentation requirements to implementation anchors.

## Runtime & Architecture

| Requirement source | Requirement (summary) | Implementation anchor(s) |
|---|---|---|
| docs/policy/README.md (Key Invariants) | Single-binary runtime (no plugin loading) | src/crates/kjxlkj/src/main.rs, Cargo.toml workspace |
| docs/policy/README.md (Key Invariants) | No mouse support; keyboard-only interaction | src/crates/kjxlkj-input (terminal key parsing); docs/spec/features/config/mouse-support.md (spec) |
| docs/policy/README.md (Concurrency baseline) | Tokio async-first services; core remains single-writer | src/crates/kjxlkj-services/src/supervisor.rs; src/crates/kjxlkj-core-state |
| docs/spec/architecture/runtime.md (Runtime topology) | Core task ↔ bounded message bus ↔ services; render consumes snapshots | src/crates/kjxlkj-services/src/bus.rs; src/crates/kjxlkj-render/src/renderer.rs; src/crates/kjxlkj-core-state |

## Crate Topology

| Requirement source | Requirement (summary) | Implementation anchor(s) |
|---|---|---|
| docs/spec/architecture/crates.md | ≥10 crates under src/crates | src/crates/* (see Cargo.toml members) |
| docs/spec/architecture/crates.md | Core facade + split subcrates (types/text/edit/mode/undo/ui/state) | src/crates/kjxlkj-core-* |
| docs/spec/architecture/crates.md | Services split by domain (lsp/git/index/fs/terminal) | src/crates/kjxlkj-service-* |

## Validation Gates

| Requirement source | Requirement (summary) | Implementation anchor(s) |
|---|---|---|
| docs/policy/WORKFLOW.md (DoD) | clippy/test verification | CI-equivalent local loop: `cargo clippy -- -D warnings`, `cargo test` |
| docs/policy/STRUCTURE.md | ≤200 lines per file | Repository-wide invariant (validated by local scans) |

