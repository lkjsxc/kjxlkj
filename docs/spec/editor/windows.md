# Windows

Back: [/docs/spec/editor/README.md](/docs/spec/editor/README.md)

Windows are core-owned viewports over buffer, explorer, and terminal content.

## Requirements

- core owns one shared tiled window tree for all window types
- exactly one tiled window is focused at all times
- renderer consumes immutable snapshots and must not mutate layout
- split, close, and focus behavior is deterministic across runs

## Window Types

| Type | Content Source | Cursor Target |
|---|---|---|
| Buffer window | `BufferId` | grapheme cursor |
| Explorer window | `ExplorerStateId` | selected node row |
| Terminal window | `TerminalId` | terminal cursor |

All types participate in one `WindowId` graph and one `Ctrl-w` command model.

## State Model

| Field | Meaning |
|---|---|
| `window_id` | stable identity for tree leaf |
| `content` | `Buffer`, `Explorer`, or `Terminal` |
| `viewport` | top/left offsets and text-area geometry |
| `cursor` | content-local cursor/caret |
| `last_focus_seq` | monotonic sequence used for deterministic tie-break |

## Layout Tree Model

| Node | Meaning |
|---|---|
| `Leaf(WindowId)` | one tiled pane |
| `Horizontal(children)` | stacked top-to-bottom |
| `Vertical(children)` | arranged left-to-right |

## Geometry Invariants

| Rule | Requirement |
|---|---|
| full coverage | panes and separators cover editor tile region |
| no overlap | pane rectangles do not overlap |
| minimum area | each pane has at least `1x1` text area |
| stable identity | `WindowId` persists across resize/rebalance when leaf survives |

## Focus Resolution (normative)

Directional focus (`Ctrl-w h/j/k/l`) MUST use geometry.

1. derive rectangles for each leaf from current tree
2. select candidates strictly in requested direction
3. rank by primary-axis distance, then orthogonal overlap, then `last_focus_seq`
4. choose top-ranked candidate
5. if no candidate exists, keep focus unchanged

Cyclic focus (`Ctrl-w w/W`) MUST use deterministic depth-first leaf order.

## Mutation Semantics

| Operation | Required Behavior |
|---|---|
| split create | focused leaf becomes container with old leaf + new leaf |
| close leaf | remove leaf, collapse unary containers, rebalance ancestors |
| close last leaf | disallowed unless editor is quitting |
| close terminal leaf | trigger terminal lifecycle cleanup before leaf removal |
| close explorer leaf | detach explorer state from window focus graph cleanly |
| `:only` | close all non-pinned leaves except current focus |

## History Semantics

- `Ctrl-w p` jumps to previous valid focused window
- if previous target is gone, fallback is deterministic nearest neighbor
- focus history must never point to deleted IDs after close/reopen churn

## Resize and Reflow

On geometry change:

1. recompute all leaf rectangles
2. clamp per-window viewport offsets
3. trigger wrap reflow in affected panes
4. propagate PTY resize to terminal leaves
5. ensure focused cursor/caret remains visible

## Session Persistence

Sessions MUST persist and restore:

- tab order and active tab
- split tree structure and leaf IDs
- content bindings for each leaf
- focused leaf identity when valid

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WIN-01R` | split create/close lifecycle keeps one valid focus |
| `WIN-02R` | directional focus on nested mixed-orientation trees |
| `WIN-03R` | mixed buffer/explorer/terminal directional navigation |
| `WIN-04R` | resize storm preserves geometry invariants |
| `WIN-05R` | session restore preserves split tree and focused leaf |

## Related

- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Wincmd semantics: [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- Explorer behavior: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal behavior: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
