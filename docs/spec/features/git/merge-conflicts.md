# Merge Conflict Resolution

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Tools for detecting and resolving merge conflicts.

## Overview

The editor detects Git merge conflict markers in files and provides navigation and resolution commands.

## Conflict Markers

The editor recognizes standard Git conflict markers:

| Marker | Meaning |
|---|---|
| `<<<<<<<` | Start of "ours" (current) changes |
| `=======` | Separator between ours and theirs |
| `>>>>>>>` | End of "theirs" (incoming) changes |
| `\|\|\|\|\|\|\|` | Optional base section (diff3 style) |

## Detection

Conflict markers are detected on file load and on buffer change events. Detected conflicts are highlighted with distinct highlight groups.

## Navigation

| Key | Command | Description |
|---|---|---|
| `]x` | `:ConflictNext` | Jump to next conflict |
| `[x` | `:ConflictPrev` | Jump to previous conflict |

## Resolution Commands

| Command | Description |
|---|---|
| `:ConflictChooseOurs` | Accept current (ours) changes, remove theirs |
| `:ConflictChooseTheirs` | Accept incoming (theirs) changes, remove ours |
| `:ConflictChooseBoth` | Keep both changes (remove markers only) |
| `:ConflictChooseNone` | Remove both changes and markers |

## Visual Indicators

| Element | Highlight Group |
|---|---|
| Ours section | `ConflictOurs` |
| Theirs section | `ConflictTheirs` |
| Base section | `ConflictBase` |
| Marker lines | `ConflictMarker` |

## Count

The statusline shows the number of remaining conflicts in the current buffer.

## Diff3

When `merge.conflictstyle = diff3` is set in Git config, the base version appears between `|||||||` and `=======`. The editor handles this three-way format.

## Related

- Git integration: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- Diff mode: [/docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)
