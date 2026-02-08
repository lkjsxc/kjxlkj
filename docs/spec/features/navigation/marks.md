# Navigation Marks Feature

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

Mark-based navigation features beyond the core mark system.

## Overview

This documents the navigation-layer features built on top of the core mark system: mark signs, mark pickers, and mark management commands.

## Mark Signs

Marks are shown as virtual text or sign column indicators:

| Setting | Default | Description |
|---|---|---|
| `marks.signs` | `true` | Show marks in sign column |

## Mark Picker

| Key | Command | Description |
|---|---|---|
| `<leader>fm` | `:Marks` | Open mark picker |

The picker lists all marks with their positions and allows jumping to any mark.

## Delete Marks

| Command | Description |
|---|---|
| `:delmarks {marks}` | Delete specified marks |
| `:delmarks!` | Delete all lowercase marks for current buffer |

## Mark List

`:marks` displays all active marks with their line, column, and file information.

## Related

- Marks core: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Mark types: [/docs/spec/editing/marks/mark-types.md](/docs/spec/editing/marks/mark-types.md)
- Jump list: [/docs/spec/editing/marks/jumplist.md](/docs/spec/editing/marks/jumplist.md)
