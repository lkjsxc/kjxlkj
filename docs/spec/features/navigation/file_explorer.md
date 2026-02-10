# File Explorer (Built-in)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

The explorer is a native project tree view managed as an editor window.

## Core Contract

| Requirement | Detail |
|---|---|
| Window identity | Explorer is a normal `WindowId` in the shared layout tree |
| Launch wiring | Explorer commands and keys route through real runtime dispatch |
| Mixed-window navigation | `Ctrl-w` navigation works across explorer, buffer, and terminal |
| Deterministic refresh | Tree updates are explicit and reproducible |

## Launch and Command Wiring

| Trigger | Required Path |
|---|---|
| `:Explorer` | command parser -> core action -> explorer window open |
| `:ExplorerClose` | command parser -> core action -> explorer window close |
| `:ExplorerReveal` | command parser -> reveal current buffer path in explorer |
| `<leader>e` | keymap -> explorer toggle action |
| `<leader>E` | keymap -> explorer reveal action |

An explorer feature MUST NOT be marked complete if these triggers do not reach
visible runtime behavior.

## Navigation and Open Targets

| Key | Behavior |
|---|---|
| `j` / `k` | Move selection down/up |
| `h` | Collapse directory or move to parent |
| `l` | Expand directory or open file |
| `Enter` / `o` | Open file in current window |
| `v` | Open file in vertical split |
| `s` | Open file in horizontal split |
| `t` | Open file in new tab |
| `q` | Close explorer window |

## File Operations

| Key | Behavior |
|---|---|
| `a` | Create file |
| `A` | Create directory |
| `r` | Rename |
| `d` | Safe delete |
| `D` | Force delete |
| `x` / `c` / `p` | Cut / copy / paste |
| `y` / `Y` / `gy` | Copy name / relative path / absolute path |

## Rendering and Overflow Rules

| Rule | Requirement |
|---|---|
| No off-screen text | Visible explorer rows MUST stay inside window bounds |
| Long node labels | Long labels MUST soft-wrap to continuation rows in the explorer window |
| Selection identity | Wrapped continuation rows are visual-only and map to one node |
| Badges | Git/diagnostic badges render without corrupting node labels |

## Data and Service Model

| Concern | Requirement |
|---|---|
| Directory traversal | Incremental, cancellable listing |
| Sorting | Directories first, then files, stable lexical order |
| Hidden/ignored toggles | Deterministic and reversible |
| FS updates | Refresh reflects external file changes |
| Large directories | Input remains responsive under high entry counts |

## Mandatory Verification

| ID | Scenario |
|---|---|
| EXP-01 | `:Explorer` opens explorer window |
| EXP-02 | `<leader>e` toggle and `<leader>E` reveal work |
| EXP-03 | Open selected file into current, horizontal, and vertical targets |
| EXP-04 | `Ctrl-w` navigation across explorer/buffer/terminal |
| EXP-05 | Long file names wrap in explorer window without off-screen overflow |
| EXP-06 | Refresh reflects external file creation/rename/delete |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Keybindings: [/docs/spec/ux/keybindings/features.md](/docs/spec/ux/keybindings/features.md)
- E2E tests: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
