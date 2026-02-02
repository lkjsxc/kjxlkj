# Overview

High-level project overview for kjxlkj.

Purpose: provide orientation and shared vocabulary before reading design and technical specifications.

## Documents

| Document | Purpose |
|----------|---------|
| [principles.md](principles.md) | Core design principles |
| [glossary.md](glossary.md) | Terminology and definitions |

## Reading order

Canonical reading order:

1. [docs/spec/README.md](docs/spec/README.md)
2. [docs/spec/overview/README.md](docs/spec/overview/README.md)
3. [docs/spec/architecture/README.md](docs/spec/architecture/README.md)
4. [docs/spec/features/README.md](docs/spec/features/README.md)
5. [docs/spec/technical/README.md](docs/spec/technical/README.md)

Legacy vocabulary:

1. [principles.md](principles.md)
2. [glossary.md](glossary.md)

## What is kjxlkj?

A **Neovim-inspired TUI text editor** written in Rust with these characteristics:

- **Modal editing** - Normal, Insert, Visual, Command, Replace modes
- **Keyboard-only** - No mouse support
- **Single binary** - All features built-in, no plugins
- **Native performance** - Rust + terminal-native rendering
- **Modern features** - LSP, fuzzy finder, git integration, multiple cursors

The canonical spec clarifies that “modern features” are implemented as **built-in services** under a **Tokio async-first** runtime.

## Project Goals

1. **Familiar** - Vim-compatible where practical
2. **Fast** - Large file handling, minimal latency
3. **Integrated** - Everything works out of the box
4. **Hackable** - Clean codebase, clear architecture

## System-at-a-glance (conceptual)

This is a conceptual decomposition used for documentation and later implementation planning.

| Subsystem | Responsibility | Primary docs |
|---|---|---|
| Core editor | Editor state, buffers, window model, modes, commands | [docs/spec/editor/README.md](docs/spec/editor/README.md) |
| Input | Terminal events → editor actions | [docs/spec/features/README.md](docs/spec/features/README.md) |
| Rendering | State projection → terminal output | [docs/spec/ui/README.md](docs/spec/ui/README.md) |
| Persistence | File I/O, session behavior | [docs/spec/features/session/README.md](docs/spec/features/session/README.md) |
| Contracts | Cross-cutting invariants | [docs/spec/technical/contracts.md](docs/spec/technical/contracts.md) |

See [docs/spec/architecture/README.md](docs/spec/architecture/README.md) for the system-level diagrams.

## Quick Start for Developers

1. Read [docs/policy/INSTRUCT.md](docs/policy/INSTRUCT.md)
2. Understand [principles.md](principles.md)
3. Explore [docs/spec/architecture/README.md](docs/spec/architecture/README.md)

## Documentation policy

All documentation under [docs/](docs/README.md) is normative. See [docs/policy/README.md](docs/policy/README.md).
