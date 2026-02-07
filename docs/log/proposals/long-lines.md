# Proposal: Long-line Rendering Stability

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Lines exceeding the viewport width must render without artifacts, flickering, or cursor drift. Both wrapped and truncated display modes must be stable.

## Defining specs

- Viewport invariants: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Large files: [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Scroll customization: [/docs/spec/features/ui/scroll-customization.md](/docs/spec/features/ui/scroll-customization.md)

## Conformance claim

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Viewport section

## Limitations

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — long-line entry

## Implementation TODO

- [/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md](/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md)

## Test requirements

- Given a line exceeding viewport width, when the viewport renders, then the line is correctly truncated or wrapped.
- Given cursor on a long line, when navigating horizontally, then horizontal scroll follows the cursor.
- Given a long line with multibyte characters, when rendered, then column alignment is correct.

## Status

Implemented and tested in kjxlkj-render and kjxlkj-core-ui crates.
