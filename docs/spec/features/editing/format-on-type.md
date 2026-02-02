# Format on Type

Automatic formatting while typing.

## Overview

Format on type automatically formats code as you
type specific trigger characters.

## Enabling


## Trigger Characters

### Default

| Character | Effect |
|-----------|--------|
| `;` | Format statement |
| `}` | Format block |
| `\n` | Format line |

### Configuration


## Examples

### After Semicolon


### After Closing Brace


## Configuration


## LSP Integration

### Server Support

| Server | Format on Type |
|--------|----------------|
| rust-analyzer | ✓ |
| typescript | ✓ |
| gopls | ✓ |

### Fallback

Uses external formatter if LSP doesn't support.

## Per-Language


## Undo

### Single Undo

Format operation is part of the edit:
`u` undoes both character and format.

## Performance

### Debouncing

Rapid typing doesn't trigger multiple formats.

### Async

Formatting runs asynchronously.

## Disable Temporarily

### Command


### Toggle


## Tips

1. Enable for statically typed languages
2. Disable for prose/markdown
3. Combine with format on save
4. Check LSP support

## Comparison

| Feature | Format on Type | Format on Save |
|---------|----------------|----------------|
| When | While typing | On `:w` |
| Scope | Line/block | Full file |
| Interruption | Minimal | None |

## Keybindings

