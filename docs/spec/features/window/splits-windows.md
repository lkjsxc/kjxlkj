# Split and Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

This document defines split, close, resize, and traversal behavior for all tiled windows.

## Core Contract

| Requirement | Detail |
|---|---|
| Single tree | buffer, explorer, and terminal leaves share one tree |
| Stable identity | `WindowId` survives resize and rebalance while leaf survives |
| Deterministic focus | exactly one focused leaf exists at all times |
| Observable state | every mutation emits `layout_summary`, `focused_window_id`, and pane rectangles |
| User-visible correctness | behavior is closed only when screen-state assertions pass |

## Split Creation Semantics

| Trigger | Required Mutation | Focus Result | Visible Result |
|---|---|---|---|
| `:split`, `Ctrl-w s` | focused leaf becomes vertical stack container | focus new leaf | two rows become visible |
| `:vsplit`, `Ctrl-w v` | focused leaf becomes horizontal row container | focus new leaf | two columns become visible |
| `:new`, `Ctrl-w n` | split + new empty buffer binding | focus new leaf | new empty buffer pane is visible |
| `:split {path}` | split + open path in new leaf | focus new leaf | opened file path visible in new pane |
| `:vsplit {path}` | vertical split + open path in new leaf | focus new leaf | opened file path visible in new pane |

The old leaf keeps its `WindowId`; the new leaf gets a fresh `WindowId`.

## Close and Rebalance

| Operation | Required Behavior |
|---|---|
| `Ctrl-w c`, `Ctrl-w q` | close focused leaf, collapse unary parents, rebalance siblings |
| `Ctrl-w o`, `:only` | keep focused leaf, close all others, preserve focused binding |
| close explorer leaf | detach explorer state without dangling focus pointers |
| close terminal leaf | perform PTY cleanup before removing leaf |

After each close operation, pane rectangles must still tile the full editor region.

## Focus and Navigation

| Key | Required Behavior |
|---|---|
| `Ctrl-w h/j/k/l` | geometry-based directional focus selection |
| `Ctrl-w w/W` | deterministic next/previous leaf cycle |
| `Ctrl-w p` | previous valid focus target or deterministic fallback |
| `Ctrl-w t/b` | deterministic top-left / bottom-right leaf |

Focus transition must always emit previous and next `WindowId` in diagnostics.

## Resize Semantics

| Trigger | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | relative resize with min-size clamp |
| `{n}Ctrl-w _`, `{n}Ctrl-w |` | set or maximize height/width |
| `Ctrl-w =` | equalize sibling dimensions |
| `:resize`, `:vertical resize` | command-driven resize with deterministic clamp |

Resize must never create zero-size panes and must preserve one focused pane.

## Failure Prevention Invariants

- same initial state + same key sequence yields byte-identical layout timeline
- no mutation may leave stale `WindowId` references in focus history
- pane rectangles must not overlap or leave uncovered gaps
- pane content binding must survive unrelated neighbor mutations

## Mandatory E2E Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `WIN-01R` | split-create-close-only lifecycle | pane count/geometry and focused pane match expected timeline |
| `WIN-02R` | directional focus on asymmetric tree | focus transitions match geometry oracle after each key |
| `WIN-03R` | mixed buffer/explorer/terminal navigation | focused type and `WindowId` sequence is deterministic |
| `WIN-04R` | resize/equalize storm | no invalid geometry and no focus loss |
| `WIN-05R` | save and restore complex split tree | restored layout and focus match pre-save state |
| `WIN-SCREEN-01` | visible pane map oracle | screen snapshot panes match computed rectangles |
| `WIN-SCREEN-02` | replay determinism | repeated run yields identical per-step dumps |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Wincmd catalog: [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- Explorer integration: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal integration: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- E2E contract: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
