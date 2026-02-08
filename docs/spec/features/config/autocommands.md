# Autocommands

Automatic command execution on events.

## Overview

Autocommands register handlers that run when specific editor events occur. They are defined in TOML configuration and are the primary mechanism for event-driven behavior.

## Events (normative)

All supported autocommand event names.

### Buffer Events

| Event | Trigger |
|---|---|
| `BufNew` | New buffer created |
| `BufRead` | File read into buffer |
| `BufWrite` | Before writing buffer to file |
| `BufWritePost` | After writing buffer to file |
| `BufEnter` | Cursor enters buffer |
| `BufLeave` | Cursor leaves buffer |
| `BufDelete` | Buffer removed |

### File Events

| Event | Trigger |
|---|---|
| `FileType` | Filetype detected/changed |
| `FileReadPre` | Before reading a file |
| `FileReadPost` | After reading a file |
| `FileWritePre` | Before writing a file |
| `FileWritePost` | After writing a file |

### Window Events

| Event | Trigger |
|---|---|
| `WinEnter` | Cursor enters window |
| `WinLeave` | Cursor leaves window |
| `WinNew` | New window created |
| `WinClosed` | Window closed |
| `WinResized` | Window dimensions changed |

### Editor Events

| Event | Trigger |
|---|---|
| `VimEnter` | Editor finished startup |
| `VimLeave` | Editor about to exit |
| `InsertEnter` | Entered insert mode |
| `InsertLeave` | Left insert mode |
| `CmdlineEnter` | Entered command line |
| `CmdlineLeave` | Left command line |

### Text Events

| Event | Trigger |
|---|---|
| `TextChanged` | Text changed in normal mode |
| `TextChangedI` | Text changed in insert mode |
| `CursorMoved` | Cursor moved in normal mode |
| `CursorMovedI` | Cursor moved in insert mode |

## Pattern Matching

Autocommands can filter by file pattern (glob):

- `*.rs` matches all Rust files
- `Makefile` matches exactly `Makefile`
- `*.{js,ts}` matches JavaScript and TypeScript files

## Groups

Autocommands are organized into groups. Clearing a group removes all its handlers, preventing duplicates on config reload. Groups are declared with a name and their handlers are listed inside.

## Common Patterns

Frequently used autocommand patterns.

### Format on Save

Register `BufWrite` handler that runs the formatter on the buffer.

### Strip Trailing Whitespace

Register `BufWrite` handler that removes trailing whitespace from all lines.

### Auto-Reload

Register `BufEnter` handler that checks file modification time and reloads if changed externally.

### Remember Cursor Position

Register `BufRead` handler that restores cursor to the last known position.

## Nested Events

A handler can trigger further events. To allow this, the handler must be marked `nested = true`.

## Once-Only

Handlers marked `once = true` fire once and then auto-remove themselves.

## Priority

Handlers within the same group execute in definition order. Cross-group order follows group definition order.

## Related

- Event automation: [/docs/spec/scripting/event-automation.md](/docs/spec/scripting/event-automation.md)
- Hooks events: [/docs/spec/features/config/hooks-events.md](/docs/spec/features/config/hooks-events.md)
