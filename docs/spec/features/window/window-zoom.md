# Window Zoom and Maximize

Full-screen window operations.

## Overview

Temporarily maximize a window
while preserving layout state.

## Zoom Toggle

### Basic Zoom


### Behavior

- First press: maximize current window
- Second press: restore previous layout

## Maximize Commands

### Height Only


### Width Only


### Full Maximize


## Zoom vs Only

### Zoom (`:ZoomToggle`)

- Preserves layout state
- Reversible with same command
- Other windows hidden, not closed

### Only (`:only`, `<C-w>o`)

- Closes other windows
- Not reversible
- Buffers remain in buffer list

## Configuration

### Zoom Settings


### Zoom Indicator


## Statusline Integration

### Show Zoom State


### Template


## Keybindings

### Zoom Keys


### Maximize Keys


## Zoom Events

### Autocommands


### Use Cases


## Focus Mode

### Distraction-Free


### Focus Options


## Presentation Mode

### Toggle


### Settings


## Layout Preservation

### How It Works

1. Save current layout (splits, sizes)
2. Maximize current window
3. On unzoom, restore saved layout

### State Storage


## Multiple Zooms

### Nested Zooming

Not supported. Zooming while zoomed
triggers unzoom instead.

