# Bufferline

Visual buffer tab bar at the top of the editor.

## Overview

The bufferline displays open buffers as tabs in a
horizontal bar at the top of the editor window. It
provides visual feedback about which buffers are open,
which is active, and which have unsaved changes.

## Layout

### Position

The bufferline occupies one row at the top of the
terminal, above the window area. When disabled, the
row is reclaimed for editing space.

### Tab Display

Each tab shows:

| Element | Description |
|---------|-------------|
| Icon | File type icon (if icons enabled) |
| Name | File name (not full path) |
| Modified | `[+]` indicator for unsaved changes |
| Close | Visual hint (keyboard-only close) |

### Overflow

When tabs exceed the available width, the bufferline
scrolls horizontally. Indicators show that more tabs
exist in each direction.

## Navigation

### Keyboard Commands

| Key / Command | Action |
|---------------|--------|
| `:BufferNext` / `]b` | Switch to next buffer |
| `:BufferPrev` / `[b` | Switch to previous buffer |
| `:BufferPick` | Show letter labels for jump |
| `:buffer {n}` | Switch to buffer number n |
| `:buffer {name}` | Switch by partial name match |
| `Ctrl-^` | Toggle alternate buffer |

### Pick Mode

`:BufferPick` overlays a unique letter on each tab.
Pressing the letter switches to that buffer. Press
`Esc` to cancel pick mode.

## Buffer Groups

### Grouping Strategies

| Strategy | Description |
|----------|-------------|
| `directory` | Group by parent directory |
| `filetype` | Group by detected file type |
| `project` | Group by project root |
| `manual` | User-assigned groups only |
| `none` | No grouping |

### Display

Groups appear as labeled sections in the bufferline.
The group name is shown as a separator between tabs.

## Pinned Buffers

### Behavior

Pinned buffers always appear first in the bufferline
and are not affected by `:BufferCloseLeft` or
`:BufferCloseRight`.

### Commands

| Command | Action |
|---------|--------|
| `:BufferPin` | Toggle pin on current buffer |
| `:BufferMoveLeft` | Move tab position left |
| `:BufferMoveRight` | Move tab position right |

## Sorting

### Sort Options

| Value | Description |
|-------|-------------|
| `insertion` | Order of opening (default) |
| `name` | Alphabetical by file name |
| `directory` | By parent directory, then name |
| `extension` | By file extension, then name |
| `modified` | Most recently modified first |

### Command

`:BufferSort {strategy}` re-sorts the bufferline.

## Commands Summary

| Command | Description |
|---------|-------------|
| `:BufferNext` | Next buffer |
| `:BufferPrev` | Previous buffer |
| `:BufferClose` | Close current buffer |
| `:BufferCloseLeft` | Close all buffers to the left |
| `:BufferCloseRight` | Close all buffers to the right |
| `:BufferOnly` | Close all except current |
| `:BufferPick` | Pick buffer by letter |
| `:BufferPin` | Toggle pin status |
| `:BufferSort {s}` | Sort bufferline |
| `:BufferMoveLeft` | Move tab left |
| `:BufferMoveRight` | Move tab right |

## Appearance Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `bufferline.enabled` | bool | true | Show bufferline |
| `bufferline.style` | string | "slant" | Tab separator style |
| `bufferline.icons` | bool | true | Show file type icons |
| `bufferline.numbers` | bool | false | Show buffer numbers |
| `bufferline.max_name_length` | int | 18 | Truncate long names |
| `bufferline.sort` | string | "insertion" | Default sort |
| `bufferline.group_by` | string | "none" | Grouping strategy |

## Buffer Limit Warning

When `bufferline.max_buffers` (default: 0, disabled)
is set and the count exceeds it, a warning appears
in the status line suggesting to close unused buffers.

## Related

- Buffer model: [docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Buffer groups: [docs/spec/features/buffer/buffer-groups.md](/docs/spec/features/buffer/buffer-groups.md)
- Buffer switching: [docs/spec/features/buffer/buffer-switching.md](/docs/spec/features/buffer/buffer-switching.md)
