# Reconstruction Audits

Back: [/docs/log/reconstruction/README.md](/docs/log/reconstruction/README.md)

## Active Audit Records

| ID | Date | Scope | Status |
|---|---|---|---|
| `AUD-2026-02-11-BASELINE-01` | 2026-02-11 | baseline reconstruction for workspace and grouped crate topology | verified for high-severity blocker closure (medium hardening gaps remain) |

Detailed per-wave records are stored in [waves/README.md](waves/README.md).

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
| `cargo test -p kjxlkj-core-state` | pass (window-tree split/focus/geometry invariants foundation tests) |
| `cargo test -p kjxlkj-test-harness --test window_nav_e2e` | pass (`WIN-01R` split lifecycle baseline and `WINNAV-06R` replay determinism baseline) |
| `cargo test -p kjxlkj-test-harness --test window_nav_more_e2e` | pass (`WIN-02R`, `WIN-03R`, `WIN-04R`, `WINNAV-01R`..`WINNAV-04R`) |
| `cargo test -p kjxlkj-test-harness --test window_nav_session_terminal_e2e` | pass (`WIN-05R`, `WINNAV-05R`) |
| `cargo test -p kjxlkj-test-harness --test explorer_terminal_paths_e2e` | pass (route/open-target/mixed-navigation/resize baselines for `EXP-01R`..`EXP-03R`, `TERM-01R`..`TERM-04R`) |
| `cargo test -p kjxlkj-test-harness --test explorer_terminal_more_e2e` | pass (mixed focus + close/flood/CJK baselines for `EXP-04R`, `TERM-05R`..`TERM-07R`) |
| `cargo test -p kjxlkj-test-harness --test explorer_terminal_stress_e2e` | pass (`EXP-05R`, `EXP-06R`, `BD-RACE-01`) |
| `cargo test -p kjxlkj-test-harness --test key_mode_e2e` | pass (`KEY-TRACE-01`, `KEY-TRACE-03`, `KEY-TRACE-04`, `WR-01R`) |
| `cargo test -p kjxlkj-test-harness --test cursor_wrap_e2e` | pass (`WRAP-11R`..`WRAP-13R`, `CUR-07R`..`CUR-11R`) |
| `cargo test -p kjxlkj-test-harness --test cursor_wrap_more_e2e` | pass (`WRAP-14R`..`WRAP-16R`) |
| `cargo test -p kjxlkj-render` | pass (render diagnostics regression coverage including wide-span cursor assertions) |
| runtime final trace bundle | includes bounded `recent_events` plus `cursor_span` diagnostics for failure triage |
| PTY harness API contract surface | implemented (`spawn`, `send raw`, `send symbolic`, `wait pattern`, `capture frame`, `resize`, `quit`) |
| source files over 200 lines | none (`find src -type f -name '*.rs' ...` top file is 200 lines) |

## Improvement Ideas

| ID | Idea | Rationale | Target |
|---|---|---|---|
| `IDEA-BASELINE-LOG-01` | persist per-wave evidence rows under reconstruction audits | keeps closure evidence local to active wave | current wave |
| `IDEA-BASELINE-LOG-02` | extend topology audit with per-directory trend reporting over time | provides early warning before fan-out pressure exceeds policy | phase-5 hardening |
| `IDEA-KEY-TRACE-01` | capture last 20 raw events and resolved actions in PTY failure diagnostics | aligns with blocker diagnostic contract and speeds root-cause analysis | phase-1 follow-up |
| `IDEA-WIN-RUNTIME-01` | wire `Ctrl-w` prefix through runtime to `WindowTree` and emit focus trace ids | enables live `WIN*R` and `WINNAV*R` closure path | phase-2 focus |
| `IDEA-WIN-RUNTIME-02` | add mixed buffer/explorer/terminal leaf binding in runtime setup and PTY scripts | required to close `WIN-03R` and `WINNAV-05R` semantics | phase-2 focus |
| `IDEA-WIN-SESSION-01` | persist window session dumps as a compact deterministic codec with version tags | enables stable roundtrip checks and future migration testing | phase-2 follow-up |
| `IDEA-WIN-TEST-DSL-01` | add a compact PTY script DSL helper shared by all `WIN*R` tests | reduces duplicate byte-script builders and improves diagnostics | phase-3 hardening |
| `IDEA-EXPTERM-ROUTES-01` | extract command and leader routing state machine from app loop into dedicated module with timeout-aware prefixes | needed for full `<leader>t` ambiguity handling and richer command coverage | phase-2 follow-up |
| `IDEA-TERM-LIFECYCLE-01` | introduce explicit terminal-child lifecycle model and reaping assertions in harness | needed to convert current terminal baselines into full lifecycle conformance evidence | phase-3 focus |
| `IDEA-CURSOR-WRAP-01` | include layout-tree summary and top-frame excerpt in every failing live diagnostic line | closes remaining medium trace gap and speeds failure triage |
| `IDEA-TRACE-RAW-01` | append bounded raw-input bytes alongside normalized key/action history | satisfies full failure-diagnostic contract from `testing-e2e` matrix |
