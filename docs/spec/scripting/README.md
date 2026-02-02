# Scripting

Mapping and automation specifications for kjxlkj.

## Overview

kjxlkj does not have a plugin system, but provides:

- Custom keybinding mappings
- User-defined commands
- Event-driven automation
- Script file loading

## Directory Structure

| Directory | Content |
|-----------|---------|
| [mappings/](mappings/README.md) | Keybinding mappings |

## Documents

| Document | Content |
|----------|---------|
| [cmdline-completion.md](cmdline-completion.md) | Command completion |
| [event-automation.md](event-automation.md) | Event automation |
| [script-files.md](script-files.md) | Script loading |
| [timing-debounce.md](timing-debounce.md) | Timing control |
| [user-commands.md](user-commands.md) | Custom commands |
| [user-functions.md](user-functions.md) | Custom functions |

## Key Concepts

| Concept | Description |
|---------|-------------|
| Mapping | Custom key → action binding |
| Command | Named ex command sequence |
| Autocommand | Event-triggered action |
| Function | Reusable script logic |

## Related

- Commands: [docs/spec/commands/README.md](docs/spec/commands/README.md)
- UX keybindings: [docs/spec/ux/keybindings.md](docs/spec/ux/keybindings.md)
