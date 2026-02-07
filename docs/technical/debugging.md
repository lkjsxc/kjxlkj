# Debugging Runbook

Back: [/docs/technical/README.md](/docs/technical/README.md)

Operational guidance for debugging `kjxlkj` during reconstruction and implementation waves.

## Core workflow

1. Reproduce with the smallest possible input.
2. Convert reproduction into an automated test (unit/integration/headless/PTy E2E).
3. Fix implementation.
4. Re-run target tests and then full verification.
5. Update conformance/limitations if user-visible behavior changed.

## Local command reference

| Goal | Command |
|---|---|
| Build workspace | `cargo build --workspace` |
| Fast check | `cargo check --workspace` |
| Run editor | `cargo run -- <path>` |
| Run all tests | `cargo test --workspace` |
| Run one test | `cargo test <name>` |
| Show test logs | `cargo test -- --nocapture` |
| Lint review | `cargo clippy --workspace --all-targets` |

## CI parity rules

- Local debug loops should use the same command families as CI.
- CI currently reports warnings but does not globally fail only on rustc warnings.
- Fix warnings in touched areas when practical; record accepted scaffolding warnings in docs when relevant.

Canonical CI contract:

- [/docs/reference/CI.md](/docs/reference/CI.md)

## Headless and PTY strategy

| Harness | Use when |
|---|---|
| Headless script execution | Repro does not depend on terminal transport or escape sequences |
| PTY-driven E2E | Repro depends on input decoding, resize/focus routing, or terminal behavior |

Prefer assertions on persisted output (files, serialized state) over fragile screen scraping.

## Crash and terminal recovery

If a crash leaves the terminal in a broken state:

1. Press `Enter` once or twice.
2. Run `reset`.
3. If still broken, run `stty sane`.

Persistent recovery failures are product bugs and should get regression tests.

## Backtrace policy

| Setting | Effect |
|---|---|
| `RUST_BACKTRACE=1` | compact backtrace |
| `RUST_BACKTRACE=full` | full stack trace |

Collect failing command, file path, and exact key sequence with the trace.

## Performance triage

When debugging latency or large-file issues:

- check for accidental O(file) work in per-keystroke or per-frame paths
- verify viewport-bounded snapshot behavior
- verify no idle busy-loop redraws

Primary specs:

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

## Documentation hygiene

If debugging discovers spec/implementation drift:

1. update canonical spec or limitations
2. link the fix to a regression test
3. add a short record in `/docs/log/reconstruction/` when needed
