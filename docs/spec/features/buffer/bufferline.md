# Buffer Line (Tab Line)

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

The top line showing open buffers or tabs.

## Overview

The bufferline displays a row of buffer/tab indicators at the top of the editor window.

## Layout

Each entry shows:

| Element | Content |
|---|---|
| Icon | File type icon (if enabled) |
| Name | File name |
| Modified | `●` indicator if modified |
| Close | `×` indicator (mouse click to close) |

## Navigation

| Key | Action |
|---|---|
| `gt` / `:tabnext` | Next tab |
| `gT` / `:tabprev` | Previous tab |
| `{N}gt` | Go to tab N |
| `<leader>1`..`<leader>9` | Go to buffer by position |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `bufferline.enabled` | `true` | Show bufferline |
| `bufferline.show_modified` | `true` | Show modified indicator |

## Related

- Buffer management: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)
- Buffer groups: [/docs/spec/features/buffer/buffer-groups.md](/docs/spec/features/buffer/buffer-groups.md)
