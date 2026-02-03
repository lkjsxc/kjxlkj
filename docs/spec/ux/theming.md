# Theming
spec treats theming as pure data applied at render time.

## Requirements

- Theme changes never mutate core editor state.
- Colors/styles are applied in the renderer from UI snapshots.
- Diagnostic and git semantics remain consistent across themes.

## Related

- UI system: [docs/spec/ui/README.md](/docs/spec/ui/README.md)
- Syntax and diagnostics surfaces: [docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md), [docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)
