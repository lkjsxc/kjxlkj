# Unicode and Multilingual Text Guidance

Back: [/docs/technical/README.md](/docs/technical/README.md)

This document translates the Unicode specs into implementation guidance.

## Canonical behavior sources

- [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Implementation split

| Concern | Owner |
|---|---|
| UTF-8 storage and edits | core text model |
| Cursor semantics and mode clamping | core state/mode logic |
| Width and wrapped row layout | renderer + viewport logic |
| Terminal/IME event decoding | host/input layer |

## Required invariants

| Invariant | Why it matters |
|---|---|
| Buffer bytes remain valid UTF-8 | prevents corruption and crashes on save/reload |
| Insert to Normal transition always clamps to end-exclusive cursor | prevents floating cursor defects (`a ... Esc`) |
| Wrap uses display width, not raw byte count | prevents off-screen drift on CJK and emoji lines |
| Composition preedit is not committed early | prevents ghost text and duplicated Japanese input |

## Japanese input guidance

| Topic | Guidance |
|---|---|
| Preedit lifecycle | model preedit as transient state in input layer or host bridge |
| Commit event | apply as one atomic insertion transaction |
| Cancel path | clear transient state without touching committed buffer |
| Leader safety | suppress mapping expansion for composition-only events |

## Long-line behavior

When `wrap = true`, long lines MUST wrap to the next display row regardless of script.

Implementations SHOULD avoid full-line remeasurement on every frame by caching grapheme/width slices per visible window span.

## Regression design

Minimum regression classes:

| Class | Example |
|---|---|
| Cursor clamp regression | repeated `a`, type text, `Esc`, cursor never beyond EOL in Normal mode |
| IME conversion regression | conversion candidate cancel/commit behavior remains deterministic |
| Width regression | mixed Japanese + ASCII line stays navigable with stable wrapping |
| Persistence regression | file written with `:wq` preserves expected UTF-8 bytes |

## Current quality gate

Reconstruction SHOULD include:

- unit and integration tests for grapheme/width-aware cursor behavior
- PTY E2E tests for Unicode typing and Japanese composition paths
- limitation entry if any IME path is not automated on current platform

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Known gaps ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
