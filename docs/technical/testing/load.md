# Load Testing

Back: [/docs/technical/testing/README.md](/docs/technical/testing/README.md)
Stress and load testing guidance for kjxlkj.

## Goals

Load tests should catch regressions that unit tests miss, especially:

- accidental O(file) work per input (vs O(viewport))
- idle CPU busy-loops (continuous redraw without input)
- rendering instability on extremely long lines
- slow large-file open and sluggish scroll/typing responsiveness
- large-directory performance for navigation UIs (file explorer)

The normative requirements live in:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

## Core scenarios

Load testing scenarios to exercise.

### Large files

Recommended datasets:

| Dataset | Shape | Why it matters |
|---|---|---|
| 1 MiB | typical source/log | baseline responsiveness |
| 10 MiB | medium logs | reveals O(N) per keypress issues |
| 100 MiB | large logs | stresses I/O + snapshot bounds |
| 1,000,000 lines | very high line count | stresses indexing and line access |

Required checks:

- time-to-first-interaction after open is reasonable (avoid “frozen after open”)
- snapshot generation remains viewport-bounded (no full-buffer materialization)
- scrolling does not trigger pauses proportional to total line count

### Extremely long lines

Recommended datasets:

| Dataset | Shape | Why it matters |
|---|---|---|
| 1 line, 1,000,000 columns (ASCII) | single huge line | stresses slicing and width handling |
| 1 line, Unicode-heavy | tabs + wide graphemes + combining marks | stresses display-width determinism |

Required checks:

- no panics and no terminal display corruption
- per-frame work is bounded to the visible slice (not full line length)

Related requirement:

- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

### Input storms

Required checks:

- typing bursts: all input applied in order; final state correct
- scroll bursts: cursor remains visible; viewport follow deterministic
- resize storms: viewport clamps and no redraw corruption

### Large directories (file explorer)

Required checks:

- expanding a directory with 10,000 children does not freeze input
- directory listing work is incremental/cancellable if needed

See:

- [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)

## What to measure

| Metric | Why | Notes |
|---|---|---|
| Open time-to-first-snapshot | user-perceived startup on file open | prefer measured from inside the app when possible |
| Snapshot generation cost | detects O(file) behavior | record “lines materialized” for strong signal |
| Render cost | detects full-screen redraw storms | measure terminal writes when feasible |
| Idle CPU | detects busy-loop rendering | measure with no input/no animation |
| Peak memory | detects intermediate copies | especially on large-file open |

## Automation (recommended)

Prefer a layered approach:

1. Deterministic headless probes where interactive terminal state is not required.
2. Interactive PTY-driven E2E where the bug class is input decoding or terminal rendering behavior.
3. Benchmarks as trend signals (avoid brittle absolute budgets in CI unless proven stable).

Record results and regressions in:

- [/docs/log/audits/README.md](/docs/log/audits/README.md)
