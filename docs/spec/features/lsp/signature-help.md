# Signature Help

Function signature display while typing.

## Overview

Signature help shows function parameters as you
type function calls.

## Automatic Display

### Trigger

Appears automatically when typing:
- `(` - Function call
- `,` - Next argument
- `<` - Generic arguments

### Example


## Display

### Format


### Multiple Signatures


## Navigation

| Key | Action |
|-----|--------|
| `<C-j>` | Next signature |
| `<C-k>` | Previous signature |
| `<Esc>` | Dismiss |

## Configuration


## Manual Trigger

| Key | Action |
|-----|--------|
| `<C-s>` | Show signature |

### Command


## Parameter Highlighting

### Visual

Current parameter is highlighted.

### Style


## Documentation

### Parameter Docs

Shows documentation for current parameter.


## LSP Integration

### Requirements

Requires LSP server with signature help support.

### Capability


## Overloaded Functions

### Navigation

When function has multiple signatures:


Press `<C-j>`/`<C-k>` to cycle.

## Closing

### Auto-Close

Closes when:
- Cursor moves out of call
- Typing `)` after all parameters
- Pressing `<Esc>`

### Manual Close


## Position

### Configuration


### Adaptive

Adjusts based on available space.

## Tips

1. Watch parameter highlight
2. Use navigation for overloads
3. Read parameter documentation
4. Manual trigger when needed

## Keybindings

