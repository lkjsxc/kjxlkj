# Command-Line Editing

Back: [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

Keys for editing text on the command line.

## Overview

The command line (`:`, `/`, `?`) supports Emacs-style editing keys for cursor movement, deletion, and text insertion.

## Cursor Movement

| Key | Action |
|---|---|
| `<Left>` | Move cursor left one character |
| `<Right>` | Move cursor right one character |
| `<Home>` / `<C-b>` | Move to beginning of line |
| `<End>` / `<C-e>` | Move to end of line |
| `<S-Left>` | Move one word left |
| `<S-Right>` | Move one word right |

## Deletion

| Key | Action |
|---|---|
| `<BS>` / `<C-h>` | Delete character before cursor |
| `<Del>` | Delete character under cursor |
| `<C-w>` | Delete word before cursor |
| `<C-u>` | Delete from cursor to start of line |

## Insert Special Characters

| Key | Action |
|---|---|
| `<C-r>{reg}` | Insert contents of register |
| `<C-r><C-w>` | Insert word under cursor |
| `<C-r><C-a>` | Insert WORD under cursor |
| `<C-r><C-f>` | Insert file path under cursor |
| `<C-r>=` | Evaluate expression and insert result |
| `<C-v>{char}` | Insert literal character |

## History Navigation

| Key | Action |
|---|---|
| `<Up>` / `<C-p>` | Previous history entry (matching prefix) |
| `<Down>` / `<C-n>` | Next history entry (matching prefix) |

History entries are filtered to match the typed prefix.

## Completion

| Key | Action |
|---|---|
| `<Tab>` | Complete next match |
| `<S-Tab>` | Complete previous match |
| `<C-d>` | List all completions |

Completion applies to commands, file paths, settings, etc.

## Confirm / Cancel

| Key | Action |
|---|---|
| `<CR>` | Execute command |
| `<Esc>` / `<C-c>` | Cancel and close command line |

## Command-Line Window

`<C-f>` in command-line mode opens the command-line window, which allows editing the command using full normal-mode editing capabilities. Press `<CR>` to execute.

## Related

- Command-line mode: [/docs/spec/modes/cmdline/README.md](/docs/spec/modes/cmdline/README.md)
- Command history: [/docs/spec/commands/cmdline/cmdline-history.md](/docs/spec/commands/cmdline/cmdline-history.md)
