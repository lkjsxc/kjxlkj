# Canonical Specifications

Back: [/docs/README.md](/docs/README.md)

`/docs/spec/` defines target product behavior.

## Authority

- `spec` defines what the reconstructed implementation must do.
- `reference` defines what is currently verified.
- `todo` defines execution order and completion gates for reaching spec compliance.

## Intra-Spec Conflict Rule

If two spec files conflict:

1. More specific leaf spec overrides a parent index page.
2. Safety and determinism constraints in `spec/technical/` override convenience behavior.
3. Architecture invariants in `spec/architecture/` override local feature mechanics.
4. Any unresolved conflict MUST be logged in reference limitations and turned into TODO work.

## Spec Domains

| Domain | Scope |
|---|---|
| [architecture/](architecture/README.md) | Runtime, startup, crate topology |
| [editor/](editor/README.md) | Buffer/window/state model |
| [modes/](modes/README.md) | Modal behavior |
| [editing/](editing/README.md) | Motions, operators, text objects, search |
| [commands/](commands/README.md) | Ex command grammar and execution |
| [features/](features/README.md) | Integrated features and services |
| [scripting/](scripting/README.md) | Mappings, user commands/functions, automation |
| [ui/](ui/README.md) | UI model and rendering surfaces |
| [ux/](ux/README.md) | Keybinding and interaction expectations |
| [technical/](technical/README.md) | Testing, latency, contracts, memory |
| [overview/](overview/README.md) | Principles and glossary |

## Normative Language

| Term | Meaning |
|---|---|
| `MUST` | Required |
| `MUST NOT` | Forbidden |
| `SHOULD` | Preferred unless justified |
| `MAY` | Optional |

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Current-state evidence: [/docs/reference/README.md](/docs/reference/README.md)
- Reconstruction controls: [/docs/todo/README.md](/docs/todo/README.md)
