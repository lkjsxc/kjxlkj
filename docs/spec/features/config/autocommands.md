# Autocommands

Automatic command execution on events.

## Overview

Autocommands run actions when specific events
occur, enabling powerful automation.

## Definition


## Events

### Buffer Events

| Event | Trigger |
|-------|---------|
| `BufNew` | New buffer created |
| `BufRead` | File read into buffer |
| `BufWrite` | Before writing |
| `BufWritePost` | After writing |
| `BufEnter` | Enter buffer |
| `BufLeave` | Leave buffer |
| `BufDelete` | Buffer deleted |

### File Events

| Event | Trigger |
|-------|---------|
| `FileType` | Filetype detected |
| `FileReadPre` | Before reading |
| `FileReadPost` | After reading |
| `FileWritePre` | Before writing |
| `FileWritePost` | After writing |

### Window Events

| Event | Trigger |
|-------|---------|
| `WinEnter` | Enter window |
| `WinLeave` | Leave window |
| `WinNew` | New window |
| `WinClosed` | Window closed |
| `WinResized` | Window resized |

### Editor Events

| Event | Trigger |
|-------|---------|
| `VimEnter` | Editor started |
| `VimLeave` | Editor closing |
| `InsertEnter` | Enter insert mode |
| `InsertLeave` | Leave insert mode |
| `CmdlineEnter` | Enter command line |
| `CmdlineLeave` | Leave command line |

### Text Events

| Event | Trigger |
|-------|---------|
| `TextChanged` | Text changed |
| `TextChangedI` | Text changed (insert) |
| `CursorMoved` | Cursor moved |
| `CursorMovedI` | Cursor moved (insert) |

## Patterns

### Glob Patterns


### Regex Patterns


## Groups

### Definition


### Clearing


## Conditional


## Common Patterns

### Format on Save


### Strip Whitespace


### Auto-reload


### Cursor Line


## Nested Events


## Once-Only


## Priority

