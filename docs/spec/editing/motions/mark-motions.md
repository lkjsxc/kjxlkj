# Mark Motions

Navigate using marks.

## Overview

Marks save positions for later
navigation with ` or ' motions.

## Setting Marks

### Local Marks


### Global Marks


## Mark Motions

### Exact Position


Moves to exact line and column.

### Line Start


Moves to first non-blank of line.

## Comparison


## Local Marks (a-z)

### Scope

Within current buffer only.

### Example


## Global Marks (A-Z)

### Scope

Across all files. Also remembers file.

### Example


## Special Marks

### Automatic Marks

| Mark | Position |
|------|----------|
| `` ` `` | Position before last jump |
| `'` | Line of last jump |
| `"` | Last exit position (in file) |
| `[` | Start of last change |
| `]` | End of last change |
| `<` | Start of last visual |
| `>` | End of last visual |
| `.` | Position of last change |
| `^` | Last insert position |

### Usage


## Operators with Marks

### Delete to Mark


### Yank to Mark


### Change to Mark


## Visual Selection

### Select to Mark


## Jump List

### Related Commands


### Difference

Marks are explicit. Jump list is automatic.

## List Marks

### Show All Marks


### Output


## Delete Marks

### Delete Specific


## Mark Commands

### Reference

| Command | Action |
|---------|--------|
| `:marks` | List marks |
| `:delmarks` | Delete marks |
| `:mark` | Show mark info |

## Keybindings

### Default Keys

| Key | Action |
|-----|--------|
| `m{x}` | Set mark x |
| `` `{x} `` | Go to mark x |
| `'{x}` | Go to line of mark x |
| ``` `` ``` | Toggle last position |
| `''` | Toggle last line |

## Configuration

### Mark Settings

