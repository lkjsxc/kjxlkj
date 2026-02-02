# Command-Line History

Command and search history management.

## Overview

Stores and recalls previous
commands and searches.

## History Types

### Separate Histories

| Type       | Access   | Content        |
|------------|----------|----------------|
| Command    | `:`      | Ex commands    |
| Search     | `/` `?`  | Search patterns|
| Expression | `=`      | Expressions    |
| Input      | `@`      | User input     |
| Debug      | `>`      | Debug commands |

## Navigation

### Basic Keys

| Key      | Action           |
|----------|-----------------|
| `<Up>`   | Previous entry   |
| `<Down>` | Next entry       |
| `<C-p>`  | Previous entry   |
| `<C-n>`  | Next entry       |

### Prefix Filtering

Type text first, then arrows
match only entries with prefix.


## History Size

### Configuration


### Default

1000 entries per history type.

## Persistence

### Save Location


### Separate Files


### Disable Persistence


## History Commands

### View History


### Output


### Limit Output


## Command-Line Window

### Open History Window

| Key   | Opens           |
|-------|-----------------|
| `q:`  | Command history |
| `q/`  | Search history  |
| `q?`  | Search history  |

### From Command-Line


### Window Features

- Full editing capabilities
- Navigate with j/k
- Execute with `<CR>`
- Quit with `<C-c>` or `:q`

### Edit and Execute


## History Search

### Incremental

When typing partial command:

### Pattern Match

History window supports search:

## Re-Execute

### Last Command


### From History


## Clear History

### Commands


### Delete Single


## History Modification

### Add Entry


### Get Entry

