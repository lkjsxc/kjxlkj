# Font Rendering

Text rendering and font handling.

## Overview

Font rendering controls how text appears
in the terminal interface.

## Terminal Fonts

### Selection

The editor uses the terminal's font settings.
Configure fonts in your terminal emulator.

### Recommended Fonts

| Font | Features |
|------|----------|
| JetBrains Mono | Ligatures, clear |
| Fira Code | Extensive ligatures |
| Cascadia Code | Windows default |
| Iosevka | Narrow, customizable |
| Source Code Pro | Adobe, clean |
| Hack | Highly legible |

### Nerd Font Variants

For icons, use Nerd Font patched versions:

- JetBrainsMono Nerd Font
- FiraCode Nerd Font
- Hack Nerd Font

## Character Width

### Detection


### Manual Override


### Wide Characters

| Category | Width |
|----------|-------|
| ASCII | 1 |
| CJK | 2 |
| Emoji | 2 |
| Combining | 0 |

## Cell Rendering

### Standard

Each character occupies terminal cells:


### Alignment


## Font Fallback

### Terminal Handles

The terminal manages font fallback.
Configure fallback in terminal settings.

### Example (iTerm2)


### Example (Kitty)


## Rendering Modes

### True Color


### 256 Color


### Basic

For limited terminals:


## Anti-Aliasing

Handled by terminal. Common settings:

### macOS

- LCD smoothing
- Font smoothing level

### Linux

- FreeType settings
- Fontconfig hinting

## Line Height

Terminal-controlled. For spacing:


## Tab Rendering


## Space Rendering


## Cursor Rendering


## Selection Rendering


## Performance

### Refresh Rate


### Batch Rendering


### Lazy Rendering

