# Latency and Event Ordering

Define the ordering guarantees that prevent perceived “one-key lag” and keep typing responsive.

## Definitions

| Term | Meaning |
|------|---------|
| Input event | A decoded terminal event (key press, resize). |
| Intent | A typed request emitted by UI/mode handling for the core to apply. |
| Core update | A deterministic state transition applied by the single-writer core. |
| Snapshot | An immutable projection of editor + UI state for rendering. |
| Frame | The terminal output produced from a snapshot. |
| Coalescing | Skipping intermediate snapshots while still rendering the latest state. |

## Ordering requirements (normative)

| Rule | Requirement |
|------|-------------|
| Total order | Input events MUST be applied by the core in a single total order. |
| Snapshot-after-update | Any core update that changes user-visible state MUST eventually produce a newer snapshot. |
| Monotonic snapshots | Snapshots MUST carry a monotonic sequence value so the renderer can discard stale snapshots. |
| Last-wins rendering | The renderer MAY coalesce intermediate snapshots, but MUST render the latest snapshot available. |
| No stale frames | The renderer MUST NOT render a snapshot older than one it has already rendered. |
| Flush | A rendered frame MUST be flushed to the terminal promptly after write. |

## No “one-key lag” guarantee

The implementation MUST satisfy:

- After processing input event `N`, the next frame that reflects user-visible changes MUST correspond to snapshot `N` or newer.
- Under bursty input, frames MAY skip intermediate states, but the screen MUST converge to the latest processed input without remaining one step behind.

## Coalescing and backpressure

| Concern | Requirement |
|---------|-------------|
| Snapshot coalescing | Coalescing MUST be snapshot-sequence-aware (drop stale, keep latest). |
| Input backpressure | Input events MUST NOT be dropped; overload MUST be handled via bounded buffering and fast core processing. |
| Time budgets | The core SHOULD process input in bounded batches (size or time budget) to avoid starving rendering. |

## Performance requirements

| Area | Requirement |
|------|-------------|
| Keystroke latency | Typing SHOULD remain responsive under rapid input; rendering SHOULD avoid full-screen redraws on each key. |
| Rendering writes | Render output SHOULD be diff-based (dirty region / cell diff) to minimize terminal writes. |
| Resize storms | Rapid resizes MUST not corrupt state; viewport MUST re-clamp and keep cursor visible. |

## Acceptance criteria (Given/When/Then)

1. Given Insert mode in a large file, when 200 characters are typed quickly, then the final rendered buffer MUST contain all characters in order and the cursor MUST be visible at the insertion point.
2. Given Normal mode with `scrolloff > 0`, when holding `j` to move down for 200 lines, then the viewport MUST follow deterministically and the cursor MUST remain visible at all times.
3. Given an active render backlog, when a new snapshot arrives, then the renderer MUST discard older snapshots and render the latest one.
4. Given a terminal resize while typing, when the resize completes, then the viewport MUST clamp to the new geometry and the cursor MUST remain visible.

## Related

- Runtime model: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Contracts: [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md)
- Viewport follow rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Testing strategy: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
