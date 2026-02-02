# Tags Support

Navigate code using tags files.

## Overview

Tags provide symbol-based navigation using ctags
or LSP-generated tag information.

## Generating Tags

### ctags


### Configuration


## Navigation

### Go to Definition

| Key | Action |
|-----|--------|
| `<C-]>` | Go to tag |
| `<C-t>` | Pop tag stack |
| `g]` | Show all matches |

### Commands


## Tag Stack

### View


### Display


### Navigate


## Multiple Matches

### When Multiple Tags


### Display


### Select

Type number or use `g]` interactively.

## Configuration


## LSP Integration

### Prefer LSP


### Fallback

Use tags when LSP unavailable.

## Tag Kinds

### Common Kinds

| Kind | Description |
|------|-------------|
| `f` | Function |
| `c` | Class |
| `m` | Method |
| `v` | Variable |
| `t` | Type |

### Filter by Kind


## Tag Commands


## Tips

1. Generate tags for large projects
2. Use LSP when available
3. Check stack with `:tags`
4. Use `g]` for disambiguation

## Integration

### With FZF/Finder


### With Quickfix

