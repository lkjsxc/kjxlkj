# Debug Adapter Protocol (DAP)

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

DAP integration for debugging within the editor.

## Overview

The editor supports the Debug Adapter Protocol (DAP) for source-level debugging. DAP adapters communicate over JSON-RPC to control debuggers for various languages.

## Architecture

The editor acts as a DAP client. It communicates with debug adapters (separate processes) that implement the DAP server protocol.

## Launch/Attach

| Command | Description |
|---|---|
| `:DapLaunch` | Start debugging (launch mode) |
| `:DapAttach {pid}` | Attach to running process |
| `:DapTerminate` | Stop debugging session |

## Breakpoints

| Command | Key | Description |
|---|---|---|
| `:DapToggleBreakpoint` | `<leader>db` | Toggle breakpoint at cursor |
| `:DapConditionalBreakpoint` | `<leader>dB` | Set conditional breakpoint |
| `:DapClearBreakpoints` | - | Remove all breakpoints |

Breakpoints are shown in the sign column.

## Stepping

| Command | Key | Description |
|---|---|---|
| `:DapContinue` | `<F5>` | Continue execution |
| `:DapStepOver` | `<F10>` | Step over |
| `:DapStepInto` | `<F11>` | Step into |
| `:DapStepOut` | `<S-F11>` | Step out |

## Inspection

| Command | Description |
|---|---|
| `:DapHover` | Show value under cursor |
| `:DapScopes` | Show local/global scopes |
| `:DapFrames` | Show call stack |
| `:DapRepl` | Open debug REPL |

## Configuration

Debug configurations are defined per project in the workspace manifest:

| Field | Description |
|---|---|
| `type` | Adapter type (e.g., `lldb`, `codelldb`) |
| `request` | `launch` or `attach` |
| `program` | Executable path |
| `args` | Command-line arguments |
| `cwd` | Working directory |

## Related

- Terminal: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- LSP: [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
