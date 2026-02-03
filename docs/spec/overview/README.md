# Overview

Back: [/docs/spec/README.md](/docs/spec/README.md)
## Reading order

1. [docs/spec/README.md](/docs/spec/README.md)
2. [docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
3. [docs/spec/features/README.md](/docs/spec/features/README.md)
4. [docs/spec/technical/README.md](/docs/spec/technical/README.md)

## What is kjxlkj
A Neovim-inspired terminal editor with:

- A **Tokio async-first runtime** (services for IO and heavy compute)
- A **single-writer core** (deterministic edits)
- **Built-in “top plugin” capabilities** (finder, explorer, LSP+completion, git UX, etc.)
- Snapshot-driven rendering (render consumes immutable snapshots)

## Product boundary

| Non-goal | Meaning |
|---|---|
| Plugin ecosystem | There is no plugin system; features ship natively. |
| Mouse-first UI | Keyboard-only interaction. |

## Glossary and principles

Legacy docs remain the place for vocabulary and values:

- [docs/overview/glossary.md](/docs/overview/glossary.md)
- [docs/overview/principles.md](/docs/overview/principles.md)

Specification-local references:

- [glossary.md](glossary.md)
- [principles.md](principles.md)
