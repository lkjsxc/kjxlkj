# Reconstruction UX Requirements

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Scope

Normative UX requirements for rebuild waves and acceptance gating.

## Requirement Matrix

| UX ID | Requirement | Acceptance IDs |
|---|---|---|
| `UX-EDIT-01` | Editor keeps separate synced snapshot and local draft models. | `E2E-06`, `E2E-17` |
| `UX-EDIT-02` | Editor is Obsidian-like markdown-first with preview and wiki-link support. | `E2E-24` |
| `UX-EDIT-03` | Autosave is default and exposes `saving/saved/conflict/offline` states. | `E2E-06`, `E2E-17` |
| `UX-EDIT-04` | Title rename propagates same cycle to notes list and navigation. | `API-NOTE-02`, `E2E-23` |
| `UX-EDIT-05` | Default editor chrome remains low-noise (no inline save/version/delete). | `E2E-24` |
| `UX-NOTE-01` | New note without title defaults to current datetime title. | `API-NOTE-01`, `E2E-23` |
| `UX-LAYOUT-01` | One responsive tree across desktop/mobile. | `E2E-12`, `E2E-19`, `E2E-25` |
| `UX-LAYOUT-02` | Compact menu mode activates at about `<=1280px` and uses top-right toggle. | `E2E-12`, `E2E-25` |
| `UX-LAYOUT-03` | In compact mode, selecting a note closes menu and returns focus to editor. | `E2E-23`, `E2E-25` |
| `UX-LAYOUT-04` | At `320px`, no horizontal scroll in core note workflows. | `E2E-19` |
| `UX-NAV-01` | Note-first baseline; optional modules stay opt-in. | `E2E-23`, `E2E-24` |
| `UX-AGENT-01` | Agent prompts and run behavior are JSON-defined and inspectable in UI/admin. | `API-AUTO-03`, `AGENT-01` |

## Closure Rule

A UX item is closed only when:

1. acceptance IDs pass with deterministic evidence
2. linked TODO wave is checked
3. reference ledgers are synchronized

## Related

- Editor flow: [editor-flow.md](editor-flow.md)
- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Testing: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
