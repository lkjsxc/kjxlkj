# Code Lens

Actionable information displayed above code.

## Overview

Code lens shows contextual information and actions
above functions, classes, and other code elements.

## Display

### Example


## Common Code Lens

### References

Select the lens and execute it to see references.

### Implementations


### Run/Debug


## Enabling


## Configuration


## Keybindings

| Key | Action |
|-----|--------|
| `<leader>cl` | Toggle code lens |
| `<CR>` | Execute lens (when selected) |

## Navigation

### Jump to Lens

| Key | Action |
|-----|--------|
| `]l` | Next code lens |
| `[l` | Previous code lens |

### Execute

| Key | Action |
|-----|--------|
| `<CR>` | Execute selected lens |
| `<leader>cx` | Execute lens at cursor |

## LSP Integration

### Requirements

Requires LSP server with code lens support.

### Providers

| Server | Code Lens |
|--------|-----------|
| rust-analyzer | ✓ |
| clangd | ✓ |
| gopls | ✓ |
| typescript | Limited |

## Custom Code Lens

### Configuration


## Performance

### Refresh


### Lazy Loading

Lens computed only for visible code.

## Styling


## Commands


## Rust-Analyzer Specific

### Run Single Test

Select the code lens above the test and execute it.

### Run All Tests


### Debug

Select the code lens and execute it to start the debugger.

## Tips

1. Use for quick test runs
2. Check reference counts
3. Navigate implementations
4. Refresh after changes

## Keybinding Configuration
