# Reconstruction Audits

Back: [/docs/log/reconstruction/README.md](/docs/log/reconstruction/README.md)

## Active Audit Records

| ID | Date | Scope | Status |
|---|---|---|---|
| `AUD-2026-02-11-BASELINE-01` | 2026-02-11 | baseline reconstruction for workspace and grouped crate topology | partial (topology and key blocker closed, runtime blockers still open) |

## Evidence Snapshot

| Check | Result |
|---|---|
| `src` presence before wave | absent |
| root `Cargo.toml` presence before wave | absent |
| root `Cargo.lock` presence before wave | absent |
| `rust-toolchain.toml` presence before wave | absent |

## Post-Reconstruction Evidence

| Check | Result |
|---|---|
| `cargo metadata --no-deps --format-version 1` | pass (all 20 required members resolve) |
| `cargo check --workspace` | pass |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --workspace` | pass |
| `cargo test -p kjxlkj-test-harness` | pass (group roots, member paths, fan-out, and `<=200` file lines) |
| `cargo test -p kjxlkj-test-harness --test key_mode_e2e` | pass (`KEY-TRACE-01` and `WR-01R`) |
| source files over 200 lines | none (`find src -type f -name '*.rs' ...` top file is 178 lines) |

## Improvement Ideas

| ID | Idea | Rationale | Target |
|---|---|---|---|
| `IDEA-BASELINE-LOG-01` | persist per-wave evidence rows under reconstruction audits | keeps closure evidence local to active wave | current wave |
| `IDEA-BASELINE-LOG-02` | extend topology audit with per-directory trend reporting over time | provides early warning before fan-out pressure exceeds policy | phase-5 hardening |
| `IDEA-KEY-TRACE-01` | capture last 20 raw events and resolved actions in PTY failure diagnostics | aligns with blocker diagnostic contract and speeds root-cause analysis | phase-1 follow-up |
