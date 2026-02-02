# Canonical Specifications

kjxlkj is an async-first terminal editor with all features built-in natively.

## Directory Structure

| Directory | Content |
|-----------|---------|
| [architecture/](architecture/README.md) | System design |
| [commands/](commands/README.md) | Ex commands |
| [editing/](editing/README.md) | Text editing |
| [editor/](editor/README.md) | Editor core |
| [features/](features/README.md) | Built-in features |
| [modes/](modes/README.md) | Modal editing |
| [overview/](overview/README.md) | High-level overview |
| [scripting/](scripting/README.md) | Mappings and automation |
| [technical/](technical/README.md) | Technical details |
| [ui/](ui/README.md) | UI components |
| [ux/](ux/README.md) | User experience |

## Core Principles

| Constraint | Meaning |
|------------|---------|
| No plugins | Features are native components |
| Single-writer core | Only core task mutates state |
| Snapshot rendering | Rendering consumes immutable snapshots |
| Async services | IO/compute isolated in Tokio services |
| Deterministic edits | All edits serialized through core |

## Reading Order

1. [how-to-read.md](how-to-read.md) - Guide for reading specifications
2. [architecture/README.md](architecture/README.md)
3. [features/README.md](features/README.md)
4. [ux/keybindings.md](ux/keybindings.md)

## Related

- Policy: [docs/policy/README.md](docs/policy/README.md)
- Overview: [docs/overview/README.md](docs/overview/README.md)
