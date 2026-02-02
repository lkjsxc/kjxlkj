# Quickfix and Location Lists

kjxlkj provides quickfix and location list functionality.

## Overview

Lists for navigating through:
- Compiler errors
- Grep results
- LSP diagnostics
- Search results

## Quickfix (Global)

One quickfix list shared across windows.

### Navigation


### Open/Close


## Location List (Per-window)

Each window has its own location list.


## Commands

### Quickfix Commands

| Command | Description |
|---------|-------------|
| `:copen` | Open quickfix window |
| `:cclose` | Close quickfix window |
| `:cnext` | Next item |
| `:cprev` | Previous item |
| `:cfirst` | First item |
| `:clast` | Last item |
| `:cdo {cmd}` | Run command on each item |

### Location Commands

| Command | Description |
|---------|-------------|
| `:lopen` | Open location list |
| `:lclose` | Close location list |
| `:lnext` | Next item |
| `:lprev` | Previous item |

## Populating Lists

### From Grep


Fills quickfix with matches.

### From LSP


### From Build


## Configuration


## Quickfix Window

Navigate within quickfix:

| Key | Action |
|-----|--------|
| `j/k` | Move up/down |
| `Enter` | Jump to item |
| `o` | Open in split |
| `t` | Open in tab |
| `p` | Preview (don't close) |
| `q` | Close quickfix |

## History

Navigate quickfix history:



## Filtering

Filter quickfix items:


## Integration

### With LSP


### With Finder

Send finder results to quickfix:

Press `<C-q>` in finder to send to quickfix.

## Trouble Alternative

View diagnostics in organized list:


Shows grouped diagnostics with context.
