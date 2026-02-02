# Clipboard Integration

System clipboard integration across platforms.

## Overview

kjxlkj integrates with the system clipboard for
copy/paste operations.

## Registers

### Clipboard Registers

| Register | Description |
|----------|-------------|
| `"+` | System clipboard |
| `"*` | Primary selection (X11) |

### Usage


## Configuration

### Default Register


### Sync with Unnamed


## Platform Support

### Linux (X11)

Uses: xclip, xsel


### Linux (Wayland)

Uses: wl-clipboard


### macOS

Uses: pbcopy/pbpaste (built-in)

### Windows

Uses: Windows clipboard API (built-in)

## Detection

kjxlkj auto-detects clipboard provider:

1. Check for Wayland (wl-copy/wl-paste)
2. Check for X11 (xclip/xsel)
3. Check for macOS (pbcopy/pbpaste)
4. Check for Windows (win32 API)
5. Fall back to internal clipboard

## OSC 52

### Terminal Clipboard

Works over SSH and in containers.


### Supported Terminals

- Kitty
- Alacritty
- iTerm2
- WezTerm
- Windows Terminal

## Troubleshooting

### Clipboard Not Working


### SSH Sessions

Enable OSC52 in config.

### tmux


## Selection Types (X11)

### Primary

Middle-click paste. Selected text automatically.

### Clipboard

Ctrl+C/V. Explicit copy action.

### Secondary

Rarely used.

## Large Content

### Size Limits


### Chunked Transfer

Large content transferred in chunks.

## Security

### Sensitive Content


### Paste Confirmation


## Commands

| Command | Description |
|---------|-------------|
| `:clipboard` | Show clipboard content |
| `:clipboard clear` | Clear clipboard |

## Keybindings

| Key | Action |
|-----|--------|
| `<C-c>` | Copy (insert mode) |
| `<C-v>` | Paste (insert mode) |
| `"+y` | Copy to clipboard |
| `"+p` | Paste from clipboard |

## Internal Clipboard

### Fallback

When system clipboard unavailable:


### Persistence

