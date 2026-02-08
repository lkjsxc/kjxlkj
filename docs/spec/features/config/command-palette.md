# Command Palette

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Searchable list of all available commands and keybindings.

## Overview

The command palette provides a fuzzy-search interface for discovering and executing commands. It lists all ex commands, user commands, and their keybindings.

## Activation

| Key | Action |
|---|---|
| `<leader>p` | Open command palette |

| Command | Description |
|---|---|
| `:CommandPalette` | Open command palette |

## Display

Each entry shows:

| Column | Content |
|---|---|
| Command name | The ex command or action name |
| Keybinding | Associated keybinding (if any) |
| Description | Brief description of what the command does |

## Search

Typing filters the list by fuzzy match against the command name and description. The matching algorithm ranks results by relevance.

## Execution

Pressing `<CR>` on a selected entry executes the command. If the command requires arguments, the command line opens pre-filled with the command name.

## Navigation

| Key | Action |
|---|---|
| `j` / `<Down>` | Move selection down |
| `k` / `<Up>` | Move selection up |
| `<CR>` | Execute selected command |
| `<Esc>` | Close palette |

## Source

The command palette aggregates commands from:

| Source | Examples |
|---|---|
| Built-in ex commands | `:w`, `:q`, `:e` |
| User-defined commands | `:MyCommand` |
| LSP commands | Code actions |
| Session commands | `:SessionSave` |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `palette.max_results` | integer | `50` | Maximum displayed results |
| `palette.show_keybindings` | boolean | `true` | Show associated keybindings |

## Related

- Which-key: [/docs/spec/features/config/which-key.md](/docs/spec/features/config/which-key.md)
- Essential commands: [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
