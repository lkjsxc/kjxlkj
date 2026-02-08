# Unicode and Multilingual Text Guidance

Back: [/docs/technical/README.md](/docs/technical/README.md)

This document provides implementation guidance for Unicode text handling, with emphasis on CJK/Japanese text and wide character correctness.

## Canonical behavior sources

- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) — grapheme-based cursor model
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) — wrapping and display width
- [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md) — Unicode insertion
- [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) — IME behavior

## Implementation split

| Concern | Owner crate |
|---|---|
| UTF-8 storage and grapheme decomposition | `kjxlkj-core-text` |
| Display width computation (East Asian Width) | `kjxlkj-core-text` |
| Cursor arithmetic (grapheme-based) | `kjxlkj-core-edit` and `kjxlkj-core-state` |
| Mode transition clamping | `kjxlkj-core-mode` |
| Viewport wrapping with display widths | `kjxlkj-render` |
| IME preedit and composition events | `kjxlkj-input` and `kjxlkj-host` |
| Terminal cell rendering of wide chars | `kjxlkj-render` |

## Display width computation (normative)

The implementation MUST use the Unicode East Asian Width property (UAX #11) to determine character display widths:

| Category | Width | Examples |
|---|---|---|
| Narrow (Na), Neutral (N), Half-width (H) | 1 | ASCII, most Latin, Greek, Cyrillic |
| Wide (W), Full-width (F) | 2 | CJK ideographs, fullwidth ASCII, most Katakana/Hiragana |
| Ambiguous (A) | 1 (default) | Some Greek, Cyrillic, symbols; configurable per locale |
| Combining marks, zero-width joiners | 0 | Combining accents, ZWJ, ZWNJ |

The `unicode-width` crate or equivalent MUST be used. Simplified range checks are NOT sufficient.

## Grapheme cluster rules

| Rule | Requirement |
|---|---|
| Segmentation | Use Unicode UAX #29 grapheme cluster boundaries for all cursor operations. |
| Combining marks | A base character + combining marks = one grapheme cluster. |
| Emoji sequences | ZWJ sequences and flag sequences = one grapheme cluster. |
| Recommended crate | `unicode-segmentation` for grapheme boundaries. |

## CJK cursor behavior summary

The cursor model is grapheme-based (see [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)). Key rules:

| Rule | Detail |
|---|---|
| No half-cell positions | The cursor MUST never be on the second column of a wide character. |
| Motion granularity | `h`/`l` move one grapheme at a time; on CJK text this moves 2 display columns. |
| Block cursor width | On a width-2 grapheme, the block cursor spans 2 terminal columns. |
| Grapheme-to-display mapping | A bidirectional map between grapheme indices and display columns MUST be maintained. |

## Japanese input guidance

| Topic | Guidance |
|---|---|
| Preedit lifecycle | Model preedit as transient state in `kjxlkj-input`; do not write to buffer. |
| Commit event | Apply confirmed text as one atomic insertion at the insertion point. |
| Cancel path | Clear transient state; buffer remains unchanged. |
| Leader safety | `Space` during IME composition MUST NOT trigger leader mappings. |
| Esc safety | `Esc` during composition MUST cancel preedit first, not switch mode. |

## Line wrapping with CJK

When `wrap = true`, the wrapping algorithm MUST handle wide characters at row boundaries (see [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)):

| Scenario | Required behavior |
|---|---|
| Wide char at row boundary | If only 1 column remains, push wide char to next row; pad current row. |
| Mixed-width line | Display width varies per grapheme; wrapping uses cumulative display widths. |
| Performance | Cache grapheme/width slices per visible span; avoid full-line remeasurement per frame. |

## Required invariants

| Invariant | Why it matters |
|---|---|
| Buffer bytes remain valid UTF-8 | Prevents corruption on save/reload. |
| Mode transition clamps cursor to grapheme boundary | Prevents floating cursor on CJK text. |
| Wrap uses display width, not byte count | Prevents off-screen drift on CJK lines. |
| Preedit not committed early | Prevents ghost text and duplicated input. |

## Regression test classes

| Class | Example scenario |
|---|---|
| CJK cursor motion | `l` on `あいう` advances by 2 display columns per press |
| Mode churn on CJK | `a` + type CJK + `Esc` repeatedly; cursor always on grapheme boundary |
| IME commit/cancel | Commit inserts atomically; cancel leaves buffer unchanged |
| Mixed-width wrap | Line with ASCII + CJK wraps correctly at row boundaries |
| Wide char at boundary | Width-2 char pushed to next row when only 1 column remains |
| Persistence | `:wq` preserves expected UTF-8 bytes for CJK content |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- BiDi guidance: [/docs/technical/bidi.md](/docs/technical/bidi.md)
