# Document Symbols

Navigate code structure within a file.

## Overview

Document symbols show the structure of the current
file for quick navigation.

## Opening

### Keybinding

| Key | Action |
|-----|--------|
| `<leader>ds` | Document symbols |
| `go` | Symbol outline (sidebar) |

### Command


## Display

### List View


### Tree View


## Symbol Types

| Icon | Type |
|------|------|
| ◇ | Class/Struct |
| ƒ | Function |
| ○ | Method |
| ⬡ | Interface |
| ▢ | Enum |
| • | Property/Field |
| △ | Variable |
| ☆ | Constant |

## Navigation

### Fuzzy Search

Type to filter symbols.


### Jump

| Key | Action |
|-----|--------|
| `<CR>` | Jump to symbol |
| `o` | Jump and close |
| `<Esc>` | Close |

## Outline Sidebar

### Toggle


### Configuration


## Configuration


## Breadcrumbs

### Statusline

Shows path to current symbol:


### Configuration


## LSP Integration

### Requirements

Requires LSP server with document symbol support.

### Hierarchical Symbols

Nested structure from language server.

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `]s` | Next symbol |
| `[s` | Previous symbol |

### Within Picker

| Key | Action |
|-----|--------|
| `j` | Down |
| `k` | Up |
| `<C-d>` | Page down |
| `<C-u>` | Page up |

## Commands


## Tips

1. Use for large files
2. Jump to functions quickly
3. Keep outline open for context
4. Use breadcrumbs for orientation
