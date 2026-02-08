# Window Zoom and Maximize

Full-screen window operations.

## Overview

Temporarily maximize a window
while preserving layout state.

## Zoom Toggle

### Basic Zoom

| Command | Action |
|---------|--------|
| `<C-w>_` | Maximize current window height |
| `<C-w>\|` | Maximize current window width |
| `:ZoomToggle` | Toggle full zoom on/off |

### Behavior

- First press: maximize current window
- Second press: restore previous layout
- Other windows minimized, not closed
- Buffer list unchanged during zoom

## Maximize Commands

### Height Only

| Command | Effect |
|---------|--------|
| `<C-w>_` | Current window takes all available height |
| `[count]<C-w>_` | Set height to `count` rows |
| Result | Other windows shrink to `winminheight` (default 1 row) |

### Width Only

| Command | Effect |
|---------|--------|
| `<C-w>\|` | Current window takes all available width |
| `[count]<C-w>\|` | Set width to `count` columns |
| Result | Other windows shrink to `winminwidth` (default 1 col) |

### Full Maximize

Combine both commands to fill the entire frame:

1. `<C-w>_` -- maximize height
2. `<C-w>\|` -- maximize width
3. Other windows become minimal (1 row/col each)
4. `:ZoomToggle` performs both in a single action

## Zoom vs Only

### Zoom (`:ZoomToggle`)

- Preserves layout state
- Reversible with same command
- Other windows hidden, not closed

### Only (`:only`, `<C-w>o`)

- Closes other windows
- Not reversible
- Buffers remain in buffer list

## Configuration

### Zoom Settings

| Option | Default | Description |
|--------|---------|-------------|
| `winheight` | `1` | Min height for current window |
| `winwidth` | `1` | Min width for current window |
| `winminheight` | `1` | Min height for non-current windows |
| `winminwidth` | `1` | Min width for non-current windows |
| `equalalways` | `on` | Auto-equalize after split/close |

### Zoom Indicator

Display `[Z]` in the statusline when the current
tab is in a zoomed state. Driven by `t:zoomed`.

## Statusline Integration

### Show Zoom State

| Variable | Value | Meaning |
|----------|-------|---------|
| `t:zoomed` | `1` | Tab is currently zoomed |
| `t:zoomed` | `0` or unset | Tab is not zoomed |

### Template

Statusline expression to show zoom state:

`%{exists('t:zoomed') && t:zoomed ? '[Z]' : ''}`

## Keybindings

### Zoom Keys

| Key | Action |
|-----|--------|
| `<C-w>z` | Toggle zoom for current window |
| `<Leader>z` | Suggested custom zoom mapping |

### Maximize Keys

| Key | Action |
|-----|--------|
| `<C-w>_` | Maximize height only |
| `<C-w>\|` | Maximize width only |
| `<C-w>=` | Restore equal sizing (unzoom) |

## Zoom Events

### Autocommands

| Event | Fired |
|-------|-------|
| `User ZoomPre` | Before zoom or unzoom |
| `User ZoomPost` | After zoom or unzoom |

### Use Cases

- Resize companion panes after unzoom
- Update statusline highlights on zoom
- Disable distracting UI during zoom

## Focus Mode

### Distraction-Free

Combine zoom with UI hiding for focused editing:
hide statuslines, tab bar, and line numbers.

### Focus Options

| Option | Set To | Effect |
|--------|--------|--------|
| `laststatus` | `0` | Hide statusline |
| `showtabline` | `0` | Hide tab bar |
| `number` | `off` | Hide line numbers |
| `signcolumn` | `no` | Hide sign column |
| `cmdheight` | `0` | Hide command line |

## Presentation Mode

### Toggle

`:PresentationMode` toggles large font with
minimal UI. Internally calls `:ZoomToggle`
and applies presentation settings.

### Settings

| Setting | Value | Purpose |
|---------|-------|---------|
| `guifont` | Increased size | Readable from distance |
| `laststatus` | `0` | Hide statusline |
| `showtabline` | `0` | Hide tab bar |
| `cmdheight` | `0` | Hide command line |

## Layout Preservation

### How It Works

1. Save current layout (splits, sizes)
2. Maximize current window
3. On unzoom, restore saved layout

### State Storage

| Variable | Contents |
|----------|----------|
| `t:zoom_winrestcmd` | Output of `winrestcmd()` to restore sizes |
| `t:zoom_winnr` | Window number that was zoomed |
| `t:zoomed` | Boolean flag (`1` when zoomed) |

## Multiple Zooms

### Nested Zooming

Not supported. Zooming while zoomed
triggers unzoom instead.
