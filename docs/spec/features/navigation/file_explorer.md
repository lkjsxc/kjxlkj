# File Explorer (Built-in)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

The explorer is a native project tree view managed as an editor window.

## Goals

- Navigate project files without leaving editor context.
- Open targets in current window, splits, or tabs.
- Keep explorer responsive on large directories.

## Explorer Window Contract

| Requirement | Detail |
|---|---|
| Window identity | Explorer MUST be represented as a regular window in the layout tree |
| Focus behavior | `Ctrl-w` navigation MUST move between explorer, buffer, and terminal windows |
| Width control | Explorer width is window-local and resizable |
| Close behavior | Closing explorer MUST not close non-explorer windows |

## Activation

| Key/Command | Required Behavior |
|---|---|
| `<leader>e` | Toggle explorer window |
| `<leader>E` | Reveal current file in explorer |
| `:Explorer` | Open explorer |
| `:ExplorerClose` | Close explorer |

## Core Navigation Keys

| Key | Behavior |
|---|---|
| `j` / `k` | Move selection down/up visible nodes |
| `h` | Collapse directory or move to parent |
| `l` | Expand directory or open file |
| `Enter` | Open selected node |
| `gg` / `G` | Jump to first/last visible node |

## Open Targets

| Key | Behavior |
|---|---|
| `o` | Open selected file in current window |
| `v` | Open in vertical split |
| `s` | Open in horizontal split |
| `t` | Open in new tab |

## File Operations

| Key | Behavior |
|---|---|
| `a` | Create file |
| `A` | Create directory |
| `r` | Rename |
| `d` | Delete (safe mode) |
| `D` | Force delete |
| `x` / `c` / `p` | Cut / copy / paste |
| `y` / `Y` / `gy` | Copy name / relative path / absolute path |

## Visual and Filter Controls

| Key | Behavior |
|---|---|
| `R` | Refresh tree |
| `H` | Toggle hidden files |
| `I` | Toggle ignored files |
| `/` | Filter by substring |
| `q` | Close explorer |

## Data and Service Model

| Concern | Requirement |
|---|---|
| Directory traversal | MUST be incremental and cancellable |
| Sort order | Directories first, files second, stable lexical order |
| Hidden files | Controlled by explorer setting and filter |
| Git badges | Async overlay from git service |
| Diagnostic badges | Async overlay from diagnostics service |

## Scalability Requirements

| Scenario | Required Behavior |
|---|---|
| 10k-entry directory | Input remains responsive during listing |
| Deep tree expansion | Expand/collapse operations are bounded and cancellable |
| Large rename/move | Buffer paths and watchers update atomically |

## Required Verification

| ID | Scenario |
|---|---|
| EXP-01 | Toggle explorer, navigate, open file in same window |
| EXP-02 | Open file from explorer into horizontal and vertical splits |
| EXP-03 | Explorer with terminal and buffer windows under `Ctrl-w` navigation |
| EXP-04 | Hidden/ignored toggles update visible set deterministically |
| EXP-05 | Refresh after filesystem change updates tree without restart |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Keybindings: [/docs/spec/ux/keybindings/navigation.md](/docs/spec/ux/keybindings/navigation.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
