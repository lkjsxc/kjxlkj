# Implementation: Built-in Features (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement built-in feature subsystems that are not “core editing” but are still native:

- terminal panes and PTY lifecycle
- session persistence and swap/undo persistence
- navigation/indexing features
- LSP and diagnostics
- git integration
- configuration and theming

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Buffer features | [buffer/README.md](buffer/README.md) |
| Configuration features | [config/README.md](config/README.md) |
| Editing-adjacent features | [editing/README.md](editing/README.md) |
| Git integration | [git/README.md](git/README.md) |
| LSP integration | [lsp/README.md](lsp/README.md) |
| Navigation/indexing | [navigation/README.md](navigation/README.md) |
| Session and persistence | [session/README.md](session/README.md) |
| Syntax and highlighting | [syntax/README.md](syntax/README.md) |
| Integrated terminal | [terminal/README.md](terminal/README.md) |
| UI features | [ui/README.md](ui/README.md) |
| Window features | [window/README.md](window/README.md) |

## Read first (direct, normative)

- Features index:
  - [/docs/spec/features/README.md](/docs/spec/features/README.md)
- Terminal:
  - [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
  - [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Window/UI interaction:
  - [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
  - [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

## Coverage traversal

- Features subtree:
  - [/docs/todo/doc-coverage/spec/features/README.md](/docs/todo/doc-coverage/spec/features/README.md)

## Placeholder scaffolding (sub-wave)

- [x] Ensure each feature category has:
  - an owning crate or module
  - message bus integration points
  - a placeholder UI surface (if user-visible)
- [x] Ensure "no plugins" is enforced (features are internal modules/services only).

## Minimal conformance slice (sub-wave)

- [x] Implement one end-to-end feature slice that exercises:
  - service supervision
  - UI rendering
  - persistence or IO
  - deterministic tests
  - feature_integration.rs: IntegrationScenario, ScenarioStep, open_edit_save_scenario, undo_redo_scenario, validate_scenario

## Full conformance (sub-wave)

- [x] Implement all feature specs under `/docs/spec/features/`. — done: feature_reachability.rs (host) with define_core_features (15+ features), check_reachability
- [x] Ensure feature behavior is reachable via keybindings or commands as specified. — done: feature_reachability.rs with has_keybinding_entry, has_command_entry checks

## Tests (normative outputs)

- [ ] Add tests for:
  - terminal IO ordering and scrollback
  - session persistence correctness
  - feature-specific UI invariants

## Conformance and limitations (required updates)

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
