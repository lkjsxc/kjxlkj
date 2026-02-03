# Canonical Specifications

Back: [/docs/README.md](/docs/README.md)
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

## How to Read (Recommended)

### Reading order

| Order | Document | Purpose |
|-------|----------|---------|
| 1 | [README.md](README.md) | Spec index and core principles |
| 2 | [architecture/README.md](architecture/README.md) | System shape overview |
| 3 | [architecture/runtime.md](architecture/runtime.md) | Runtime and ordering model |
| 4 | [features/README.md](features/README.md) | Built-in feature categories |
| 5 | [ux/keybindings.md](ux/keybindings.md) | Keybinding reference |

### Document types

| Type | Purpose | Examples |
|------|---------|----------|
| README.md | Directory index and navigation | Every directory |
| Spec files | Normative requirements | commands/, editing/, modes/ |
| Reference files | Lookup tables and mappings | ux/keybindings.md |

### Spec language

| Keyword | Meaning |
|---------|---------|
| MUST | Mandatory requirement |
| MUST NOT | Prohibited behavior |
| SHOULD | Recommended but not mandatory |
| MAY | Optional behavior |

### Navigation conventions

| Link type | Pattern |
|-----------|---------|
| Parent | Links to parent directory `README.md` (use repo-root paths; avoid `../`) or parent section |
| Children | Listed in `## Documents` or `## Directory Structure` |
| Cross-references | Listed in `## Related` section |

### Key concepts

| Concept | Location |
|---------|----------|
| Core task | [architecture/runtime.md](architecture/runtime.md) |
| Services | [architecture/runtime.md](architecture/runtime.md) |
| Snapshots | [architecture/runtime.md](architecture/runtime.md) |
| Modes | [modes/README.md](modes/README.md) |
| Editing primitives | [editing/README.md](editing/README.md) |
| Glossary | [overview/glossary.md](overview/glossary.md) |

## Related

- Policy: [docs/policy/README.md](/docs/policy/README.md)
- Overview: [docs/overview/README.md](/docs/overview/README.md)
