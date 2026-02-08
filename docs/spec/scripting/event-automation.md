# Event-Driven Automation

React to editor events with automatic actions and callbacks.

## Overview

The event system lets users attach handlers to named events. When events fire, registered handlers execute in registration order. Event groups allow batch management of handlers.

## Event Types (normative)

### Buffer Events

| Event | Trigger |
|---|---|
| `buffer_new` | Buffer created |
| `buffer_read` | File content loaded into buffer |
| `buffer_write_pre` | Before write; handler can abort write |
| `buffer_write` | Buffer written to file |
| `buffer_write_post` | After write completes |
| `buffer_enter` | Focus moves to buffer |
| `buffer_leave` | Focus moves away from buffer |
| `buffer_delete` | Buffer removed from buffer list |
| `buffer_modified` | Content changed (any edit) |
| `buffer_unmodified` | Buffer reaches unmodified state |

### Window Events

| Event | Trigger |
|---|---|
| `window_new` | Window created |
| `window_closed` | Window closed |
| `window_enter` | Focus moves to window |
| `window_leave` | Focus leaves window |
| `window_resize` | Window dimensions changed |

### Mode Events

| Event | Trigger |
|---|---|
| `mode_changed` | Any mode transition (carries old/new mode) |
| `insert_enter` | Entered insert mode |
| `insert_leave` | Left insert mode |
| `visual_enter` | Entered visual mode |
| `visual_leave` | Left visual mode |

### Cursor Events

| Event | Trigger |
|---|---|
| `cursor_moved` | Cursor position changed (normal mode) |
| `cursor_hold` | Cursor idle for `updatetime` ms |
| `cursor_hold_insert` | Idle in insert mode for `updatetime` ms |

### File Events

| Event | Trigger |
|---|---|
| `file_type` | Filetype detected or changed |
| `file_changed` | External modification detected |
| `file_read_cmd` | Custom read command triggered |

### Application Events

| Event | Trigger |
|---|---|
| `app_enter` | Editor gained focus |
| `app_leave` | Editor lost focus |
| `app_suspend` | SIGTSTP (Ctrl+Z) |
| `app_resume` | Resumed from suspend |
| `exit_pre` | Before exit; handler can abort exit |

## Event Handlers

### Basic Handler

Register a handler with the event name, an optional file pattern (glob), and a callback action (command string or function reference).

### Multiple Handlers

Multiple handlers can be registered for the same event. They execute in registration order.

### Conditional Handler

Handlers can include conditions checked at execution time. Use event data fields (buffer filetype, filename pattern, etc.) to filter.

## Event Groups

### Group Definition

Groups collect related handlers under a name. Clearing a group removes all its handlers. Re-defining a group first clears old handlers, preventing duplicate registration on config reload.

### Clear Group

Clear all handlers in a group by name. This is the standard pattern for idempotent configuration.

## Event Data

When a handler executes, it receives context about the triggering event:

| Field | Available On | Description |
|---|---|---|
| `buffer` | Buffer/file events | Buffer ID |
| `file` | File events | File path |
| `old_mode` / `new_mode` | `mode_changed` | Transition modes |
| `match` | Pattern-filtered events | Matched pattern |

## Reactive Patterns

### Auto Format on Save

Register `buffer_write_pre` handler that runs formatter on the buffer content before write.

### Auto Reload Changed Files

Register `app_enter` and `file_changed` handlers that check file modification time and reload if changed externally.

### Remember Cursor Position

Register `buffer_read` handler that restores cursor to the position stored in marks when reopening a file.

### Highlight Yanked Text

Register handler on yank event that briefly highlights the yanked region (200ms flash).

### Auto Close Completion

Register `insert_leave` handler that dismisses the completion popup.

## Chained Events

Sequential actions: a single event handler can trigger commands that themselves fire further events. Guard against infinite loops by tracking re-entry depth.

## Related

- Autocommands config: [/docs/spec/features/config/autocommands.md](/docs/spec/features/config/autocommands.md)
- Hooks events: [/docs/spec/features/config/hooks-events.md](/docs/spec/features/config/hooks-events.md)
