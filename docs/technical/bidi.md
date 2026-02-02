# Bidirectional Text

Support for right-to-left and mixed-direction text.

## Overview

kjxlkj supports bidirectional (bidi) text for
languages like Arabic, Hebrew, and Persian.

## Enabling Bidi


## Text Direction

### Right-to-Left (RTL)

- Arabic: العربية
- Hebrew: עברית
- Persian: فارسی

### Left-to-Right (LTR)

- English, European languages
- CJK (top-to-bottom in traditional)

### Mixed


## Cursor Behavior

### Visual Mode

Cursor moves visually (left/right on screen).

### Logical Mode

Cursor follows text order (reading direction).

### Configuration


## Editing

### Insertion

Text inserted in current direction context.

### Deletion

Backspace deletes logically previous character.

## Display Modes

### Implicit

Algorithm determines direction per segment.

### Explicit

Markers control direction:

- LRM (U+200E) - Left-to-Right Mark
- RLM (U+200F) - Right-to-Left Mark

### Configuration


## Statusline

Shows current paragraph direction:


## Selection

### Visual Selection

Highlights visual region, may be non-contiguous.

### Logical Selection

Selects logical text range.

## Line Numbers

### LTR Document


### RTL Document


## Terminal Support

### Requirements

- Unicode 9.0+ support
- Bidi-capable font
- Terminal bidi algorithm

### Tested Terminals

| Terminal | Support |
|----------|---------|
| Kitty | Full |
| Alacritty | Limited |
| iTerm2 | Full |
| Windows Terminal | Full |

## Limitations

### Current

- No vertical text
- Limited neutral handling
- Basic mirroring

### Planned

- Enhanced neutral support
- Paragraph direction detection
- Better cursor handling

## Keybindings

| Key | Action |
|-----|--------|
| `<C-S-u>` | Toggle direction |
| `<C-S-l>` | Force LTR |
| `<C-S-r>` | Force RTL |

## Tips

1. Use monospace fonts with RTL support
2. Enable bidi marks for debugging
3. Use visual cursor mode
4. Test with mixed content

## Resources

- Unicode Bidirectional Algorithm (UAX #9)
- ICU Bidi documentation
