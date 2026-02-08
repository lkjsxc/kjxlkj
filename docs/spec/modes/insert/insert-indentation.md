# Insert Mode Indentation

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Indentation controls available in insert mode.

## Overview

Insert mode provides keys for adjusting indentation of the current line. These work alongside auto-indent, smart-indent, and file-type-specific indentation rules.

## Manual Indentation

| Key | Effect |
|---|---|
| `<C-t>` | Increase indent by one `shiftwidth` |
| `<C-d>` | Decrease indent by one `shiftwidth` |
| `0<C-d>` | Remove all indentation from current line |
| `^<C-d>` | Remove indentation temporarily (restored on next line) |

## Auto-Indent

When `autoindent` is enabled, pressing `<CR>` copies the indentation of the current line to the new line.

| Setting | Default | Description |
|---|---|---|
| `autoindent` | `true` | Copy indent from current line to new line |

## Smart Indent

When `smartindent` is enabled, the editor adjusts indentation based on C-like syntax.

| Trigger | Action |
|---|---|
| `{` at end of line | Increase indent on next line |
| `}` at start of line | Decrease indent of current line |
| After keyword (`if`, `else`, `for`, `while`, `do`) | Increase indent on next line |

## Indent Settings

| Setting | Default | Description |
|---|---|---|
| `tabstop` | `8` | Display width of a tab character |
| `shiftwidth` | `4` | Number of spaces per indent level |
| `expandtab` | `true` | Use spaces instead of tabs |
| `softtabstop` | `4` | Number of spaces `<Tab>` inserts |

## Tab Key Behavior

| Scenario | `<Tab>` inserts |
|---|---|
| `expandtab` off | Literal tab character |
| `expandtab` on | Spaces to next `softtabstop` boundary |
| `<C-v><Tab>` | Always inserts literal tab |

## Indentation Commands from Normal Mode

| Command | Effect |
|---|---|
| `>>` | Indent line by `shiftwidth` |
| `<<` | Unindent line by `shiftwidth` |
| `={motion}` | Re-indent region using indent expression |

## Entering Insert with Indentation

| Key | Effect |
|---|---|
| `o` | Open line below with auto-indent |
| `O` | Open line above with auto-indent |
| `S` | Delete line content and auto-indent |
| `cc` | Change entire line with auto-indent |

## Auto-Indent Triggers

After `{` followed by `<CR>`, the new line is indented by one `shiftwidth`. Typing `}` on a blank line auto-unindents to match the `{`.

## File Type Settings

Per-language indentation is configured in filetype settings:

| Language | `tabstop` | `shiftwidth` | `expandtab` |
|---|---|---|---|
| Rust | 4 | 4 | true |
| Go | 4 | 4 | false |
| Python | 4 | 4 | true |
| JavaScript | 2 | 2 | true |

## Paste Indentation

When `paste` mode is active, auto-indent and smart-indent are disabled to allow pasting pre-formatted text without indentation adjustments.

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
