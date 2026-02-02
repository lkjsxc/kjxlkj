# Workspace Symbols

Search symbols across entire workspace.

## Overview

Workspace symbols search finds functions, types,
and other symbols across all project files.

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `<leader>ws` | Workspace symbols |
| `<leader>fs` | Find symbol |

### Command


## Search

### Interactive


### Fuzzy Matching

Matches partial strings:


## Display

### Format


### Information

- Symbol name
- Symbol kind (icon)
- File path
- Line number

## Navigation

| Key | Action |
|-----|--------|
| `<CR>` | Jump to symbol |
| `<C-v>` | Open in vsplit |
| `<C-x>` | Open in split |
| `<C-t>` | Open in new tab |
| `<Esc>` | Cancel |

## Filtering

### By Kind


### Kind Shortcuts

| Shortcut | Kind |
|----------|------|
| `#f` | Function |
| `#c` | Class |
| `#m` | Method |
| `#t` | Type |
| `#v` | Variable |
| `#e` | Enum |

## Configuration


## Performance

### Indexing


### Caching

Symbol index cached and updated incrementally.

## LSP Integration

### Requirements

Requires LSP server with workspace symbol support.

### Servers

| Server | Support |
|--------|---------|
| rust-analyzer | ✓ |
| typescript | ✓ |
| gopls | ✓ |
| clangd | ✓ |

## vs Document Symbols

| Feature | Document | Workspace |
|---------|----------|-----------|
| Scope | Current file | All files |
| Speed | Fast | Indexed |
| Use | Navigation | Discovery |

## Commands


## Tips

1. Use for finding definitions
2. Filter by kind for precision
3. Type more for fewer results
4. Use splits for comparison

## Keybindings

