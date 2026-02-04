# Debugging (Runbook)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Practical debugging guidance for kjxlkj development and reconstruction.

Drift rule: when debugging reveals a mismatch between docs and behavior, update the docs first (or record the gap), then adjust the implementation.
Contract reference: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

## Recommended workflow

1. Reproduce in the smallest possible setup (small file, minimal keys).
2. Convert the reproduction into a headless script (deterministic).
3. Add a regression test (or a headless/E2E check) before fixing.
4. Fix, then re-run the entire test suite.
5. Update conformance/limitations if behavior is user-visible.

Related: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) and [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Build and run (Rust)

| Goal | How (examples) | Notes |
|---|---|---|
| Build | `cargo build` | fastest iteration loop |
| Run editor | `cargo run -- <path>` | open the given file |
| Run tests | `cargo test` | run full suite |
| One test | `cargo test <name>` | filter by test name substring |
| See test output | `cargo test -- --nocapture` | useful for failing tests |

## Headless execution (recommended for repros)

The binary supports a headless mode that runs a deterministic script:

- `--headless` enables headless execution
- `--script <path>` points to a script file

This is the preferred way to:

- reproduce editor bugs without terminal state
- create E2E tests that survive refactors

See current surface: [/docs/reference/CONFORMANCE_COMMANDS_TESTING.md](/docs/reference/CONFORMANCE_COMMANDS_TESTING.md)

## Terminal “stuck state” recovery

If the editor crashes while the terminal is in raw/alternate-screen mode, your shell may look “broken”.

Recommended recovery steps (in order):

1. Press `Enter` a few times (some shells redraw after a newline).
2. Run `reset` (restores a sane terminal state on many systems).
3. Run `stty sane` (restores cooked mode when `reset` is not enough).

If this happens frequently, treat it as a bug: crash paths should attempt to restore the terminal before exit.

## Panic/backtrace debugging

Rust panics can be diagnosed via backtraces:

| Setting | Meaning |
|---|---|
| `RUST_BACKTRACE=1` | short backtrace |
| `RUST_BACKTRACE=full` | full backtrace |

If you add structured crash reporting, record it in:

- [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md)

## Profiling and performance debugging

The large-file performance spec explains where hidden O(file) work can appear:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

General guidance:

- look for loops that iterate “all lines” on every input
- prefer viewport-bounded snapshots for rendering
- avoid idle busy-loops that rebuild snapshots without new input

For profiling targets and methodology (spec): [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)


### Flamegraph


## Test Debugging

### Single Test


### With Logging


## Tips

1. Use `eprintln!` for quick debugging
2. Enable backtraces always during dev
3. Keep debug builds for iteration
4. Use release builds for profiling
