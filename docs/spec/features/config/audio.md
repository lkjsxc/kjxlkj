# Audio Feedback

Optional audio cues for accessibility.

## Overview

Audio feedback provides non-visual cues for
mode changes, errors, and events.

## Enabling Audio


## Terminal Bell

### Basic Bell


### Bell Events


## Supported Cues

| Event | Default | Configurable |
|-------|---------|--------------|
| Error | Bell | Yes |
| Mode change | None | Yes |
| Search wrap | Bell | Yes |
| Motion fail | None | Yes |
| Save complete | None | Yes |

## Configuration

### Events


### Bell Type


## Visual Bell

### Instead of Sound


### Flash Duration


## External Sounds

### Command Hook


### Per-Event Sounds


## Screen Reader Integration

### Speech Synthesis

kjxlkj doesn't directly support speech synthesis.
Use system screen readers:

- NVDA (Windows)
- Orca (Linux)
- VoiceOver (macOS)

### Terminal Requirements

Use accessible terminal emulator.

## Accessibility Features

### Cursor Position

Announce cursor position on navigation.

### Mode Announcements

Screen reader announces mode changes.

### Error Messages

Errors spoken aloud.

## Configuration Reference


## Terminal Support

### Bell Support

Most terminals support the bell character (`\a`).

### Visual Bell Support

| Terminal | Visual Bell |
|----------|-------------|
| Kitty | ✓ |
| Alacritty | ✓ |
| iTerm2 | ✓ |
| GNOME Terminal | ✓ |

## Tips

1. Use visual bell in quiet environments
2. Enable bell for critical events only
3. Test with your terminal
4. Combine with screen reader for full accessibility
