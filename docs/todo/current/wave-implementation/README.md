# Wave: Implementation (Iteration 33)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Implement the planned surface, starting with placeholders and steadily replacing them with fully specified, fully tested behavior.

This wave is expected to be the largest and to be executed as many sub-waves as needed.

## Work rules (normative)

- Implement only what is specified.
- For each shipped behavior change:
  - add/extend tests first or alongside
  - update conformance and limitations docs
  - keep policy constraints satisfied (structure, naming, docs fences)

## Entry points (recursive)

| Area | Checklist |
|---|---|
| Architecture | [architecture/README.md](architecture/README.md) |
| Editor core | [editor/README.md](editor/README.md) |
| Modes | [modes/README.md](modes/README.md) |
| Editing | [editing/README.md](editing/README.md) |
| Ex commands | [commands/README.md](commands/README.md) |
| Built-in features | [features/README.md](features/README.md) |
| Scripting/automation | [scripting/README.md](scripting/README.md) |
| UI model + rendering | [ui/README.md](ui/README.md) |
| UX + keybindings | [ux/README.md](ux/README.md) |
| Technical requirements | [technical/README.md](technical/README.md) |

## Implementation approach (multiple sub-waves)

Each area is executed in three sub-waves:

1. Placeholder scaffolding: compile-time structure only (types, traits, stubs).
2. Minimal conformance slice: implement a small, test-backed subset that is fully specified.
3. Full conformance: implement the entire spec subtree, with tests and conformance updates.

## Definition of done (for any leaf)

- The behavior is defined by direct links to spec documents.
- Acceptance criteria exist (Given/When/Then).
- Tests exist and are deterministic.
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) is updated.
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) is updated when user-visible.
- No doc policy violations are introduced.

## Reference anchors

- Canonical spec index: [/docs/spec/README.md](/docs/spec/README.md)
- Architecture crates/topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Runtime ordering: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
