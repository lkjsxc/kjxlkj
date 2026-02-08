# Merge Conflict Resolution

Resolve git merge conflicts.

## Overview

Built-in tools for resolving git merge conflicts efficiently.
Conflict markers are auto-detected, highlighted, and
navigable with dedicated keybindings.

## Conflict Detection

### Automatic Detection

When a file is opened that contains conflict markers
(`<<<<<<<`, `=======`, `>>>>>>>`), the editor enables
conflict mode automatically. A warning appears in the
statusline: `[CONFLICT]`.

### Manual Check

`:ConflictCheck` scans the current buffer for markers.
Returns the count of unresolved conflicts.

## Conflict Markers

### Standard Format

A two-way conflict has three marker lines:
- `<<<<<<< {branch}` - start of current changes
- `=======` - separator
- `>>>>>>> {branch}` - end of incoming changes

### Three-Way

Three-way conflicts (with `diff3` enabled) add a base section:
- `<<<<<<< {ours}`
- `||||||| {base}`
- `=======`
- `>>>>>>> {theirs}`

## Highlighting

### Conflict Colors

| Region | Highlight Group | Default |
|--------|----------------|---------|
| Ours (current) | `ConflictCurrent` | Green background |
| Base | `ConflictAncestor` | Blue background |
| Separator | `ConflictMarker` | Bold red |
| Theirs (incoming) | `ConflictIncoming` | Purple background |

## Navigation

### Jump to Conflicts

| Key | Action |
|-----|--------|
| `]x` | Jump to next conflict marker |
| `[x` | Jump to previous conflict marker |

### Commands

`:ConflictNext` and `:ConflictPrev` are command equivalents.
`:ConflictFirst` jumps to the first conflict in the file.

## Resolution Commands

### Choose Version

| Command | Effect |
|---------|--------|
| `:ConflictChooseOurs` | Keep current (ours) version |
| `:ConflictChooseTheirs` | Keep incoming (theirs) version |
| `:ConflictChooseBoth` | Keep both versions (concatenated) |
| `:ConflictChooseBase` | Keep base version (three-way only) |
| `:ConflictChooseNone` | Delete both versions |

### Keybindings

| Key | Action |
|-----|--------|
| `<Leader>co` | Choose ours |
| `<Leader>ct` | Choose theirs |
| `<Leader>cb` | Choose both |
| `<Leader>cn` | Choose none |

## Visual Resolution

### Select and Keep

1. Enter visual mode in the conflict region
2. Select the lines to keep
3. `:ConflictKeep` removes markers and non-selected content

### Manual Edit

Edit the conflict section directly. Remove all marker lines
when done, then `:ConflictResolved` marks the conflict
as resolved (removes highlighting for that section).

## Three-Way View

### Open Three-Way

`:ConflictThreeWay` opens the file in a three-way diff layout
with LOCAL, BASE, and REMOTE in separate windows.

### Layout

Three vertical splits with synchronized scrolling.
The merge result buffer is shown below.

### Navigate Panes

Use `<C-w>h/l` to navigate between the three views.

## Diff Integration

### Compare Versions

`:ConflictDiff` opens a diff between the ours and theirs
versions of the conflict at the cursor.

### Side-by-Side

`:ConflictSideBySide` displays ours and theirs in two
splits with the conflict markers removed.

## Inline Actions

### Virtual Text

When the cursor is on a conflict, virtual text hints
appear showing available actions (e.g. "Accept Current |
Accept Incoming | Accept Both").

### Key Actions

Mouse input is ignored; conflict actions MUST be keyboard-driven.
All conflict resolution commands work from normal mode.

## Conflict List

### Show All Conflicts

`:ConflictList` shows all conflicts in the current file
in a quickfix-like list with line numbers and context.

### Quickfix Integration

`:ConflictList!` populates the quickfix list with all
conflicts across all open buffers.

## Merge Tool

`:MergeTool` opens the editor as a git mergetool,
reading `LOCAL`, `BASE`, `REMOTE`, and `MERGED` from
environment variables set by `git mergetool`.
