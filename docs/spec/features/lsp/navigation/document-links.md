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

Mouse input is ignored.

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

- **Text Links** - Written using the pattern `[` `Link Text` `]` `(` `target` `)`
- **Image Links** - Written using the pattern `!` `[` `Alt` `]` `(` `path` `)`

## Commands

| Command | Description |
|---------|-------------|
| `:DocumentLinks` | List all links in document |
| `:OpenLink` | Open link at cursor position |
