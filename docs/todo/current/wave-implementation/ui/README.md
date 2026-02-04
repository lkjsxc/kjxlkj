# Implementation: UI (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the UI model and rendering pipeline:

- UI component model and view composition
- themes and styling
- viewport rendering invariants
- statusline and notifications
- terminal rendering performance expectations

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| UI components | [components/README.md](components/README.md) |
| UI views | [views/README.md](views/README.md) |
| Themes | [themes/README.md](themes/README.md) |
| Viewport model | [viewport/README.md](viewport/README.md) |
| Cursor rendering | [cursor/README.md](cursor/README.md) |
| Statusline | [statusline/README.md](statusline/README.md) |
| Popups/overlays | [popups/README.md](popups/README.md) |
| Notifications | [notifications/README.md](notifications/README.md) |

## Read first (direct, normative)

- UI spec:
  - [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
  - [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
  - [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
  - [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- UI features:
  - [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Cursor rendering:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Coverage traversal

- UI subtree:
  - [/docs/todo/doc-coverage/spec/ui/README.md](/docs/todo/doc-coverage/spec/ui/README.md)
- UI features subtree:
  - [/docs/todo/doc-coverage/spec/features/ui/README.md](/docs/todo/doc-coverage/spec/features/ui/README.md)

## Placeholder scaffolding (sub-wave)

- [x] Define UI snapshot structures and rendering contract.
- [x] Define view composition for:
  - editor window(s)
  - command-line window
  - popups/overlays
  - terminal panes

## Minimal conformance slice (sub-wave)

- [x] Implement a minimal UI that satisfies:
  - cursor is always visible
  - viewport follows cursor deterministically
  - redraws are stable and do not flicker

## Full conformance (sub-wave)

- [ ] Implement all UI spec and UI feature documents.
- [ ] Implement theme and styling support as specified.
- [ ] Implement notification and popup APIs as specified (or record limitations).

## Tests (normative outputs)

- [ ] Add tests for:
  - snapshot → render determinism
  - cursor visibility across modes and overlays
  - viewport invariants across scrolling and edits

## Conformance and limitations (required updates)

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
