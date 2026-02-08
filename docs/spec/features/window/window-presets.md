# Window Presets

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Pre-defined window layout configurations.

## Overview

Window presets allow quickly switching to commonly used window arrangements.

## Built-in Presets

| Preset | Description |
|---|---|
| `single` | Single window, close all others |
| `dual-vertical` | Two vertical splits |
| `dual-horizontal` | Two horizontal splits |
| `grid-4` | 2×2 grid |
| `main-left` | Large left window, stacked right |
| `main-right` | Large right window, stacked left |

## Commands

| Command | Description |
|---|---|
| `:LayoutPreset {name}` | Apply a window preset |
| `:LayoutSave {name}` | Save current layout as preset |
| `:LayoutDelete {name}` | Delete a saved preset |

## Configuration

Custom presets are defined in configuration files. Each preset specifies the layout tree structure.

## Related

- Window layout: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Session: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
