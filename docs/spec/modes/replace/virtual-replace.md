# Virtual Replace Mode

Tab-aware and width-aware character replacement.

## Overview

Virtual replace mode (`gR`) replaces characters by display width rather than byte position. Unlike standard replace mode (`R`), it preserves column alignment when replacing tabs and wide characters.

## Entry and Exit (normative)

| Key | Action |
|---|---|
| `gR` | Enter virtual replace mode |
| `gr{char}` | Replace single character virtually |
| `Esc` | Exit to Normal mode |

## Standard R vs Virtual gR (normative)

| Aspect | `R` (standard) | `gR` (virtual) |
|---|---|---|
| Tab handling | Replaces tab byte with one character | Treats tab as its display-width columns |
| Wide char (CJK) | Replaces one byte | Replaces based on display width (2 columns) |
| Column alignment | May break alignment | Preserves alignment |
| Backspace | Restores original character (1 byte) | Restores original display columns |

## Tab Replacement Example

Given a tab displaying as 8 spaces:

- `R` then typing `ab` replaces the tab with `a`, then replaces the next character with `b`.
- `gR` then typing `ab` replaces the first 2 columns of the tab's display. The remaining 6 columns are filled with spaces. Visual alignment is preserved.

## Wide Character (CJK) Handling

When replacing a CJK character (display width 2) in virtual replace mode:

- Typing a width-1 character replaces the first column. The second column becomes a space to preserve width.
- Typing another width-2 character replaces both columns exactly.

## Backspace in Virtual Replace

Backspace restores the original display columns, not just the original byte. If the original was a tab spanning 8 columns, backspace restores all 8 display columns.

## Single Virtual Replace (gr)

`gr{char}` replaces a single character at the cursor position using virtual-replace semantics, then returns to Normal mode. It is the virtual equivalent of `r{char}`.

## Count

`{count}gR` enters virtual replace mode and the subsequent replacement text is applied `count` times on exit (same behavior as counted replace mode).

## Comparison Table

| Feature | `R` | `gR` |
|---|---|---|
| Tab treated as | 1 byte | Display width |
| Wide char treated as | 1 byte | Display width |
| Alignment | May break | Preserved |
| Best for | Byte-level replacement | Visual/tabular data |

## Related

- Replace mode: [/docs/spec/modes/replace/replace-mode.md](/docs/spec/modes/replace/replace-mode.md)
- CJK/Unicode: [/docs/technical/unicode.md](/docs/technical/unicode.md)
