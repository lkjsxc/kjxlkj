# Auto-Formatting

Automatic code formatting on save or command.

## Overview

kjxlkj integrates with external formatters for
automatic code formatting.

## Enabling Format on Save

### Configuration


### Per-Filetype


## Formatter Configuration

### Language Formatters


### Custom Commands


## Manual Formatting

### Commands


### Keybindings

| Key | Action |
|-----|--------|
| `<leader>lf` | Format buffer |
| `gq` | Format selection/motion |

## LSP Formatting

### Prefer LSP


### Fallback to External


## Range Formatting

### Visual Selection

1. Select text visually
2. Press `gq`

### Motion-Based


## Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `format_on_save` | false | Auto-format |
| `prefer_lsp` | true | Use LSP first |
| `lsp_fallback` | true | External if no LSP |
| `timeout` | 2000 | ms before cancel |

## Error Handling

### On Format Error


### Async Formatting


## Formatter Installation

### Rust


### JavaScript/TypeScript


### Python


### Go

Included with Go installation.

## Project Configuration

### .editorconfig

Respected by formatters that support it.

### Local Formatter Config

- `.rustfmt.toml`
- `.prettierrc`
- `pyproject.toml`

## Undo Integration

Format operations are a single undo unit.
Press `u` to undo entire format.

## Performance

- Only formats changed ranges when possible
- Debounced on rapid saves
- Cached formatter processes
