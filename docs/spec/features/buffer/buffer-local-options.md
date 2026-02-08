# Buffer-Local Options

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Options scoped to individual buffers.

## Overview

Buffer-local options override global settings for a specific buffer.

## Setting Commands

| Command | Description |
|---|---|
| `:setlocal {option}={value}` | Set option for current buffer only |
| `:setlocal {option}?` | Query buffer-local value |
| `:set {option}={value}` | Set globally (and for current buffer) |

## Common Buffer-Local Options

| Option | Description |
|---|---|
| `filetype` | File type |
| `tabstop` | Tab width |
| `shiftwidth` | Indent width |
| `expandtab` | Spaces vs tabs |
| `textwidth` | Line wrap width |
| `fileencoding` | File encoding |
| `fileformat` | Line ending format |

## Filetype Detection

On file open, the editor detects the file type and applies corresponding buffer-local settings from filetype configuration.

## Modeline

The first/last few lines of a file can contain modeline directives that set buffer-local options.

## Related

- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- Filetype config: [/docs/spec/features/config/ftconfig.md](/docs/spec/features/config/ftconfig.md)
