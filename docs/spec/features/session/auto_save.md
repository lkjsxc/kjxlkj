# Auto Save (auto-save Built-in)

Native automatic saving replacing auto-save.nvim plugin.

## Overview

Built-in automatic file saving with configurable triggers and conditions.

## Save Triggers

| Trigger | Description |
|---------|-------------|
| Focus lost | Save when window loses focus |
| Buffer leave | Save when leaving buffer |
| Insert leave | Save when exiting Insert mode |
| Timer | Save after idle period |
| Text changed | Save after text changes (debounced) |

## Activation

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>as` | Toggle auto-save | Enable/disable auto-save |
| `:AutoSaveToggle` | Toggle | Command to toggle |
| `:AutoSaveStatus` | Status | Show current status |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `auto_save.enabled` | `true` | Enable auto-save |
| `auto_save.events` | `["FocusLost", "BufLeave"]` | Trigger events |
| `auto_save.debounce` | `1000` | Debounce delay (ms) |
| `auto_save.write_all` | `false` | Save all buffers |
| `auto_save.silent` | `false` | Silent saves |
| `auto_save.exclude` | `[]` | Excluded filetypes |

## Save Conditions

Auto-save only triggers when:

| Condition | Description |
|-----------|-------------|
| Buffer modified | Buffer has unsaved changes |
| Buffer writable | Buffer is not read-only |
| File exists | File is not new/unsaved |
| Not excluded | Filetype not in exclude list |
| Not in command | Not in command-line mode |

## Excluded Filetypes

Default excluded filetypes:

| Filetype | Reason |
|----------|--------|
| `gitcommit` | Commit message editing |
| `gitrebase` | Interactive rebase |
| `help` | Help documentation |
| `qf` | Quickfix window |
| `terminal` | Terminal buffers |

## Visual Feedback

| Indicator | Description |
|-----------|-------------|
| Status line | Auto-save status indicator |
| Save notification | Brief save confirmation |
| Error message | Save failure notification |

## Integration

- Works with undo tree
- Respects file watchers
- Coordinates with LSP format-on-save
- Triggers buffer-write autocommands

## Acceptance Criteria

- Saves are debounced (not excessive)
- Failed saves produce clear errors
- Never saves incomplete edits
- Respects read-only buffers
- Works with all trigger events

