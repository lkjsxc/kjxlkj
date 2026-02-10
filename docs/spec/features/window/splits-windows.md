# Split and Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

The split system manages all tiled panes, including buffer, explorer, and terminal windows.

## Unified Window Graph

| Requirement | Detail |
|---|---|
| Single layout tree | every tiled leaf belongs to one shared tree |
| Stable identity | each leaf has stable `WindowId` |
| Deterministic focus | one focused leaf at all times |
| Geometry ownership | core computes rectangles; renderer consumes snapshots |

## Split Creation

| Command/Key | Required Behavior |
|---|---|
| `:split`, `Ctrl-w s` | create horizontal split |
| `:vsplit`, `Ctrl-w v` | create vertical split |
| `:new`, `Ctrl-w n` | create split with new empty buffer |
| `:split {path}` | create split and open file in new leaf |
| `:vsplit {path}` | create vertical split and open file in new leaf |

Split creation MUST place focus on the new leaf unless command explicitly requests otherwise.

## Mixed Window Participation

| Window Type | Split Requirements |
|---|---|
| Buffer | full edit viewport with independent cursor/scroll |
| Explorer | tree viewport with open-in-target commands |
| Terminal | PTY viewport with lifecycle hooks |

All three types MUST be navigable through the same `Ctrl-w` family.

## Focus Navigation

| Key | Required Behavior |
|---|---|
| `Ctrl-w h/j/k/l` | geometry-based directional focus |
| `Ctrl-w w/W` | deterministic traversal next/previous |
| `Ctrl-w p` | previous focused leaf |
| `Ctrl-w t/b` | top-left / bottom-right leaf |

Directional focus MUST use rectangle overlap and distance rules from [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md).

## Resize and Rearrangement

| Key/Command | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | relative resize with minimum-size clamp |
| `{n}Ctrl-w _` | set/maximize height |
| `{n}Ctrl-w |` | set/maximize width |
| `Ctrl-w =` | equalize sibling sizes |
| `:resize`, `:vertical resize` | command-driven resize |
| `Ctrl-w H/J/K/L` | move current window to edge |
| `Ctrl-w r/R` | rotate siblings deterministically |
| `Ctrl-w x` | exchange current leaf with sibling |

## Close and Rebalance Rules

- `Ctrl-w c` and `Ctrl-w q` close current leaf
- closing leaf rebalances nearest valid ancestor split
- closing terminal leaf MUST trigger PTY cleanup
- closing explorer leaf MUST keep focus graph valid
- after close, focus MUST resolve deterministically to a surviving leaf

## Invariants

- no overlap among tiled panes
- full editor area coverage by panes and separators
- minimum pane size enforcement
- no orphan focus after create/close/move/resize

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WIN-01R` | multi-split create/close/only lifecycle |
| `WIN-02R` | directional focus correctness on non-trivial tree |
| `WIN-03R` | mixed buffer/explorer/terminal focus and movement |
| `WIN-04R` | resize storm keeps layout integrity |
| `WIN-05R` | session roundtrip restores layout and focus |
| `WINNAV-01R` | `Ctrl-w h/j/k/l` golden trace against known geometry |
| `WINNAV-02R` | `Ctrl-w w/W/p/t/b` sequence determinism |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Explorer window: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal window: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Wincmd catalog: [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
