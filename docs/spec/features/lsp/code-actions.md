# Code Actions

LSP code actions for quick fixes and refactoring.

## Overview

Code actions provide context-aware suggestions for
fixing errors, refactoring, or improving code.

## Triggering

### Keybindings

| Key | Action |
|-----|--------|
| `<leader>ca` | Code actions |
| `<leader>.` | Quick action |

### Command


## Display

### Menu


### Lightbulb


Shows when actions available.

## Types of Actions

### Quick Fix

- Import missing items
- Fix typos
- Add missing fields

### Refactor

- Extract function/method
- Rename
- Inline variable

### Source Actions

- Organize imports
- Format file
- Generate documentation

## Configuration


## Lightbulb

### Appearance


### Position


## Navigation

### Within Menu

| Key | Action |
|-----|--------|
| `j` | Next action |
| `k` | Previous action |
| `<CR>` | Apply action |
| `1-9` | Apply numbered action |
| `<Esc>` | Cancel |

## Visual Mode

### On Selection

Code actions apply to selection:


## Preferred Actions

### Auto-Apply


### Keybinding


## LSP Requirements

### Server Support

All major LSP servers support code actions.

### Capability


## Common Actions

### Rust

- Import item
- Fill match arms
- Convert to pattern
- Extract variable

### TypeScript

- Import module
- Add missing property
- Convert to async
- Generate getter/setter

## Filtering

### By Kind


## Tips

1. Check lightbulb for suggestions
2. Use on errors for quick fixes
3. Select code before refactoring
4. Try preferred action first

## Keybindings

