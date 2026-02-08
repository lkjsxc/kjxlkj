# Ligatures

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Programming font ligature awareness in the rendering pipeline.

## Overview

Ligatures are sequences of characters that a font renders as a single combined glyph. Because kjxlkj is a TUI application, ligature rendering is delegated entirely to the terminal emulator. The editor's responsibility is to ensure correct cursor positioning and editing behavior when ligatures are visually present.

## Requirements

Terminal font requirements for ligature rendering.

### Font support

The user must configure a ligature-capable font in their terminal. Common programming fonts with ligature support include Fira Code, JetBrains Mono, Cascadia Code, Iosevka, and Hasklig.

### Terminal support

The terminal emulator must support ligature rendering. Ligature rendering is a terminal feature, not an editor feature.

| Terminal | Ligature support |
|---|---|
| Kitty | Yes |
| iTerm2 | Yes |
| Alacritty | No (by design) |
| Windows Terminal | Yes |
| GNOME Terminal (VTE) | No |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `editor.ligatures` | boolean | `true` | Enable ligature-aware behavior |

When `editor.ligatures = false`, the editor makes no special accommodation for ligatures. Characters are treated independently.

## Cursor behavior

How the cursor interacts with ligature sequences.

### Cursor on ligature sequence

When the cursor moves through a character sequence that the terminal renders as a ligature, the editor MUST still move one grapheme at a time. Each character in the sequence occupies one cell. The cursor positions are:

| Cursor position | Display |
|---|---|
| On first char of sequence | Cursor at the start of the ligature glyph |
| On middle char of sequence | Cursor in the middle of the ligature glyph |
| On last char of sequence | Cursor at the end of the ligature glyph |

The editor does NOT need to expand or decompose ligatures. The terminal handles all glyph rendering.

### Editing within ligatures

When the user inserts or deletes characters within a ligature sequence, the terminal will re-render the affected cells. The editor simply updates the buffer text and redraws the line; the terminal decides whether to form or break a ligature.

## Visual width

Ligatures do not change the logical width of characters. Each character in a ligature sequence still occupies exactly one terminal cell. The display width mapping in the cursor model is unaffected by ligature rendering.

## Performance

Ligature rendering is entirely a terminal-side concern. The editor's rendering pipeline outputs individual characters to their respective cells. No caching or special processing is required for ligatures.

## Language-specific behavior

No language-specific ligature configuration is needed. The editor is ligature-agnostic; it outputs characters and the terminal decides which sequences to combine into ligature glyphs.

## Related

- Font rendering: [/docs/spec/features/ui/font-rendering.md](/docs/spec/features/ui/font-rendering.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Render pipeline: [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
