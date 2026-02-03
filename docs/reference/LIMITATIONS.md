# Known Limitations

Current limitations and planned improvements.

This repository contains an early implementation slice. The implemented surface is recorded in:

- [docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Not Yet Implemented

### Editing surface

- Most Vim/Neovim motions, text objects, and operators are not implemented.
- Search (`/`, `?`) and match navigation (`n`, `N`) are not implemented.
- Multi-buffer and multi-window management is not implemented (single buffer/window focus).

### Built-in features

- LSP is not implemented.
- Git integration is not implemented.
- Syntax highlighting and diagnostics UI are not implemented.
- Integrated terminal panes are not implemented (only `:! {cmd}` execution exists).

### Configuration

- TOML configuration, keybinding remapping, and themes are not implemented.

## Platform Specific

Platform-specific behavior has not been fully validated.

## Performance Limits

Performance characteristics (large files, long lines, many buffers) have not been benchmarked.

## Missing Vim Features

### Ex Commands

Not all ex commands supported:

| Unsupported | Alternative |
|-------------|-------------|
| `:global` | Search + visual |
| `:vglobal` | Invert search |
| `:normal` | Macros |
| `:execute` | Not needed |

### Vim Script

No Vim script support. TOML config only.

### Modelines

Not implemented for security.

## Configuration Limits

### Keybinding Limits

- No recursive keybindings
- No functions in keybindings
- Limited key combinations

### Theme Limits

- 256 color minimum
- True color recommended
- No GUI features

## Planned Improvements

See [/docs/todo/README.md](/docs/todo/README.md) for roadmap.

## Workarounds

### Large Files


### Missing LSP Feature

Use external tools via terminal.

### Clipboard Issues


## Reporting Issues

File issues at: GitHub repository

Include:
- kjxlkj version
- OS and terminal
- Minimal reproduction
- Expected vs actual behavior
