# Proposal: Long Line Rendering

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Long lines must be rendered stably with proper virtualization to avoid performance issues.

## Defining specs

- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- [/docs/technical/large-files.md](/docs/technical/large-files.md)

## Status

Placeholder - to be refined in implementation wave.

## Acceptance criteria

- Given a line longer than viewport width, when rendering, then MUST NOT cause performance degradation.
- Given horizontal scroll, when scrolling, then viewport MUST update correctly.

## Test strategy

- Unit tests for long line rendering
- Performance tests for large files with long lines
