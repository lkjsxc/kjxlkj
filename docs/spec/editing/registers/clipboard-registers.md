# Clipboard Registers

System clipboard integration with `+` and `*` registers.

## Overview

| Register | System | Description |
|----------|--------|-------------|
| `"+` | Clipboard | System clipboard (Ctrl-C/V) |
| `"*` | Selection | Primary selection (X11 middle-click) |

## Platform Behavior

### Linux (X11/Wayland)

| Register | Clipboard |
|----------|-----------|
| `"+` | System clipboard (xclip, wl-copy) |
| `"*` | Primary selection (mouse selection) |

### macOS

Both registers use the same system clipboard (pbcopy/pbpaste).

### Windows

Both registers use the same system clipboard.

## Basic Usage

### Copy to System Clipboard


### Paste from System Clipboard


### Delete to Clipboard


## Visual Mode


## Insert Mode

| Key | Action |
|-----|--------|
| `Ctrl-R +` | Insert clipboard contents |
| `Ctrl-R *` | Insert selection contents |

## Command Line


## Configuration

### Sync with Unnamed Register


With `sync_unnamed = true`:

### Clipboard Provider


### Provider Commands


## Keybinding Shortcuts

Common convenience mappings:


## OSC 52 Support

For remote terminals (SSH), use OSC 52 escape sequences:


## Troubleshooting

### Check Clipboard Support


### Common Issues

| Problem | Solution |
|---------|----------|
| No clipboard | Install xclip or xsel |
| Wayland | Install wl-clipboard |
| SSH | Enable OSC 52 |
| WSL | Install win32yank |

## Selection vs Clipboard (X11)

### Selection (`*`)

- Updated when text is selected with mouse
- Pasted with middle-click
- Cleared when selection is lost

### Clipboard (`+`)

- Updated by explicit copy (Ctrl-C)
- Pasted by explicit paste (Ctrl-V)
- Persists until overwritten

## API Reference

