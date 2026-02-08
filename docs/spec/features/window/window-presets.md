# Window Arrangement Presets

Pre-configured window layouts.

## Overview

Quickly apply common window arrangements with single
commands. Presets define split structure, ratios, and
buffer assignments.

## Built-in Presets

### IDE Layout

Left panel (file explorer, 20%), center (editor, 60%),
right panel (outline/symbols, 20%). Bottom panel
(terminal, 25% height) spans full width.

### Writing Layout

Single centered column, 80 characters wide, with equal
padding on left and right (achieved via empty panels).

### Debug Layout

Top-left (source, 50%), top-right (variables, 25%),
top-right-bottom (watch, 25%). Bottom (terminal, 30%).

### Split Layout

Two equal vertical splits.

### Columns Layout

Three equal vertical splits.

### Grid Layout

Four equal quadrants (2x2 grid).

## Configuration

### Define Preset

In `~/.config/kjxlkj/config.toml` under `[presets.{name}]`:
each preset has a `splits` array describing the tree of
splits.

### Split Properties

| Property | Type | Description |
|----------|------|-------------|
| `type` | string | `"vertical"` or `"horizontal"` |
| `ratio` | float | 0.0-1.0 size ratio of first child |
| `children` | array | Nested split definitions |
| `name` | string | Window identifier for buffer assignment |
| `buffer` | string | Buffer to open (path, `"terminal"`, `"explorer"`) |

## Window Assignments

### Assign Buffers

Each leaf node in the split tree can specify a `buffer`:
- File path: opens that file
- `"terminal"`: opens a terminal
- `"explorer"`: opens the file explorer
- `"empty"`: empty buffer
- `"current"`: the buffer that was active before applying

## Keybindings

### Quick Access

| Key | Preset |
|-----|--------|
| `<Leader>w1` | IDE layout |
| `<Leader>w2` | Writing layout |
| `<Leader>w3` | Split layout |
| `<Leader>w4` | Columns layout |
| `<Leader>w5` | Grid layout |

### Layout Menu

`<Leader>wl` opens a picker showing all available presets
with preview descriptions.

## Commands

### Apply Layout

`:Layout {name}` applies the named preset.

### List Layouts

`:Layout` with no argument lists available presets.

### Save Current

`:LayoutSave {name}` saves the current window arrangement
as a new preset in the config file.

### Delete Layout

`:LayoutDelete {name}` removes a user-defined preset.

## Project Layouts

### Per-Project

Presets defined in `.kjxlkj.toml` at the project root
override global presets of the same name.

### Auto-Apply

`project.auto_layout = "ide"` in `.kjxlkj.toml` applies
the named preset automatically when the project is opened.
