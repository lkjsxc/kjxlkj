# Window Commands (wincmd)

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Normative command semantics for the `Ctrl-w` family.

## Navigation Commands

| Key | Command | Required Behavior |
|---|---|---|
| `Ctrl-w h` | `:wincmd h` | focus geometry-left leaf |
| `Ctrl-w j` | `:wincmd j` | focus geometry-below leaf |
| `Ctrl-w k` | `:wincmd k` | focus geometry-above leaf |
| `Ctrl-w l` | `:wincmd l` | focus geometry-right leaf |
| `Ctrl-w w` | `:wincmd w` | cycle to next leaf in deterministic order |
| `Ctrl-w W` | `:wincmd W` | cycle to previous leaf |
| `Ctrl-w t` | `:wincmd t` | focus top-left leaf |
| `Ctrl-w b` | `:wincmd b` | focus bottom-right leaf |
| `Ctrl-w p` | `:wincmd p` | focus previous valid leaf |

## Split and Close Commands

| Key | Command | Required Behavior |
|---|---|---|
| `Ctrl-w s` | `:split` | split horizontal from focused leaf |
| `Ctrl-w v` | `:vsplit` | split vertical from focused leaf |
| `Ctrl-w n` | `:new` | split and bind new empty buffer |
| `Ctrl-w c` | `:close` | close focused leaf with rebalance |
| `Ctrl-w q` | `:quit` | close focused leaf with quit semantics |
| `Ctrl-w o` | `:only` | close all other leaves |

## Resize and Rearrangement Commands

| Key | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | relative resize by count or default step |
| `Ctrl-w =` | equalize sibling leaf sizes |
| `Ctrl-w _` | maximize/set height |
| `Ctrl-w |` | maximize/set width |
| `Ctrl-w H/J/K/L` | move focused leaf to edge and preserve bindings |
| `Ctrl-w r/R` | rotate siblings deterministically |
| `Ctrl-w x` | exchange focused leaf with sibling target |

## Count Prefix Rules

Count prefix applies to navigation and resize commands.

| Example | Required Behavior |
|---|---|
| `3 Ctrl-w j` | move three directional steps or stop deterministically |
| `5 Ctrl-w +` | increase height by five units with clamps |

## Cross-Window-Type Rule

All commands in this file apply equally to buffer, explorer, and terminal leaves.
No command path may special-case by window type unless explicitly stated.

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WINNAV-01R` | directional focus golden trace |
| `WINNAV-02R` | cyclic and directional consistency |
| `WINNAV-03R` | previous-focus stability after churn |
| `WINNAV-04R` | `t` and `b` deterministic boundary targets |
| `WINNAV-05R` | command behavior in terminal insert/normal transitions |
| `WINNAV-06R` | replay determinism across repeated runs |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
