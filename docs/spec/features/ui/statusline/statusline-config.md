# Statusline Customization

kjxlkj's statusline is fully configurable.

## Default Layout


Components:
- Mode indicator
- File name
- Modified indicator [+]
- Encoding
- Line ending
- Filetype
- Line:Column
- Percentage through file

## Configuration

### Component Order


### Available Components

| Component | Description |
|-----------|-------------|
| `mode` | Current editing mode |
| `file` | File path/name |
| `file_name` | Just filename |
| `file_path` | Full path |
| `modified` | [+] if modified |
| `readonly` | [RO] if readonly |
| `encoding` | File encoding |
| `fileformat` | Line ending (lf/crlf) |
| `filetype` | Language type |
| `position` | Line:Column |
| `progress` | Percentage in file |
| `git_branch` | Current git branch |
| `git_status` | Git file status |
| `diagnostics` | Error/warning counts |
| `lsp_status` | LSP server status |
| `selection` | Selection info |

### Custom Separators


### Mode Colors


## Per-Mode Statusline

Different layout per mode:


## Custom Components

Define custom components:


## Hiding Statusline


## Examples

### Minimal


### Powerline Style

