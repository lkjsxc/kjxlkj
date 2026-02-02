# Inlay Hints

Inline type and parameter hints.

## Overview

Inlay hints display type information and parameter
names inline within your code.

## Enabling


## Types of Hints

### Type Hints


### Parameter Hints


### Chaining Hints


## Display

### Appearance


Hints shown in different color, non-intrusive.

## Configuration


## Keybindings

| Key | Action |
|-----|--------|
| `<leader>ih` | Toggle inlay hints |

## Per-Language

### Rust


### TypeScript


## Styling


## Performance

### Delay


### Viewport Only

Only hints for visible lines are computed.

## Interactive

### Hover on Hint

Show full type if truncated.

### Double-Click

Insert hint text into code (for some hints).

## Commands


## LSP Requirements

### Server Support

Not all LSP servers support inlay hints.

| Server | Support |
|--------|---------|
| rust-analyzer | ✓ |
| typescript-language-server | ✓ |
| clangd | ✓ |
| gopls | ✓ |

## Use Cases

### Learning

See types without annotations.

### Reviewing

Understand code quickly.

### Debugging

Verify expected types.

## Tips

1. Enable for unfamiliar codebases
2. Disable for clean screenshots
3. Use with type-inferred code
4. Toggle when hints clutter view

## Keybinding Configuration

