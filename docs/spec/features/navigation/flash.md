# Flash/Leap Navigation

kjxlkj supports rapid jump navigation.

## Overview

Jump to any visible location with 2-3 keystrokes.

## Keybindings


## How It Works

1. Press `s` (flash jump)
2. Type 1-2 characters to search
3. Labels appear at matches
4. Press label to jump

Example:

## Configuration


### Label Style


## Modes

### Character Jump


Shows labels for all matches of next character.

### Word Jump


Labels all word starts.

### Line Jump


Labels all lines for quick vertical movement.

### Tree-sitter Jump


Labels tree-sitter nodes (functions, classes, etc).

## Operator Pending

Use flash with operators:

- `ds` then flash → delete to flash location
- `ys` then flash → yank to flash location

## Remote Actions

Jump and execute action:


1. Press `yr` (yank remote)
2. Flash to location
3. Press text object (e.g., `iw`)
4. Text yanked, return to origin

## Search Integration


During search, press labels to jump to matches.

## Repeat


## Cross-Window

Jump across splits:


Labels work across all visible windows.

## Performance

