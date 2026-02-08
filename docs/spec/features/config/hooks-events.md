# Hooks and Events

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Internal event system for triggering actions on editor state changes.

## Overview

Hooks are callback registrations that fire when specific editor events occur. They provide the foundation for autocommands, format-on-save, and other automatic behaviors.

## Event types

Categories of events that can trigger hooks.

### Buffer events

| Event | Fires when |
|---|---|
| `BufNew` | A new buffer is created |
| `BufRead` | A file is read into a buffer |
| `BufWrite` | A buffer is about to be written to disk |
| `BufWritePost` | A buffer has been written to disk |
| `BufEnter` | A buffer becomes the active buffer in any window |
| `BufLeave` | A buffer is no longer the active buffer |
| `BufDelete` | A buffer is being deleted |
| `BufModified` | A buffer's modified flag changes |

### Window events

| Event | Fires when |
|---|---|
| `WinNew` | A new window is created |
| `WinClosed` | A window is closed |
| `WinEnter` | A window gains focus |
| `WinLeave` | A window loses focus |
| `WinResize` | A window is resized |

### Editor events

| Event | Fires when |
|---|---|
| `EditorStartup` | The editor finishes startup |
| `EditorExit` | The editor is about to exit |
| `ModeChanged` | The editor mode changes |
| `CursorMoved` | The cursor position changes (Normal mode) |
| `CursorMovedI` | The cursor position changes (Insert mode) |
| `TextChanged` | Buffer text changes (Normal mode) |
| `TextChangedI` | Buffer text changes (Insert mode) |

### File events

| Event | Fires when |
|---|---|
| `FileType` | File type is detected or changed |
| `DirChanged` | The working directory changes |

## Hook registration

Hooks are registered via autocommands (see [/docs/spec/features/config/autocommands.md](/docs/spec/features/config/autocommands.md)). Each registration specifies the event, an optional file pattern, and an action (ex command string).

## Event dispatch

Events are dispatched synchronously within the core task. The core processes all registered hooks for an event before continuing. Long-running hook actions MUST NOT block the core; they should dispatch async work to services.

## Event data

Each event carries a context table with relevant data:

| Field | Type | Description |
|---|---|---|
| `buf` | integer | Buffer ID |
| `file` | string | File path (if applicable) |
| `match` | string | File pattern that matched |
| `event` | string | Event name |

## Ordering

When multiple hooks are registered for the same event, they fire in registration order. A hook that calls `:quit` or modifies the buffer list may prevent subsequent hooks from firing if the target buffer no longer exists.

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `events.cursor_debounce_ms` | integer | `50` | Debounce for CursorMoved events |
| `events.text_debounce_ms` | integer | `100` | Debounce for TextChanged events |

## Related

- Autocommands: [/docs/spec/features/config/autocommands.md](/docs/spec/features/config/autocommands.md)
- Event automation: [/docs/spec/scripting/event-automation.md](/docs/spec/scripting/event-automation.md)
