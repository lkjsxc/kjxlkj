# Views

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

Views are panel-sized components that fill layout regions.

## View Classes

| View | Purpose | Typical Location |
|---|---|---|
| Editor | text editing and navigation | main tiled area |
| Explorer | workspace tree and file operations | side window |
| Finder | fuzzy pickers | floating overlay |
| Diagnostics | problems and search results | bottom panel |
| Git | status and diff views | side or bottom panel |
| Terminal | PTY-backed shell sessions | tiled or floating window |
| Undo tree | history graph and restore | side or floating window |

## Focus Rules

| Rule | Requirement |
|---|---|
| Single focus | exactly one view is focused |
| Focus cycle | `Ctrl-w w` and `Ctrl-w W` include visible non-buffer views |
| Focus memory | regions SHOULD restore last focused view |

## Overflow and Wrapping Policy

| Rule | Requirement |
|---|---|
| No off-screen overflow | visible text MUST remain inside view bounds |
| Editor long lines | use soft-wrap or horizontal scrolling per viewport settings |
| Explorer long labels | wrap to continuation rows in the explorer view |
| Terminal output | wrap to continuation rows with wide-char safety |
| Finder/diagnostics rows | truncate only when explicitly configured; default is wrap |

## View Contracts

| View | Required Behavior |
|---|---|
| Editor | independent viewport per window; deterministic cursor and wrap behavior |
| Explorer | open/reveal/rename/delete wiring through core actions |
| Finder | fuzzy filter with deterministic keyboard navigation |
| Diagnostics | selecting item jumps to exact file position |
| Git | status and diff rendering with explicit state refresh |
| Terminal | PTY lifecycle integrated with shared window tree |
| Undo tree | navigating nodes restores exact historical state |

## Test Requirements

| Layer | Minimum Checks |
|---|---|
| Unit | render behavior and input response types |
| Integration | focus transitions and cross-view intents |
| E2E | window creation, navigation, and persisted effects |

## Related

- Components: [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- Windows and splits: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
