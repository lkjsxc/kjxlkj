# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

Active control surface for reconstruction waves. All items use checklist semantics (`- [ ]` pending, `- [x]` verified complete). Implementation MUST follow [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md).

## Rules

- A `- [x]` item MUST have passing tests exercising real user paths.
- A `- [x]` item MUST be reachable from the binary's `main` function.
- Conformance and limitations MUST be updated in the same change.
- Deferred items MUST record rationale in `/docs/log/proposals/`.

## Implementation areas

| Area | Checklist | Primary specs |
|---|---|---|
| Architecture | [architecture.md](architecture.md) | [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md) |
| Editor core | [editor-core.md](editor-core.md) | [/docs/spec/editor/README.md](/docs/spec/editor/README.md) |
| Modes | [modes.md](modes.md) | [/docs/spec/modes/README.md](/docs/spec/modes/README.md) |
| Editing | [editing.md](editing.md) | [/docs/spec/editing/README.md](/docs/spec/editing/README.md) |
| Commands | [commands.md](commands.md) | [/docs/spec/commands/README.md](/docs/spec/commands/README.md) |
| Features: core | [features-core.md](features-core.md) | Terminal, session, window specs |
| Features: services | [features-services.md](features-services.md) | Git, LSP, navigation, syntax specs |
| Features: editing | [features-editing.md](features-editing.md) | Auto-pairs, completion, snippets, etc. |
| Scripting | [scripting.md](scripting.md) | [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md) |
| UI and rendering | [ui-rendering.md](ui-rendering.md) | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) |
| UX | [ux.md](ux.md) | [/docs/spec/ux/README.md](/docs/spec/ux/README.md) |
| Technical | [technical.md](technical.md) | [/docs/spec/technical/README.md](/docs/spec/technical/README.md) |

## Verification

| Gate | Checklist |
|---|---|
| Verification | [verification.md](verification.md) |

## Completion handshake

When all implementation areas reach full conformance and verification is green, invoke `Ask` for the next objective.
