# Command completion
Completion is a UI layer over deterministic parsing.

## Requirements
- Completion does not mutate editor state.
- Completion sources may be async (filesystem, LSP, commands), but results are versioned and cancellable.

## Sources

- Command names
- Options and option values
- File paths (FS service)
- Buffer names

## Related

- Finder command palette: [docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
