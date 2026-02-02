# Comment Toggling

kjxlkj provides easy comment toggling.

## Overview

Toggle comments with a single keypress:


## Configuration


## Keybindings

### Line Comment


### Block Comment


### With Motion


Examples:
- `gcip` - Comment inner paragraph
- `gc3j` - Comment 3 lines down
- `gcG` - Comment to end of file

### Visual Mode


## Language Support

Auto-detects comment style per language:

| Language | Line | Block |
|----------|------|-------|
| Rust | `//` | `/* */` |
| Python | `#` | `""" """` |
| HTML | N/A | `<!-- -->` |
| CSS | N/A | `/* */` |
| Lua | `--` | `--[[ ]]` |
| SQL | `--` | `/* */` |

## Custom Comment Styles

Override for specific languages:


## Smart Comments

### Uncomment Detection

Automatically detects if lines are commented:
- All lines commented → uncomment
- Mixed → comment all
- None commented → comment all

### Padding


Without padding: `//code`
With padding: `// code`

## Block Comments

### Toggle Block

`gbc` toggles block comment on current line.

### Wrap Selection

In visual mode, `gb` wraps selection:


## Advanced

### Comment Above/Below


Adds comment line and enters insert mode.

### TODO Comments

Quick TODO insertion:


## Tree-sitter Integration

Uses tree-sitter for:
- Accurate language detection in mixed files
- Respect for string literals (don't comment inside)
- JSX/TSX component handling
