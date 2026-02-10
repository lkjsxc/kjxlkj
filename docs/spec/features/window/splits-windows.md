# Split and Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

The split system manages all panes, including buffers, explorer, and terminal
windows.

## Unified Window Graph

| Requirement | Detail |
|---|---|
| Single layout tree | Every tiled window node belongs to one shared tree |
| Stable identity | Each window keeps a stable `WindowId` |
| Deterministic focus | Exactly one focused window at all times |
| Geometry ownership | Core computes pane geometry; renderer consumes snapshots |

## Window Types in Splits

| Window Type | Required Behavior |
|---|---|
| Buffer window | Full editing behavior and independent viewport |
| Explorer window | Tree navigation pane that supports open/split commands |
| Terminal window | PTY-backed pane with process lifecycle hooks |

## Split Creation

| Command/Key | Required Behavior |
|---|---|
| `:split`, `Ctrl-w s` | Horizontal split |
| `:vsplit`, `Ctrl-w v` | Vertical split |
| `:new`, `Ctrl-w n` | New empty buffer split |
| `:split {path}` | Split and open file |
| `:vsplit {path}` | Vertical split and open file |

## Focus Navigation

| Key | Required Behavior |
|---|---|
| `Ctrl-w h/j/k/l` | Focus geometric neighbor by direction |
| `Ctrl-w w` / `Ctrl-w W` | Cycle next/previous window |
| `Ctrl-w p` | Return to previous focused window |
| `Ctrl-w t` / `Ctrl-w b` | Focus top-left / bottom-right window |

## Resize and Rearrangement

| Key/Command | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | Relative resize |
| `{n}Ctrl-w _` | Set/maximize height |
| `{n}Ctrl-w |` | Set/maximize width |
| `Ctrl-w =` | Equalize sibling sizes |
| `:resize`, `:vertical resize` | Command-driven resize |
| `Ctrl-w H/J/K/L` | Move current window to edge |
| `Ctrl-w r/R` | Rotate siblings |
| `Ctrl-w x` | Exchange with sibling |

## Invariants

- No overlap among tiled panes.
- Full editor area coverage by panes and separators.
- Minimum pane size enforcement.
- Focus remains valid after every create/close/move/resize operation.
- Closing terminal panes triggers PTY cleanup.

## Mandatory Verification

| ID | Scenario |
|---|---|
| WIN-01 | Multi-split create/close/only lifecycle |
| WIN-02 | Directional focus correctness on non-trivial tree |
| WIN-03 | Mixed buffer/explorer/terminal focus with `Ctrl-w` |
| WIN-04 | Resize storm keeps layout integrity |
| WIN-05 | Session roundtrip restores layout and focused window |

## Related

- Editor windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Terminal windows: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Explorer windows: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
