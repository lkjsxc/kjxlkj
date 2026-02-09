# Neovim Migration Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

Guidance for Neovim users moving to `kjxlkj`.

## Important Status Rule

Treat this guide as orientation only. For exact availability:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Conceptual Similarities

The target interaction model is Vim-like:

- modal editing
- operator + motion grammar
- command-line workflows

Target definitions:

- [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

## Intentional Differences

- no external plugin loading
- built-in integrations only
- documentation-first reconstruction workflow

References:

- [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

## Migration Approach

1. Verify current supported behavior in conformance.
2. Map your key daily workflows to documented command/mode surfaces.
3. Track unresolved gaps in limitations before adopting as primary editor.

## Related

- Docs index: [/docs/README.md](/docs/README.md)
- UX target: [/docs/spec/ux/README.md](/docs/spec/ux/README.md)
