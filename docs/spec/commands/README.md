# Commands

Back: [/docs/spec/README.md](/docs/spec/README.md)
Ex commands are a first-class interface layered atop the same intent system.

## Directory Structure

| Directory | Content |
|-----------|---------|
| [buffer/](buffer/README.md) | Buffer management commands |
| [cmdline/](cmdline/README.md) | Command-line interface |
| [execution/](execution/README.md) | Command execution modes |
| [file/](file/README.md) | File operations |
| [ranges/](ranges/README.md) | Range specifications |
| [session/](session/README.md) | Session and persistence |
| [substitute/](substitute/README.md) | Search and replace |

## Core Documents

| Document | Content |
|----------|---------|
| [syntax.md](syntax.md) | Command syntax |
| [essential.md](essential.md) | Essential commands |
| [quit-commands.md](quit-commands.md) | Quit and exit commands |

## Command Surfaces

| Surface | Purpose |
|---------|---------|
| Ex command line | File ops, window ops, config, tooling, scripting-like workflows |
| Finder-driven commands | Discoverable UI for commands and actions |
| Keymaps | Fast-path for common actions |

## Core Requirement

Commands MUST compile to typed intents that the core serializes.

## Async Integration

Commands that trigger IO/heavy compute MUST delegate to services:

- Grep/index queries
- Git operations
- LSP actions
- External command execution

The UI MUST show progress and allow cancellation.

## Related

- Modes: [docs/spec/modes/README.md](/docs/spec/modes/README.md)
- Features: [docs/spec/features/README.md](/docs/spec/features/README.md)
