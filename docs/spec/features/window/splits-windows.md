# Split and Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

This document defines split, close, resize, and traversal behavior for all tiled windows.

## Unified Window Graph

| Requirement | Detail |
|---|---|
| Single tree | buffer, explorer, and terminal leaves share one tree |
| Stable identity | leaf IDs remain stable while leaf survives |
| Deterministic focus | exactly one focused leaf at all times |
| Core ownership | core computes tree and geometry; renderer is read-only |

## Split Creation

| Command/Key | Required Behavior |
|---|---|
| `:split`, `Ctrl-w s` | create horizontal split from focused leaf |
| `:vsplit`, `Ctrl-w v` | create vertical split from focused leaf |
| `:new`, `Ctrl-w n` | split and bind new empty buffer |
| `:split {path}` | split and open `{path}` in new leaf |
| `:vsplit {path}` | vertical split and open `{path}` |

Split create must focus the new leaf unless explicitly overridden.

## Close and Rebalance

| Operation | Required Behavior |
|---|---|
| `Ctrl-w c`, `Ctrl-w q` | close focused leaf and rebalance ancestors |
| unary container collapse | container with one child is replaced by child |
| `Ctrl-w o` / `:only` | close all other leaves and keep focused leaf |
| terminal close | child process cleanup must complete or produce explicit error |
| explorer close | focus graph remains valid with deterministic fallback |

## Focus Commands

| Key | Required Behavior |
|---|---|
| `Ctrl-w h/j/k/l` | geometry-based directional focus |
| `Ctrl-w w/W` | deterministic next/previous leaf cycle |
| `Ctrl-w p` | previous valid focus target |
| `Ctrl-w t/b` | deterministic top-left / bottom-right leaf |

Directional focus and fallback rules are defined in [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md).

## Resize Commands

| Key/Command | Required Behavior |
|---|---|
| `Ctrl-w + - > <` | relative resize with minimum-size clamp |
| `{n}Ctrl-w _` | set or maximize height |
| `{n}Ctrl-w |` | set or maximize width |
| `Ctrl-w =` | equalize sibling leaf sizes |
| `:resize`, `:vertical resize` | command-driven resize |

Any resize must preserve geometry invariants and visible cursor/caret targets.

## Determinism Invariants

- same initial tree + same input sequence yields same final tree and focus
- close/reopen churn does not leave stale focus pointers
- tree mutation must not lose surviving leaf content bindings

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WIN-01R` | multi-split create/close/only lifecycle |
| `WIN-02R` | directional focus correctness on asymmetric trees |
| `WIN-03R` | mixed window navigation across all window types |
| `WIN-04R` | resize storm and equalize churn |
| `WIN-05R` | session roundtrip of complex split layout |
| `WINNAV-01R`..`WINNAV-06R` | directional/cyclic/previous-focus determinism |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Wincmd catalog: [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- Explorer integration: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal integration: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
