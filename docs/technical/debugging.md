# Debugging Runbook

Back: [/docs/technical/README.md](/docs/technical/README.md)

Operational guidance for debugging during reconstruction waves.

## Core Workflow

1. Reproduce with the smallest deterministic input.
2. Convert the repro to automated verification.
3. Fix behavior through real runtime paths.
4. Re-run targeted tests, then profile-appropriate full verification gate.
5. Sync conformance/limitations when user-visible behavior changed.

## Local Command Reference

| Goal | Command |
|---|---|
| Build workspace | `cargo build --workspace` |
| Fast check | `cargo check --workspace` |
| Run editor | `cargo run -- <path>` |
| Run all tests | `cargo test --workspace` |
| Run one test | `cargo test <name>` |
| Show test logs | `cargo test -- --nocapture` |
| Lint review | `cargo clippy --workspace --all-targets` |

## CI Parity Rule

Use command families consistent with the active CI profile.

Canonical CI contract:

- [/docs/reference/CI.md](/docs/reference/CI.md)

## Headless vs PTY

| Harness | Use when |
|---|---|
| Headless script execution | Repro does not depend on terminal transport/escape behavior |
| PTY-driven E2E | Repro depends on input decoding, resize/focus routing, or terminal behavior |

Prefer persisted-state assertions over fragile screen scraping.

## Crash and Terminal Recovery

If a crash leaves terminal state broken:

1. Press `Enter` once or twice.
2. Run `reset`.
3. If still broken, run `stty sane`.

Persistent recovery failures are product bugs and should receive regression tests.

## Backtrace Policy

| Setting | Effect |
|---|---|
| `RUST_BACKTRACE=1` | Compact backtrace |
| `RUST_BACKTRACE=full` | Full stack trace |

Capture failing command, file path, and exact key sequence with the trace.

## Performance Triage

When debugging latency or large-file issues:

- check for accidental O(file) work in per-keystroke/per-frame paths
- verify viewport-bounded snapshot behavior
- verify no idle busy-loop redraw

Primary specs:

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

## Documentation Hygiene

If debugging finds drift:

1. update canonical spec or limitations
2. link fix to regression coverage
3. add a short dated note under `/docs/log/` when useful
