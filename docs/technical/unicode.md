# Unicode and Multilingual Text Guidance

Back: [/docs/technical/README.md](/docs/technical/README.md)

Implementation guidance for Unicode handling, with focus on CJK and IME correctness.

## Canonical Sources

- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)

## Component Responsibilities

| Concern | Owner component |
|---|---|
| UTF-8 storage and grapheme segmentation | Text model layer |
| Display width computation | Text/layout layer |
| Cursor arithmetic and clamping | Editing/state layer |
| Mode transition clamping | Mode system |
| Viewport wrapping by display width | Render pipeline |
| IME preedit/commit/cancel handling | Input pipeline + core dispatch |
| Terminal cell rendering for wide chars | Render pipeline |

## Display Width Guidance

Use Unicode East Asian Width (UAX #11) for display widths.

| Category | Width |
|---|---|
| Narrow/Neutral/Half-width | 1 |
| Wide/Full-width | 2 |
| Ambiguous | 1 by default (locale policy may vary) |
| Combining marks / ZWJ sequences | 0 for combining unit contributions |

Avoid simplified codepoint-range heuristics.

## Grapheme Rules

Use UAX #29 grapheme boundaries for cursor and edit operations.

Key invariants:

- cursor never occupies half-cell position of a wide grapheme
- movement operates on grapheme units, not bytes
- mode transitions clamp to valid grapheme boundaries

## IME Guidance

- preedit text stays transient until commit
- commit inserts as one atomic change
- cancel clears transient state without mutating buffer
- leader and mode keys must not fire normal mappings during active composition

## Wrapping with CJK

When wrapping is enabled:

- wrapping must use display width, not byte count
- wide grapheme at last single remaining cell must move to next row
- visible-row work should stay viewport-bounded

## Regression Test Classes

| Class | Example |
|---|---|
| CJK cursor motion | movement across width-2 graphemes |
| Mode churn on CJK | insert/escape cycles remain boundary-safe |
| IME commit/cancel | commit writes once, cancel writes nothing |
| Mixed-width wrap | ASCII + CJK wraps deterministically |
| Wide boundary case | width-2 grapheme near row end handled correctly |
| Persistence | save/reload preserves expected UTF-8 bytes |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- BiDi guidance: [/docs/technical/bidi.md](/docs/technical/bidi.md)
