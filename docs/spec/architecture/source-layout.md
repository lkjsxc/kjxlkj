# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

This blueprint defines the reconstruction target for source topology.

## Goals

- Keep each source directory at around 12 direct children.
- Keep each source file below 200 lines.
- Group modules by runtime responsibility so wiring is traceable.

## Workspace Skeleton

| Path | Direct Children Target | Notes |
|---|---:|---|
| `src/crates/` | 18 | One crate directory per workspace member |
| `src/crates/kjxlkj/src/` | 6-10 | Binary orchestration modules |
| `src/crates/kjxlkj-core-state/src/` | 10-12 | Split by state/dispatch domains |
| `src/crates/kjxlkj-core-edit/src/` | 8-12 | Operators, motions, text objects, helpers |
| `src/crates/kjxlkj-core-mode/src/` | 8-12 | Per-mode dispatch and transitions |
| `src/crates/kjxlkj-render/src/` | 8-12 | Grid, wrapping, decorations, diff |
| `src/crates/kjxlkj-input/src/` | 8-12 | decode, mapping, leader/count, IME gates |
| `src/crates/kjxlkj-service-terminal/src/` | 8-12 | parser, screen, PTY, lifecycle |

## Binary Crate Layout (`kjxlkj/src`)

| Module | Responsibility |
|---|---|
| `main.rs` | startup handoff only |
| `app.rs` | runtime construction and join |
| `channels.rs` | typed channel wiring |
| `services.rs` | service spawn and shutdown hooks |
| `signals.rs` | process signal orchestration |
| `cli.rs` | command-line arguments |

## Core-State Layout (`kjxlkj-core-state/src`)

| Module Group | Required Files |
|---|---|
| state model | `editor.rs`, `buffer_list.rs`, `window_tree.rs`, `session.rs` |
| dispatch | `editor_actions.rs`, `editor_mode_dispatch.rs`, `ex_dispatch.rs` |
| editing ops | `editing_ops_insert.rs`, `editing_ops_modify.rs`, `editing_ops_yank.rs` |
| search and navigation | `search_engine.rs`, `cursor_ops.rs`, `cursor_ops_scroll.rs`, `cursor_ops_findchar.rs` |
| scripting | `mappings.rs`, `user_commands.rs`, `user_functions.rs`, `events.rs` |
| persistence and marks | `registers.rs`, `marks.rs`, `config_loader.rs` |
| tests | `*_tests.rs` split by domain (no monolithic test file) |

## Terminal Service Layout (`kjxlkj-service-terminal/src`)

| Module | Responsibility |
|---|---|
| `task.rs` | service event loop |
| `pty.rs` | process spawn/read/write/resize/cleanup |
| `parser.rs` | VT state machine and UTF-8 integration |
| `csi.rs` | CSI dispatch |
| `osc.rs` | OSC handling |
| `screen.rs` | screen buffer and scrollback |
| `attrs.rs` | SGR attributes and style state |
| `tests_*.rs` | parser/screen/lifecycle unit tests |

## Directory Overflow Procedure

| Trigger | Required Action |
|---|---|
| direct children > 12 | create subdirectory by domain and move related files |
| file length > 200 | extract focused module and keep old file as thin facade |
| mixed concerns in one file | split by input path, state mutation, and IO side effects |

## Verification Rules

- Every split MUST preserve public API behavior.
- Every split MUST include deterministic tests for moved logic.
- TODO entries for topology work MUST link this file directly.

## Related

- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Workflow gates: [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- Reconstruction TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
