# Unicode and Text Semantics (Implementation Guidance)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Guidance for implementing predictable Unicode behavior in a terminal editor.

Status note: this document describes target behavior and common pitfalls. The current shipped surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Goals

- Never corrupt user text (UTF-8 in, UTF-8 out).
- Keep cursor/motion semantics deterministic and panic-free.
- Separate “text indexing” (core) from “display width” (renderer).
- Make behavior testable with deterministic fixtures.

Related (cursor semantics): [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Indexing model

The editor needs a stable internal indexing model that does not depend on terminal rendering.

Recommended split:

| Concern | Owner | Notes |
|---|---|---|
| Text storage | Core | stores UTF-8; supports edits and line access |
| Cursor/motions | Core | operates on text indices, not cell widths |
| Display width | Renderer | determines how many terminal cells a slice occupies |

Implementation implication: core operations should avoid “scan the whole line for widths” unless explicitly required.

## Grapheme clusters (user-visible “characters”)

Many user-perceived characters are composed of multiple Unicode scalar values (code points):

- combining marks (e.g., base letter + accent)
- emoji ZWJ sequences
- regional indicator pairs (flags)

Target rule:

- Motions and “delete char” operations should treat a grapheme cluster as the smallest user-visible unit.

If the implementation is scalar-value-based (temporarily), the gap MUST be recorded as a limitation because it changes observable editing behavior.

## Display width

Terminal editors must handle both:

- width-1 characters (most ASCII)
- width-2 characters (many CJK and some emoji)
- width-0 characters (combining marks, zero-width joiner)

Important: terminal width is not purely Unicode-derived; it can vary by terminal emulator, font, and settings.

Target posture:

- use a Unicode-width model for baseline behavior
- accept that exact glyph width may vary and design clamping/scrolling to be robust

## Control characters and line endings

Suggested display policy (target):

| Input | Recommended display | Notes |
|---|---|---|
| `\\t` | configurable (spaces or visible marker) | renderer concern |
| `\\r` | visible marker (e.g., `^M`) | helps debug CRLF issues |
| `\\0` | visible marker (e.g., `^@`) | avoid embedding NUL into terminal output |

File input may contain CRLF. The editor should define a deterministic policy:

- either normalize to `\\n` in-memory (and record that in docs), or
- preserve original line endings and write them back

Whatever is chosen MUST be consistent and test-covered.

## Normalization

Unicode text can exist in multiple canonically equivalent forms (NFC/NFD).

Target guidance:

- do not silently normalize buffer contents during editing
- if search/case-folding uses normalization, document it and make it deterministic

## Bidirectional text

Bidi behavior is complex and should be treated as a separately specified feature:

- [/docs/technical/bidi.md](/docs/technical/bidi.md)

## Testing (recommended)

Unicode behavior should be locked down with explicit tests that cover:

- cursor movement across combining sequences
- insertion/deletion around ZWJ emoji sequences
- wide-character display width affecting viewport clamping
- CRLF/`^M` handling policy

If terminals differ, tests should target the core semantics (indices and invariants), not pixel-perfect terminal output.
