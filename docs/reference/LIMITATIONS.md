# Known Limitations

Current limitations and planned improvements.

## Not Yet Implemented

### LSP Features

- **Code Actions**: Partial support
- **Rename Across Files**: Single file only
- **Workspace Symbols**: Not implemented
- **Call Hierarchy**: Planned

### Git Integration

- **Interactive Rebase**: Not supported
- **Merge Conflict Resolution**: Basic only
- **Stash Operations**: Limited

### Syntax Highlighting

- **Semantic Tokens**: Planned
- **Embedded Languages**: Partial
- **Custom Queries**: Not supported

### Terminal

- **Multiple Terminals**: Single terminal
- **Terminal Splits**: Planned
- **Command History**: Basic

## Platform Specific

### Windows

- **PowerShell 7**: Required
- **Windows Terminal**: Recommended
- **ConPTY**: Required for terminal

### macOS

- **iTerm2**: Recommended
- **Kitty**: Minor rendering issues

### Linux

- **Wayland**: Clipboard via wl-copy
- **XWayland**: Works normally

## Performance Limits

### File Size

- Tested up to 100MB
- Performance degrades >50MB
- Consider splitting large files

### Line Length

- Ultra-long lines may slow rendering
- Soft wrap helps

### Buffer Count

- Tested with 100+ buffers
- Memory increases linearly

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

See [docs/TODO.md](docs/TODO.md) for roadmap.

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
