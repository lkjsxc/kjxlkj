# Windows

Back: [/docs/spec/editor/README.md](/docs/spec/editor/README.md)

Windows are core-owned viewports over content sources.

## Requirements

- core owns one shared window tree for buffer, explorer, and terminal windows
- renderer consumes immutable window snapshots and MUST NOT mutate layout
- exactly one tiled window is focused at any time

## Window Types

| Type | Content Source | Description |
|---|---|---|
| Buffer window | `BufferId` | text editing viewport |
| Explorer window | `ExplorerStateId` | project tree viewport |
| Terminal window | `TerminalId` | PTY-backed terminal viewport |

All types participate in the same `WindowId` graph, split operations, and `Ctrl-w` navigation.

## Window Model

| Field | Type | Description |
|---|---|---|
| `window_id` | `WindowId` | stable unique identity |
| `content` | enum | `Buffer`, `Explorer`, or `Terminal` |
| `cursor` | content-local cursor | grapheme cursor, explorer row, or terminal cursor |
| `viewport` | viewport state | top/left offset and text-area dimensions |
| `options` | window options | wrap, line numbers, scroll margins |
| `last_focus_seq` | monotonic integer | tie-breaker for focus history |

## Layout Tree

The layout is a recursive split tree of leaves and containers.

| Node Type | Meaning |
|---|---|
| `Leaf(WindowId)` | one tiled window |
| `Horizontal(children)` | children arranged top-to-bottom |
| `Vertical(children)` | children arranged left-to-right |

## Geometry Rules

| Rule | Requirement |
|---|---|
| Full coverage | tiled windows and separators fill editor grid |
| No overlap | tiled windows do not overlap |
| Minimum size | text area remains at least 1x1 |
| Stable IDs | `WindowId` survives split/resize/rebalance operations |

## Directional Focus Algorithm (normative)

For `Ctrl-w h/j/k/l`, focus resolution MUST use geometry, not cyclic order.

1. derive virtual rectangles for each leaf from current layout tree
2. collect candidate windows strictly in requested direction
3. discard candidates with zero orthogonal overlap, unless no-overlap fallback is needed
4. rank candidates by:
   - smallest primary-axis distance
   - largest orthogonal overlap
   - most recent `last_focus_seq` (tie-break)
5. select first ranked candidate

If no directional candidate exists, focus MUST remain unchanged.

## Window Navigation

| Key | Action |
|---|---|
| `Ctrl-w h/j/k/l` | focus directional neighbor by geometry |
| `Ctrl-w w/W` | cycle next/previous in deterministic traversal order |
| `Ctrl-w p` | return to previously focused window |
| `Ctrl-w t/b` | focus top-left / bottom-right leaf |

Navigation semantics are identical across buffer, explorer, and terminal windows.

## Split and Close Semantics

| Operation | Requirement |
|---|---|
| Split create | parent leaf becomes split container with two leaves |
| Close leaf | close current leaf and rebalance nearest valid ancestor |
| Close terminal leaf | close triggers PTY hangup + child reap |
| Close explorer leaf | explorer state detaches cleanly from focus graph |
| `:only` | close all non-pinned tiled leaves except current |

## Resize and Reflow

- resize updates leaf rectangles and clamps viewports
- terminal leaf resize propagates to PTY (`SIGWINCH`)
- wrapped views recompute display rows after geometry change

## Session Persistence Contract

Sessions MUST persist and restore:

- tab order and active tab
- split tree and leaf `WindowId` mappings
- content bindings for buffer/explorer/terminal leaves
- focused window identity when valid on restore

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WIN-01R` | split create/close lifecycle preserves focus uniqueness |
| `WIN-02R` | directional focus on nested mixed-orientation tree |
| `WIN-03R` | mixed buffer/explorer/terminal `Ctrl-w` navigation |
| `WIN-04R` | resize storm preserves geometry invariants and cursor visibility |
| `WIN-05R` | session roundtrip restores focused window and split structure |

## Related

- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Explorer view: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal view: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
