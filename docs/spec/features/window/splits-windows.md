# Split and Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

The split system manages all panes, including editor buffers, terminals, and explorer windows.

## Core Model

| Requirement | Detail |
|---|---|
| Unified window tree | Buffer, terminal, and explorer windows MUST share one layout graph |
| Deterministic focus | Exactly one focused window at all times |
| Stable identity | Each window keeps stable `WindowId` across operations |
| Geometry ownership | Core computes pane geometry; renderer consumes snapshots |

## Create Splits

| Command/Key | Required Behavior |
|---|---|
| `:split`, `Ctrl-w s` | Create horizontal split |
| `:vsplit`, `Ctrl-w v` | Create vertical split |
| `:new`, `Ctrl-w n` | Create split with new empty buffer |
| `:split {path}` | Split and open path in new pane |
| `:vsplit {path}` | Vertical split and open path |

## Navigation

| Key | Required Behavior |
|---|---|
| `Ctrl-w h/j/k/l` | Move focus to geometric neighbor in direction |
| `Ctrl-w w` / `Ctrl-w W` | Cycle next/previous focus order |
| `Ctrl-w p` | Return to previously focused window |
| `Ctrl-w t` / `Ctrl-w b` | Focus top-left / bottom-right window |

## Resize and Layout

| Key/Command | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | Relative resize by row/column |
| `{n}Ctrl-w _` | Set height to `n` (or maximize when omitted) |
| `{n}Ctrl-w |` | Set width to `n` (or maximize when omitted) |
| `Ctrl-w =` | Equalize sibling windows |
| `:resize`, `:vertical resize` | Absolute/relative command-driven resize |

## Close and Transform

| Key/Command | Required Behavior |
|---|---|
| `Ctrl-w c`, `:close` | Close current window |
| `Ctrl-w o`, `:only` | Close all except current |
| `Ctrl-w H/J/K/L` | Move current window to layout edge |
| `Ctrl-w r/R` | Rotate sibling windows |
| `Ctrl-w x` | Exchange current with sibling |
| `Ctrl-w T` | Move current window to new tab |

## Non-Buffer Window Behavior

| Window Type | Required Behavior |
|---|---|
| Terminal window | Keeps process lifecycle semantics while supporting all split/navigation commands |
| Explorer window | Behaves as pane in layout graph; opening file may replace or spawn new target pane |
| Floating window | May overlap but still participates in focus and close semantics |

## Invariants

- No overlap among tiled panes.
- Full editor area coverage by panes plus separators.
- Minimum pane size enforcement.
- Focus transitions MUST remain valid after every create/close/move/resize operation.

## Required Verification

| ID | Scenario |
|---|---|
| WIN-01 | Multi-split create/close/only lifecycle |
| WIN-02 | Directional neighbor focus correctness on non-trivial layout graph |
| WIN-03 | Mixed buffer+terminal+explorer navigation with `Ctrl-w` family |
| WIN-04 | Resize storm with layout integrity maintained |
| WIN-05 | Session roundtrip restores window graph shape and focused window |

## Related

- Editor windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Terminal windows: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Explorer windows: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Known current gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
