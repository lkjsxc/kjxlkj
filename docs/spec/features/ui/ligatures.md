# Ligatures

Programming font ligatures support.

## Overview

Ligatures combine character sequences into
single glyphs for improved readability.

## Requirements

### Font Support

Use a font with ligature support:

- Fira Code
- JetBrains Mono
- Cascadia Code
- Iosevka
- Hasklig

### Terminal Support

Terminal must support ligatures:

- iTerm2 (macOS)
- Kitty
- Alacritty
- Windows Terminal

## Configuration


## Common Ligatures

### Arrows

| Sequence | Ligature |
|----------|----------|
| `->` | → |
| `<-` | ← |
| `=>` | ⇒ |
| `<=` | ⇐ |
| `<->` | ↔ |
| `<=>` | ⇔ |

### Comparison

| Sequence | Ligature |
|----------|----------|
| `==` | ═ |
| `!=` | ≠ |
| `<=` | ≤ |
| `>=` | ≥ |
| `===` | ≡ |
| `!==` | ≢ |

### Logic

| Sequence | Ligature |
|----------|----------|
| `&&` | ∧ |
| `||` | ∨ |
| `!` | ¬ |

### Types

| Sequence | Ligature |
|----------|----------|
| `::` | ∷ |
| `..` | ‥ |
| `...` | … |

### Comments

| Sequence | Ligature |
|----------|----------|
| `//` | ⫽ |
| `/*` | ⁄* |
| `*/` | *⁄ |

## Selective Ligatures

### Enable Specific


### Disable Specific


## Context Awareness

### Disable in Strings


### Disable in Comments


## Cursor Behavior

### Under Cursor


Shows original characters when cursor is on ligature.

### Editing


## Performance

### Caching


### Limit


## Language-Specific


## Visual Width

Ligatures maintain correct character width
for cursor positioning:


## Debugging

View ligature sequences:


Re-enable:


## Terminal Compatibility

If ligatures don't display:

1. Check font is installed correctly
2. Verify terminal supports ligatures
3. Check terminal ligature settings
4. Try different font

## Font Configuration

### macOS iTerm2

Preferences → Profiles → Text:
- Enable "Use ligatures"

