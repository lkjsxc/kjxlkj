# Notifications

User notification system.

## Overview

Notifications display messages, warnings, and errors
to the user through the command line area and optional
popup notifications.

## Message Types

Severity-based message categories.

### Information

Standard messages displayed in the command line area.
Examples: `:w` shows `"file.rs" written, 42 lines`.

### Warning

Yellow-highlighted warnings for non-critical issues.
Example: `W10: Warning: Changing a readonly file`.

### Error

Red-highlighted errors for failures.
Example: `E212: Can't open file for writing`.

## Display Locations

Where messages appear on screen.

### Command Line

The primary message area is the bottom command line.
Messages appear here and are cleared on the next
keypress or command.

### Floating Notification

Optional floating window notifications with timeout:
- Position: top-right or bottom-right (configurable)
- Duration: auto-dismiss after `notification_timeout` ms
- Stacking: multiple notifications stack vertically

## Message History

Accessing past messages.

### :messages

`:messages` shows the full message history for the
current session.

### Navigation

In the messages window, use `j`/`k` to scroll and
`q` to close.

### Clear

`:messages clear` clears the message history.

## Severity Levels

| Level | Display | Use |
|-------|---------|-----|
| `debug` | Hidden (log only) | Internal diagnostics |
| `info` | Normal text | File operations |
| `warn` | Yellow text | Non-fatal issues |
| `error` | Red text | Failures |

## Configuration

| Option | Default | Description |
|--------|---------|-------------|
| `notification_timeout` | `3000` | Auto-dismiss ms |
| `notification_position` | `"top-right"` | Popup position |
| `cmdheight` | `1` | Command line height |
| `shortmess` | `"filnxtToOF"` | Message shortening |

## shortmess Flags

| Flag | Effect |
|------|--------|
| `f` | Use `[3 lines]` instead of `3 lines, 45 bytes` |
| `i` | Use `[incomplete]` instead of full text |
| `l` | Use `999L, 888B` instead of `999 lines, 888 bytes` |
| `n` | Use `[New]` instead of `[New File]` |
| `s` | Suppress "search hit BOTTOM" message |
| `t` | Truncate messages that are too long |
| `T` | Truncate at start if needed to show the end |
| `W` | Suppress "written" message for `:w` |
| `c` | Suppress completion menu messages |

## LSP Notifications

Notifications originating from language servers.

### Progress

LSP servers send progress notifications displayed as
a spinner with label in the statusline.

### Server Messages

LSP `window/showMessage` requests are displayed as
notifications with appropriate severity level.
