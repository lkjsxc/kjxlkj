# Debug Adapter Protocol (DAP) Integration

kjxlkj includes a built-in DAP client for debugging support.

## Overview

DAP provides a language-agnostic debugging interface, similar to LSP.

## Supported Features

### Breakpoints
- Line breakpoints (`<leader>db`)
- Conditional breakpoints
- Function breakpoints
- Exception breakpoints
- Logpoints

### Execution Control
- Continue (`<leader>dc`)
- Step Over (`<leader>dn`)
- Step Into (`<leader>di`)
- Step Out (`<leader>do`)
- Pause (`<leader>dp`)
- Terminate (`<leader>dt`)

### Inspection
- Variables panel
- Watch expressions
- Call stack
- Hover evaluation

## Keybindings

| Key | Action |
|-----|--------|
| `<leader>db` | Toggle breakpoint |
| `<leader>dB` | Conditional breakpoint |
| `<leader>dc` | Continue |
| `<leader>dn` | Step over (next) |
| `<leader>di` | Step into |
| `<leader>do` | Step out |
| `<leader>dt` | Terminate session |
| `<leader>dr` | REPL open |
| `<leader>dl` | Run last config |
| `<leader>dh` | Hover variable |

## Configuration


## Debug Adapter Setup

### Rust (codelldb)

### Node.js

### Python (debugpy)

### Go (delve)

## UI Elements

### Debug Panel
Shows when debugging:
- Variables tree
- Watch expressions
- Call stack
- Breakpoints list

### Inline Display
- Current line indicator
- Inline variable values
- Breakpoint markers in gutter

## Implementation Details

Built on async message passing:
1. DAP client connects to adapter
2. Events streamed to UI
3. User actions sent as requests
4. Responses update editor state
