# Font Rendering

Text rendering in the terminal interface.

## Overview

kjxlkj runs in a terminal emulator and relies on the terminal for font selection, glyph rendering, and anti-aliasing. The editor's responsibility is correct character width calculation and cell placement.

## Character Width Categories

| Category | Display Width | Examples |
|---|---|---|
| ASCII printable | 1 | `a`-`z`, `0`-`9`, punctuation |
| Latin extended | 1 | Accented characters |
| CJK ideographs | 2 | Chinese, Japanese kanji, Korean |
| CJK fullwidth | 2 | Fullwidth ASCII (`Ａ`), fullwidth punctuation |
| Halfwidth katakana | 1 | `ｱ`, `ｲ`, `ｳ` |
| Emoji (presentation) | 2 | Emoji with VS16 or default emoji presentation |
| Combining marks | 0 | Diacritics, combining diacritical marks |
| Control characters | 0 (or `^X` = 2) | Displayed as `^X` notation |
| Tab | Variable | Expands to next tabstop |

Width is determined by Unicode East Asian Width property plus overrides for emoji and ambiguous-width characters.

## Ambiguous Width

Characters with East Asian Width = Ambiguous (e.g., Greek letters, some symbols) default to width 1. Configurable via `editor.ambiguous_width` (1 or 2) for CJK locale terminals.

## Recommended Fonts

For best rendering, use a monospace font with Nerd Font patching for icons:

| Font | Notes |
|---|---|
| JetBrains Mono NF | Ligatures, clear glyphs |
| Fira Code NF | Extensive ligatures |
| Hack NF | Highly legible |
| Iosevka NF | Narrow, customizable |

## Rendering Modes

| Mode | Description |
|---|---|
| True color (24-bit) | `COLORTERM=truecolor` — full RGB |
| 256 color | Fallback for older terminals |
| 16 color | Basic ANSI colors only |

The renderer detects terminal capability and adjusts automatically.

## Related

- Unicode handling: [/docs/spec/technical/unicode.md](/docs/spec/technical/unicode.md)
- Render pipeline: [/docs/spec/ui/render-pipeline.md](/docs/spec/ui/render-pipeline.md)
