# Plugin Architecture

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

All features are built-in; there is no external plugin system.

## Overview

Unlike Vim/Neovim, this editor has no plugin API. Every feature that would traditionally be a plugin is implemented as a native Rust module within the appropriate crate.

## Rationale

- Eliminates plugin compatibility issues
- Single binary deployment
- Consistent performance
- Full type safety across all features

## Feature Modules

Each feature-module follows a standard pattern:

1. A struct implementing the feature logic
2. Registration with the message bus for relevant events
3. Configuration via the TOML config system
4. UI integration via the render pipeline

## Feature Categories

| Category | Examples |
|---|---|
| Navigation | Finder, flash, file explorer |
| Git | Gitsigns, diff mode, merge conflicts |
| LSP | Completions, hover, diagnostics |
| Editing | Surround, autopairs, comments |
| UI | Statusline, bufferline, indent guides |
| Debug | DAP integration |

## Adding Features

New features are added as modules within the relevant crate. Each feature must:

1. Handle its own configuration section in TOML
2. Register message bus subscriptions
3. Provide render fragments if it has UI
4. Include integration tests

## Related

- Crate structure: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Runtime: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
