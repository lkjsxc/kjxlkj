# Bidirectional Text (Implementation Guidance)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Guidance for supporting right-to-left and mixed-direction text in a terminal editor.

Status note: many terminals do not perform full bidirectional reordering for you. If bidi support is incomplete or absent in the current implementation, that gap should be recorded in:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Goals

- Preserve logical text order in the buffer (editing is always on the stored sequence).
- Provide a deterministic visual presentation that matches the Unicode Bidirectional Algorithm (UAX #9).
- Keep cursor behavior predictable and testable in mixed-direction lines.

## Non-goals (unless explicitly specified later)

- Perfect script shaping in every terminal/font combination.
- Vertical text layout.
- Pixel-perfect compatibility with GUI editors.

## Required model split

| Concern | Recommended owner | Notes |
|---|---|---|
| Logical text | Core | buffer stores the literal code point sequence |
| Visual ordering | Renderer (or dedicated layout layer) | maps logical indices to visual cells |
| Cursor semantics | Core + layout mapping | core stores logical positions; layout maps to screen |

Key principle: editing commands operate on logical text indices; bidi only affects how those indices map to screen positions.

## Cursor movement models

Mixed-direction text forces an explicit decision: when the user presses “left” or “right”, should movement follow the screen or the logical string?

Recommended target posture:

- “left/right” in a bidi-enabled view moves visually (screen-direction).
- word motions and higher-level text objects remain logical.

If both behaviors exist, the mode must be explicit and user-visible.

## Selection and operators

Selections should be defined over logical ranges, even if the on-screen highlight is discontinuous.

Target invariants:

- a selection is serializable as logical ranges (start/end indices)
- applying an operator to a selection must not depend on terminal rendering quirks

## Direction control and markers

Unicode includes directional markers that affect bidi resolution (e.g., LRM/RLM).

Target guidance:

- preserve markers as literal text (do not strip them)
- offer an optional “show bidi marks” rendering mode for debugging

## Terminal reality and compatibility

Many terminals:

- shape complex scripts via font shaping
- do not reorder characters visually according to UAX #9

Therefore, full bidi support typically requires the editor to:

- compute a visual ordering for each displayed line segment
- render the reordered glyph sequence to the terminal

This is a substantial feature; it should be implemented behind clear conformance/limitations tracking.

## Testing (recommended)

Tests should cover:

- a line containing mixed RTL and LTR segments
- cursor-left/right movement across direction boundaries
- selection application across mixed runs

Prefer tests that validate the logical↔visual mapping rather than relying on a particular terminal’s bidi behavior.

## References

- Unicode Bidirectional Algorithm (UAX #9)
