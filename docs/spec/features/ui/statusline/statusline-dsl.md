# Statusline DSL

Back: [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)

Domain-specific language for composing customizable statusline segments.

## Overview

The statusline DSL provides a declarative syntax for defining what information appears in the statusline and how it is formatted. Each statusline is composed of left, center, and right sections, each containing an ordered list of components.

## Basic syntax

A statusline definition is a string of segments separated by `%=` alignment markers.

| Marker | Meaning |
|---|---|
| `%=` | Alignment separator. Content before the first `%=` is left-aligned. Between two `%=` markers is centered. After the last `%=` is right-aligned. |

Example: `%{mode} %{filename}%=%{filetype}%=%{line}:%{col}` renders mode and filename on the left, filetype centered, and position on the right.

## Variables

### File information

| Variable | Description | Example output |
|---|---|---|
| `%{filename}` | File name without path | `main.rs` |
| `%{filepath}` | Full path from workspace root | `src/main.rs` |
| `%{filetype}` | Detected file type | `rust` |
| `%{fileformat}` | Line ending format | `unix` or `dos` |
| `%{encoding}` | Character encoding | `utf-8` |
| `%{filesize}` | File size in human-readable form | `12K` |

### Buffer state

| Variable | Description | Example output |
|---|---|---|
| `%{modified}` | Modified indicator | `[+]` or empty |
| `%{readonly}` | Read-only indicator | `[RO]` or empty |
| `%{bufnr}` | Buffer number | `3` |
| `%{bufcount}` | Total open buffers | `7` |

### Position

| Variable | Description | Example output |
|---|---|---|
| `%{line}` | Current line number (1-based) | `42` |
| `%{col}` | Current display column (1-based) | `15` |
| `%{lines}` | Total lines in buffer | `1200` |
| `%{percent}` | Vertical position as percentage | `35%` |

### Editor state

| Variable | Description | Example output |
|---|---|---|
| `%{mode}` | Current mode short name | `NOR`, `INS`, `VIS` |
| `%{paste}` | Paste mode indicator | `[PASTE]` or empty |
| `%{spell}` | Spell check indicator | `[SPELL]` or empty |
| `%{recording}` | Macro recording indicator | `@q` or empty |

### Git information

| Variable | Description | Example output |
|---|---|---|
| `%{branch}` | Current git branch name | `main` |
| `%{diff_added}` | Lines added since last commit | `+12` |
| `%{diff_modified}` | Lines modified since last commit | `~3` |
| `%{diff_removed}` | Lines removed since last commit | `-5` |

### Diagnostics

| Variable | Description | Example output |
|---|---|---|
| `%{errors}` | LSP error count | `2` |
| `%{warnings}` | LSP warning count | `5` |
| `%{hints}` | LSP hint count | `1` |
| `%{info}` | LSP info count | `0` |

## Formatting

### Padding

| Syntax | Meaning |
|---|---|
| `%10{variable}` | Right-pad to 10 characters |
| `%-10{variable}` | Left-pad to 10 characters |
| `%010{variable}` | Zero-pad to 10 characters |

### Truncation

| Syntax | Meaning |
|---|---|
| `%.20{variable}` | Truncate to 20 characters maximum |
| `%<` | Mark truncation point: when the statusline is too wide, content to the right of `%<` is truncated first |

### Conditional

| Syntax | Meaning |
|---|---|
| `%{?modified: [+]}` | Show ` [+]` only when buffer is modified |
| `%{?branch: (%{branch})}` | Show branch in parentheses only when in a git repo |
| `%{?errors: E:%{errors}}` | Show error count only when errors exist |

Conditionals test whether the variable is non-empty or non-zero. The content between the colon and the closing brace is rendered only when the condition is true.

## Styling

### Colors

Colors are applied using highlight group references within the DSL string.

| Syntax | Meaning |
|---|---|
| `%#GroupName#` | Switch to highlight group `GroupName` until the next group switch or end of section |
| `%*` | Reset to the default statusline highlight group |

Example: `%#StatusMode#%{mode}%* %{filename}` renders the mode in the `StatusMode` highlight group and the filename in the default group.

### Highlight groups

| Group | Purpose |
|---|---|
| `StatusLine` | Default statusline in the active window |
| `StatusLineNC` | Statusline in inactive windows |
| `StatusMode` | Mode indicator segment |
| `StatusFile` | File information segment |
| `StatusGit` | Git branch and diff segment |
| `StatusDiag` | Diagnostics segment |

## Components

### Pre-built components

The DSL supports referencing pre-built component functions by name. Each component function returns a formatted string.

| Syntax | Meaning |
|---|---|
| `%{component:mode}` | Insert the `mode` component output |
| `%{component:git}` | Insert the `git` component output |

### Available components

| Component | Output |
|---|---|
| `mode` | Mode name with highlight group applied |
| `filename` | File name with modified/readonly indicators |
| `filepath` | Full workspace-relative path |
| `filetype` | Language name from filetype detection |
| `position` | `line:col` formatted position |
| `percent` | Scroll percentage with `Top`/`Bot`/`All` special cases |
| `git` | Branch name plus diff stats |
| `diagnostics` | Error/warning counts with icons |
| `encoding` | Character encoding name |
| `indent` | Tab width and tab/space indicator |
| `lsp` | LSP server name and status (running/stopped) |

## Separators

| Setting | Description | Default |
|---|---|---|
| `statusline.separator_left` | Separator between left-section components | ` ` (space) |
| `statusline.separator_right` | Separator between right-section components | ` ` (space) |
| `statusline.component_separator` | Separator between component groups | ` │ ` |

## Sections

The statusline is divided into three sections by `%=` markers.

| Section | Alignment | Typical content |
|---|---|---|
| Left | Left-aligned | Mode, filename, git branch |
| Center | Centered between left and right | Optional: filetype or custom text |
| Right | Right-aligned | Position, diagnostics, encoding |

When the total width exceeds the window width, the center section is collapsed first, then the right section truncates from the left, then the left section truncates from the right.

## Per-mode colors

| Mode | Highlight group | Default color |
|---|---|---|
| Normal | `StatusModeNormal` | Blue background |
| Insert | `StatusModeInsert` | Green background |
| Visual | `StatusModeVisual` | Purple background |
| Replace | `StatusModeReplace` | Red background |
| Command | `StatusModeCommand` | Yellow background |

The mode component automatically switches highlight groups based on the current mode.

## Inactive windows

Inactive window statuslines use `StatusLineNC` as the base highlight group. All component highlight groups are dimmed or replaced with their `NC` variants. The inactive statusline MAY show a simplified layout (e.g., filename only).

## Examples

### Minimal

`%{mode} %{filename}%{?modified: [+]}%=%{line}:%{col}`

Renders: `NOR main.rs [+]                    42:15`

### Full-featured

`%#StatusMode#%{mode}%* %{branch} %{filename}%{modified}%=%{filetype}%=%{component:diagnostics} %{encoding} %{line}:%{col} %{percent}`

Renders: `NOR main src/lib.rs [+]     rust     E:2 W:1 utf-8 42:15 35%`

## Related

- Statusline: [/docs/spec/features/ui/statusline/statusline.md](/docs/spec/features/ui/statusline/statusline.md)
- Statusline config: [/docs/spec/features/ui/statusline/statusline-config.md](/docs/spec/features/ui/statusline/statusline-config.md)
- Themes: [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
