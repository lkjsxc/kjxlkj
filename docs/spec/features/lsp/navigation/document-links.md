# Document Links

Navigate URLs and file links in documents.

## Overview

Document links recognizes URLs and file paths,
making them clickable for navigation.

## Supported Links

### URLs


### File Paths


### Line References


## Navigation

### Keybindings

| Key | Action |
|-----|--------|
| `gx` | Open link under cursor |
| `<C-]>` | Follow link |

### Mouse

`Ctrl+Click` on link to open.

## Display

### Highlighting

Links displayed with underline:


### Configuration


## URL Handling

### External URLs

Opens in system browser.


### File URLs

Opens in kjxlkj buffer.

## File Resolution

### Relative Paths

Resolved relative to current file.

### Workspace Paths

Resolved relative to workspace root.

### Configuration


## Link Types

### Markdown Links

Standard Markdown link syntax is recognized:

- **Text Links** - Written as `[Link Text](url)` format
- **Image Links** - Written as `![Image](path/to/image.png)` format

## Commands

| Command | Description |
|---------|-------------|
| `:DocumentLinks` | List all links in document |
| `:OpenLink` | Open link at cursor position |
