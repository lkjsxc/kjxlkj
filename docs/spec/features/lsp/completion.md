# Auto-Completion

Intelligent code completion support.

## Overview

kjxlkj provides auto-completion from multiple
sources including LSP, buffer words, and paths.

## Triggering Completion

### Automatic


### Manual

| Key | Action |
|-----|--------|
| `<C-n>` | Next completion |
| `<C-p>` | Previous completion |
| `<C-Space>` | Trigger completion |

## Completion Sources

### LSP

Primary source for semantic completions.

### Buffer Words

Words from current and open buffers.

### File Paths

Complete file and directory paths.

### Snippets

Snippet expansions in completion menu.

## Source Priority


## Menu Display

### Appearance


### Icons

| Icon | Source |
|------|--------|
| ƒ | Function |
| □ | Struct |
| ◇ | Enum |
| ∴ | Variable |
| ⚙ | Method |

## Navigation

| Key | Action |
|-----|--------|
| `<C-n>` | Next item |
| `<C-p>` | Previous item |
| `<C-y>` | Accept |
| `<C-e>` | Cancel |
| `<Tab>` | Accept (if configured) |

## Documentation

### Preview


### Display

Side panel shows documentation for selected item.

## Fuzzy Matching

### Enabled by Default


### Example

Typing `fn` matches:
- `function`
- `format_number`
- `find_next`

## Filtering

### By Kind


## Configuration


## Keybindings

### Custom


## LSP Integration

### Completion Capabilities


## Performance

### Debouncing

Completion waits briefly after typing.

### Caching

Repeated completions use cached results.

## Tips

1. Use `<C-Space>` for manual trigger
2. Type more characters to filter
3. Use Tab for quick accept
4. Read documentation preview
