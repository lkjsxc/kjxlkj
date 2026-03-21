# Runtime Test Wave Evidence

This document records final closure evidence for todo `runtime-final-validation-doc-sync`.
It confirms persistent-runtime contracts remain green across cargo, compose, and CLI validation gates.

## Required Final Validation Suite

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt -- --check` | **PASS** | Exit 0 |
| `cargo clippy --all-targets -- -D warnings` | **PASS** | Exit 0 |
| `cargo test` | **PASS** | Exit 0; all suites green |
| `cargo build --release` | **PASS** | Exit 0; release build succeeded |
| `docker compose config --quiet` | **PASS** | Exit 0; compose config valid |
| `docker compose build app` | **PASS** | Exit 0; app image built |
| `docker compose --profile verify run --rm verify` | **PASS** | Exit 0; verify profile run completed |
| `cargo run --bin kjxlkj -- docs validate-topology` | **PASS** | Exit 0; `violations=0` |
| `cargo run --bin kjxlkj -- quality check-lines` | **PASS** | Exit 0; `violations=0` |
| `cargo run --bin kjxlkj -- compose verify` | **PASS** | Exit 0; all compose.verify steps passed |

## Gate Highlights

- `cargo test` completed with no failures across lib/integration/doc-test runs.
  - lib tests: 22 passed
  - integration tests: 9 passed
  - total executed tests: 31 passed
- CLI JSON summaries remained deterministic:

```json
{"command":"docs.validate-topology","directories_checked":21,"status":"pass","violations":0}
{"command":"quality.check-lines","docs_files_checked":68,"status":"pass","test_source_files_checked":0,"violations":0}
{"command":"compose.verify","exit_code":0,"status":"pass","step":"config-quiet"}
{"command":"compose.verify","exit_code":0,"status":"pass","step":"build-app"}
{"command":"compose.verify","exit_code":0,"status":"pass","step":"verify-profile-run"}
{"command":"compose.verify","status":"pass","steps_passed":3,"steps_total":3}
```

## Documentation and Contract Sync

- Root `README.md` and `docs/README.md` are coherent on document-first authority with persistent runtime artifacts permitted when contract-aligned.
- Repository governance/structure contracts now define persistent-runtime root classes in `docs/repository/structure/root-layout.md`.
- Compose contracts reference root governance and preserve prebuild + `./data` mount policy.
- Relative links in touched docs resolve.

## Constraint Checks

- Markdown max under `docs/`: `docs/containers/compose/build-storage-contract.md` at **106** lines (**<300**).
- Rust source max under `src/`: `src/web/stores.rs` at **172** lines (**<200**).

## Fixes Required During Final Gate

None. All required validation commands passed on first run; no root-cause fix iteration was needed.
