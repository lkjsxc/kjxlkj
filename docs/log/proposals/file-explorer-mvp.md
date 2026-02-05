# Proposal: File Explorer MVP

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Need a minimal file explorer for navigation: toggle visibility, navigate directories, open files.

## Defining specs

- [/docs/spec/features/README.md](/docs/spec/features/README.md)
- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Status

Placeholder - to be refined in implementation wave.

## Acceptance criteria

- Given Normal mode, when pressing toggle key, then file explorer MUST toggle visibility.
- Given file explorer, when navigating, then MUST show current directory contents.
- Given file explorer, when selecting file, then MUST open in editor.

## Test strategy

- Unit tests for file explorer state
- E2E test for file explorer workflow
