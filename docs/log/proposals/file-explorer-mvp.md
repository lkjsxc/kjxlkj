# Proposal: File Explorer MVP

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Users expect a toggleable file explorer panel for navigating the project directory tree without leaving the editor.

## Defining specs

- Navigation features: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)
- UI components: [/docs/spec/ui/components.md](/docs/spec/ui/components.md)

## Conformance claim

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Navigation section

## Limitations

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — file explorer entry

## Implementation TODO

- [/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md](/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md)

## MVP scope

- Toggle file explorer with a keybinding
- Navigate directory tree with cursor keys
- Open file under cursor in a buffer
- Show current working directory as root

## Test requirements

- Given the file explorer is closed, when the toggle key is pressed, then the explorer opens.
- Given the explorer is open, when a file is selected, then it opens in a buffer.
- Given the explorer is open, when the toggle key is pressed again, then the explorer closes.

## Status

Implemented and tested in kjxlkj-service-fs and kjxlkj-core-ui crates.
