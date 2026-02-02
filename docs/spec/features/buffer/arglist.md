# Argument List

Command-line file arguments.

## Overview

The arglist manages files passed on the
command line or added during editing.

## Starting with Files


Creates arglist with three files.

## Viewing Arglist

### List All


### Current Position

Current file marked with `[]`:


## Navigation

### Commands

| Command | Action |
|---------|--------|
| `:next` | Next file |
| `:prev` | Previous file |
| `:first` | First file |
| `:last` | Last file |

### With Count


### Direct


## Modifying Arglist

### Add Files


### Delete Files


### Replace Arglist


### From Pattern


## Local Arglist

### Window-Local


Creates separate arglist for window.

### Global


Uses shared arglist.

## Editing All

### Apply to All


### Examples


### With Write


## Write All


## Arglist vs Buffers

| Aspect | Arglist | Buffer List |
|--------|---------|-------------|
| Source | Explicit | Any opened |
| Order | Preserved | By open time |
| Subset | Yes | All files |

## Keybindings


## Status

### Show Position


### In Statusline


## Wildcards

### Glob Patterns

| Pattern | Matches |
|---------|---------|
| `*` | Any characters |
| `**` | Recursive |
| `?` | Single char |
| `[abc]` | Character class |

### Examples


## Arglist Files

### Save Arglist


### Load Arglist


