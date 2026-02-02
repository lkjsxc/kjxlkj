# How to Read Specifications

Guide for navigating and understanding kjxlkj specifications.

## Reading Order

| Order | Document | Purpose |
|-------|----------|---------|
| 1 | [README.md](README.md) | Spec index and core principles |
| 2 | [architecture/README.md](architecture/README.md) | System shape overview |
| 3 | [architecture/runtime.md](architecture/runtime.md) | Tokio runtime model |
| 4 | [features/README.md](features/README.md) | Built-in feature categories |
| 5 | [ux/keybindings.md](ux/keybindings.md) | Complete keybinding reference |

## Document Types

| Type | Purpose | Examples |
|------|---------|----------|
| README.md | Directory index and navigation | Every directory |
| Spec files | Normative requirements | commands/, editing/, modes/ |
| Reference files | Lookup tables and mappings | ux/keybindings.md |

## Understanding Spec Language

| Keyword | Meaning |
|---------|---------|
| MUST | Mandatory requirement |
| MUST NOT | Prohibited behavior |
| SHOULD | Recommended but not mandatory |
| MAY | Optional behavior |

## Navigation Conventions

| Link Type | Pattern |
|-----------|---------|
| Parent | Links to parent directory `README.md` (use repo-root paths; avoid `../`) or parent section |
| Children | Listed in `## Documents` or `## Directory Structure` |
| Cross-references | Listed in `## Related` section |

## Key Concepts

| Concept | Location |
|---------|----------|
| Core task | [architecture/runtime.md](architecture/runtime.md) |
| Services | [architecture/runtime.md](architecture/runtime.md) |
| Snapshots | [architecture/runtime.md](architecture/runtime.md) |
| Modes | [modes/README.md](modes/README.md) |
| Editing primitives | [editing/README.md](editing/README.md) |
| Glossary | [overview/glossary.md](overview/glossary.md) |

## Related

- Spec index: [README.md](README.md)
- Glossary: [overview/glossary.md](overview/glossary.md)

