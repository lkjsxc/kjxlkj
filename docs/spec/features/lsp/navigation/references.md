# Reference Finder

Find all references to a symbol.

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `gr` | Go to references |
| `<leader>fr` | Find references |

### Command


## Display

### Reference List


## Information Shown

- File path
- Line and column
- Context line
- Total count

## Navigation

| Key | Action |
|-----|--------|
| `j` | Next reference |
| `k` | Previous reference |
| `<CR>` | Jump to reference |
| `<C-v>` | Open in vsplit |
| `<Esc>` | Close |

## Include Declaration

### Configuration


### Toggle


## Quickfix Integration

### Send to Quickfix


### Navigate


## Preview

### Inline


Shows context around reference.

## Configuration


## Filtering

### By File Pattern


### By Type


## Use Cases

### Refactoring

Find all usages before renaming.

### Understanding

See how symbol is used.

### Impact Analysis

Assess change impact.

## LSP Requirements

### Server Support

All major LSP servers support references.

## Related Commands

| Command | Description |
|---------|-------------|
| `gd` | Go to definition |
| `gi` | Go to implementation |
| `gr` | Go to references |

## Statusline

Shows reference count:


## Keybindings


## Tips

1. Check count before refactoring
2. Use quickfix for systematic review
3. Preview to understand context
4. Filter large results
