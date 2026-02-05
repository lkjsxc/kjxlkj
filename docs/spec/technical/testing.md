# Testing

## Strategy

| Layer | Focus |
|---|---|
| Core unit tests | Pure edits, mode transitions, undo determinism. |
| Service unit tests | Protocol parsing, ranking logic, caching. |
| Integration tests | Message ordering, cancellation, backpressure behavior. |
| Golden UI tests | Snapshot-to-frame stability for critical views. |
| Interactive PTY E2E | Real TUI path: input decode → dispatch → render, verified via persisted outputs. |

## Async correctness requirements

| Concern | Requirement |
|---|---|
| Cancellation | Tests MUST assert that cancelled requests do not mutate visible state. |
| Staleness | Tests MUST ensure stale results are discarded by version checks. |
| Backpressure | Tests MUST ensure overload is visible and does not crash. |
| Recovery | Tests MUST cover service restart and continued editing. |

## Determinism checks

Given identical input streams, the core MUST yield identical serialized edit logs and identical snapshots (ignoring wall-clock timestamps).

## End-to-End testing (normative)

### Two complementary E2E harnesses

| Harness | What it validates | When it is required |
|---|---|---|
| Headless E2E | Core state machine correctness without terminal I/O | Always (fast, deterministic) |
| PTY-driven E2E | The real interactive path including input decoding and render loop | Required for any bug that could be caused by terminal decoding, key-chord parsing, or focus/routing |

The headless harness contract and script schema are recorded in:

- [/docs/reference/CONFORMANCE_COMMANDS_TESTING.md](/docs/reference/CONFORMANCE_COMMANDS_TESTING.md)

### PTY E2E design rules

PTY E2E tests MUST:

- drive the real binary through a pseudo-terminal (not internal state injection)
- avoid screen scraping when possible; prefer verifying by writing to disk (`:w`, `:wq`) and reading the file back
- use deterministic timeouts (no unbounded waits; avoid fixed sleeps when possible)
- fix environment where needed (`TERM`, locale) to reduce terminal variance

### Required PTY E2E regressions (minimum set)

These scenarios are required because they commonly pass in headless tests while failing interactively:

| Scenario | Behavior verified | Defining spec |
|---|---|---|
| Insert newline | `Enter` inserts a newline in Insert mode and persists via `:wq` | [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md) |
| Leader chords | `<leader>e` and `<leader>t` open explorer and terminal views | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) |
| Multi-key sequences | `gg` and other prefix commands are recognized | [/docs/spec/editing/motions/motions.md](/docs/spec/editing/motions/motions.md) |
| Undo/redo | `u` undoes and `Ctrl-r` redoes without dropping input | [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md) |
| Append at EOL | `a` appends at true end-of-line (no off-by-one) | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |

## Related

- Determinism and ordering: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
