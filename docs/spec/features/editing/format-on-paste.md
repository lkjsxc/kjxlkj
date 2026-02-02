# Format on Paste

Automatic formatting when pasting code.

## Overview

Format on paste automatically formats pasted content
to match the surrounding code style.

## Enabling


## Behavior

### Before Paste


### Pasting Unformatted Code


### After Format on Paste


## Configuration


## Indentation Adjustment

### Smart Indent

Pasted code adjusted to cursor indent level.

### Example

Pasting at 4-space indent:


## Format Content

### Enabled

Full formatting applied (spacing, style).

### Disabled

Only indentation adjusted.


## Source Detection

### Same File Type

Full formatting available.

### Different File Type

Only indentation adjustment.

### Plain Text

Indentation adjustment only.

## LSP Integration

### Range Formatting

Uses LSP range formatting for pasted region.

### Fallback

External formatter or indent adjustment.

## Per-Language


## Undo

### Single Undo

Paste and format are single undo unit.


## Performance

### Large Pastes


Skip formatting for very large pastes.

## Disable Temporarily

### Paste Without Format


### Command


## Tips

1. Enable for code files
2. Disable for prose
3. Use `<C-S-v>` for raw paste
4. Check indent style matches

## Configuration Reference

