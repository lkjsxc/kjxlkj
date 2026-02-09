# Canonical Specifications

Back: [/docs/README.md](/docs/README.md)

`/docs/spec/` defines the target system that reconstruction must deliver.

## Authority

- `spec` states what MUST exist in the finished product.
- `reference` states what is currently verified in this repository state.
- When conflicts appear, reconcile via `reference` updates plus explicit TODO items.

## Spec Domains

| Domain | Scope |
|---|---|
| [architecture/](architecture/README.md) | Runtime model, crate topology, startup/shutdown |
| [editor/](editor/README.md) | Buffers, windows, viewport ownership |
| [modes/](modes/README.md) | Normal/Insert/Visual/Replace/Command behavior |
| [editing/](editing/README.md) | Motions, operators, text objects, search, marks, registers |
| [commands/](commands/README.md) | Ex command grammar and execution |
| [features/](features/README.md) | Built-in product features (terminal, explorer, LSP, Git, UI) |
| [scripting/](scripting/README.md) | Mappings, user commands/functions, automation |
| [ui/](ui/README.md) | UI components and rendering surfaces |
| [ux/](ux/README.md) | Keybindings, accessibility, layout expectations |
| [technical/](technical/README.md) | Testing, latency, contracts, memory |
| [overview/](overview/README.md) | Glossary and principles |

## Normative Language

| Term | Meaning |
|---|---|
| `MUST` | Non-optional requirement |
| `MUST NOT` | Forbidden behavior |
| `SHOULD` | Preferred behavior unless justified otherwise |
| `MAY` | Optional behavior |

## High-Risk Areas To Read First

1. [architecture/startup.md](architecture/startup.md)
2. [architecture/runtime.md](architecture/runtime.md)
3. [editing/cursor/README.md](editing/cursor/README.md)
4. [features/ui/viewport.md](features/ui/viewport.md)
5. [features/terminal/terminal.md](features/terminal/terminal.md)
6. [editor/windows.md](editor/windows.md)
7. [modes/insert/input/insert-japanese-ime.md](modes/insert/input/insert-japanese-ime.md)
8. [technical/testing.md](technical/testing.md)

## Relationship To Implementation

Use these documents together:

- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- Verified current behavior: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Known user-visible gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
