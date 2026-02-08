# Icons

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

File type and UI icons using Nerd Font glyphs.

## Overview

The editor displays icons for file types, git status, diagnostics, and UI elements when a Nerd Font is detected.

## File Type Icons

File type icons appear in the file explorer, buffer tabs, and finder:

| Extension | Icon | Description |
|---|---|---|
| `.rs` | 🦀 | Rust |
| `.py` | 🐍 | Python |
| `.js` | JS | JavaScript |
| `.ts` | TS | TypeScript |
| `.md` | MD | Markdown |
| (directory) | 📁 | Folder |

## Diagnostics Icons

| Level | Icon | Usage |
|---|---|---|
| Error | `✖` | Sign column, statusline |
| Warning | `⚠` | Sign column, statusline |
| Info | `ℹ` | Sign column, statusline |
| Hint | `💡` | Sign column, statusline |

## Git Status Icons

| Status | Icon |
|---|---|
| Added | `+` |
| Modified | `~` |
| Deleted | `-` |
| Renamed | `→` |
| Untracked | `?` |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `icons.enabled` | boolean | `true` | Enable icons |
| `icons.nerd_font` | boolean | `auto` | Nerd Font detection |

## Fallback

When Nerd Font is not available, the editor falls back to ASCII representations.

## Related

- File explorer: [/docs/spec/commands/file/file-exploration.md](/docs/spec/commands/file/file-exploration.md)
- Syntax: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)
