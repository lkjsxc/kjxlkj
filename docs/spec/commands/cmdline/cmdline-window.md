# Command-Line Window

Editing commands in a full buffer.

## Overview

The command-line window allows
editing commands with full
buffer editing capabilities.

## Opening

### From Normal Mode

| Key   | Window           |
|-------|------------------|
| `q:`  | Command history  |
| `q/`  | Search history   |
| `q?`  | Search history   |

### From Command-Line


## Window Appearance

### Layout


### Position

Opens at bottom by default.

### Size


## Navigation

### Movement

Standard normal mode:

### Search


## Editing

### Modify Commands

Full editing capabilities:

### Example


## Execution

### Run Command

From normal mode:

### Multiple Execution

Edit multiple lines, then:

## Canceling

### Exit Without Execute


### Return to Command-Line


## Window Behavior

### Temporary Buffer

- Not saved to file
- Lost on close
- Special buffer type

### Buffer Options


## History Integration

### Shows History

Window displays history entries.

### Adding New

Type new command at bottom.

### Modifying History

Edits don't change actual history
until executed.

## Configuration

### Window Options


### Mappings


## Search Window

### Different History

`q/` and `q?` show search history:

### Execute Search

`<CR>` runs search and
returns to buffer.

## Expression Window

### Open

Not standard, but can use
expression history in insert.

## Switching Windows

### From Cmdwin
