# Mark Commands

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Commands for setting, deleting, and listing marks.

## Overview

Marks are set with `m{x}`, jumped to with `` `{x} `` or `'{x}`, and managed with ex commands.

## Set Mark

| Command | Description |
|---|---|
| `m{a-z}` | Set local mark (buffer-specific) |
| `m{A-Z}` | Set global mark (cross-buffer) |

## Jump to Mark

| Command | Description |
|---|---|
| `` `{mark} `` | Jump to exact position (line + column) |
| `'{mark}` | Jump to first non-blank on mark's line |

## Delete Marks

| Command | Description |
|---|---|
| `:delmarks {marks}` | Delete specified marks (e.g., `:delmarks abc`) |
| `:delmarks!` | Delete all lowercase marks for current buffer |
| `:delmarks a-c` | Delete marks a, b, c |

## List Marks

`:marks` — display all marks with their positions.

`:marks {marks}` — display only specified marks.

## Related

- Marks overview: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Mark types: [/docs/spec/editing/marks/mark-types.md](/docs/spec/editing/marks/mark-types.md)
- Special marks: [/docs/spec/editing/marks/special-marks.md](/docs/spec/editing/marks/special-marks.md)
