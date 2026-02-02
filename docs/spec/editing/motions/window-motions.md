# Window Motions

Navigate within visible screen.

## Overview

H, M, L move cursor to specific
screen positions within window.

## High (Top)

### Basic


### With Count


### First Non-Blank

Cursor moves to first non-blank
character of the line.

## Middle

### Basic


### Calculation

If window has 25 lines visible:
M goes to line 13 (center).

## Low (Bottom)

### Basic


### With Count


## Scroll Offset Effect

### With scrolloff


H and L respect scroll offset:
- H goes to line 5 from top
- L goes to line 5 from bottom

## Visual Representation


## With Operators

### Delete to Screen Position


### Yank


## Visual Mode

### Select to Position


## Jump List

### H, M, L Behavior

These do NOT add to jump list
(they're minor movements).

## Scrolloff Interaction

### Calculation


### Disable for H/L


## Comparison

### vs gg and G

| Motion | Scope |
|--------|-------|
| `gg` | File (first line) |
| `G` | File (last line) |
| `H` | Window (top visible) |
| `L` | Window (bottom visible) |

### vs zz, zt, zb

| Motion | Action |
|--------|--------|
| `zz` | Scroll to center cursor |
| `H` | Move cursor to top |
| `zt` | Scroll, cursor stays |
| `H` | Move cursor, scroll stays |

## Keybindings

### Defaults


### Custom Counts


## Configuration

### Settings


## Practical Uses

### Quick Navigation


### Reference Point

