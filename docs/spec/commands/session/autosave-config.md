# Autosave Configuration

Back: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)

Configuration for automatic file saving behavior.

## Overview

Autosave automatically writes modified buffers to disk based on configurable triggers, reducing the risk of data loss.

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `autosave.enabled` | boolean | `false` | Enable automatic saving |
| `autosave.delay_ms` | integer | `1000` | Delay after last edit before auto-saving |
| `autosave.events` | array of string | `["FocusLost", "BufLeave"]` | Events that trigger auto-save |

## Trigger events

| Event | Description |
|---|---|
| `FocusLost` | Terminal window loses focus |
| `BufLeave` | Switching to a different buffer |
| `InsertLeave` | Leaving insert mode |
| `TextChanged` | After text changes in Normal mode |
| `Timer` | After `delay_ms` milliseconds of inactivity |

## Behavior

When autosave triggers:

1. Check if the buffer is modified
2. Check if the buffer has an associated file path (skip scratch buffers)
3. Check if the buffer is not read-only
4. Write the buffer to disk (equivalent to `:w`)
5. Fire `BufWritePost` autocommand

## Debouncing

Rapid edits reset the timer. The buffer is only saved after `delay_ms` milliseconds of inactivity. This prevents excessive disk writes during active typing.

## Exclusions

| Setting | Type | Default | Description |
|---|---|---|---|
| `autosave.exclude_filetypes` | array of string | `[]` | File types to exclude from autosave |
| `autosave.exclude_buftype` | array of string | `["nofile", "terminal"]` | Buffer types to exclude |

## Notifications

| Setting | Type | Default | Description |
|---|---|---|---|
| `autosave.notify` | boolean | `false` | Show notification on auto-save |

## Commands

| Command | Description |
|---|---|
| `:AutosaveToggle` | Toggle autosave for current buffer |
| `:AutosaveStatus` | Show autosave status |

## Related

- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Hooks and events: [/docs/spec/features/config/hooks-events.md](/docs/spec/features/config/hooks-events.md)
