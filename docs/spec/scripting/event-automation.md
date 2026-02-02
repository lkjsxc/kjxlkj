# Event-Driven Automation

Event system and reactive automation.

## Overview

React to editor events with
automatic actions and callbacks.

## Event Types

### Buffer Events

| Event | Trigger |
|-------|---------|
| `buffer_new` | Buffer created |
| `buffer_read` | File read into buffer |
| `buffer_write` | Buffer written to file |
| `buffer_write_pre` | Before buffer write |
| `buffer_write_post` | After buffer write |
| `buffer_enter` | Entered buffer |
| `buffer_leave` | Left buffer |
| `buffer_delete` | Buffer deleted |
| `buffer_modified` | Content changed |
| `buffer_unmodified` | Saved (unmodified) |

### Window Events

| Event | Trigger |
|-------|---------|
| `window_new` | Window created |
| `window_closed` | Window closed |
| `window_enter` | Entered window |
| `window_leave` | Left window |
| `window_resize` | Window resized |

### Mode Events

| Event | Trigger |
|-------|---------|
| `mode_changed` | Mode transition |
| `insert_enter` | Entered insert |
| `insert_leave` | Left insert |
| `visual_enter` | Entered visual |
| `visual_leave` | Left visual |

### Cursor Events

| Event | Trigger |
|-------|---------|
| `cursor_moved` | Cursor position changed |
| `cursor_hold` | Cursor idle timeout |
| `cursor_hold_insert` | Idle in insert mode |

### File Events

| Event | Trigger |
|-------|---------|
| `file_type` | Filetype detected |
| `file_changed` | External file change |
| `file_read_cmd` | Read command file |

### Application Events

| Event | Trigger |
|-------|---------|
| `app_enter` | Editor gained focus |
| `app_leave` | Editor lost focus |
| `app_suspend` | Editor suspended |
| `app_resume` | Editor resumed |
| `exit_pre` | Before exit |

## Event Handlers

### Basic Handler


### Multiple Handlers


### Conditional Handler


## Event Groups

### Group Definition


### Clear Group


## Event Data

### Available Context


### Cursor Events


## Reactive Patterns

### Auto Format on Save


### Auto Reload Changed Files


### Remember Cursor Position


### Highlight Yanked Text


### Auto Close Completion


## Chained Events

### Sequential Actions

