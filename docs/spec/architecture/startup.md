# Startup Sequence

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

This document defines initialization and shutdown order for the `kjxlkj` binary.

## Entry Point

| Step | Action | Failure Behavior |
|---|---|---|
| 1 | Parse CLI arguments | Print usage and exit non-zero |
| 2 | Install panic handler | Panic on unrecoverable handler setup failure |
| 3 | Build Tokio runtime | Panic on runtime build failure |
| 4 | Enter async `run()` | Propagate fatal error to process exit |

## Async Initialization (`run`)

| Step | Action |
|---|---|
| 1 | Detect terminal capabilities |
| 2 | Initialize editor state and defaults |
| 3 | Open CLI files or create initial scratch buffer |
| 4 | Create bounded channels (input action/key, service response, per-service request, snapshot, quit) |
| 5 | Enter raw mode and alternate screen; enable bracketed paste and focus reporting |
| 6 | Start signal watchers (`SIGWINCH`, termination signals) |
| 7 | Spawn render task |
| 8 | Spawn service supervisor and all service tasks |
| 9 | Spawn input task |
| 10 | Publish initial snapshot and perform initial frame render |
| 11 | Enter core select loop |

## Core Select Loop Requirements

Core loop MUST process:

- decoded actions
- decoded keys
- service responses
- quit conditions

After each state mutation, core MUST publish a new snapshot.

## Shutdown Sequence

| Step | Action |
|---|---|
| 1 | Set quit flag and broadcast quit signal |
| 2 | Wait for input/render/services with bounded timeout |
| 3 | Restore terminal (show cursor, leave alternate screen, disable bracketed paste and focus reporting, disable raw mode) |
| 4 | Exit process |

## Signal Handling

| Signal | Behavior |
|---|---|
| `SIGWINCH` | Send `Action::Resize(cols, rows)` |
| `SIGTERM` | Graceful shutdown |
| `SIGHUP` | Graceful shutdown |
| `SIGINT` | Graceful shutdown |
| `SIGCONT` | Reinitialize terminal state and request full redraw |

## Panic Handling

On panic, the binary MUST:

1. restore terminal state
2. print panic information to stderr
3. write crash diagnostics if crash-reporting is enabled
4. exit non-zero

## Related

- Runtime topology: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Crash reporting guidance: [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md)
