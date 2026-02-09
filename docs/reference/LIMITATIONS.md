# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks user-visible gaps between target spec and current implementation.

## Entry Rules

Each limitation includes:

- expected behavior link (`/docs/spec/...`)
- observed behavior in current runtime
- deterministic evidence path
- concrete next action

## Open Limitations

| ID | Expected Behavior | Observed Behavior | Status | Evidence | Next Action |
|---|---|---|---|---|---|
| LIM-01 | [`A` appends at end-of-line](/docs/spec/modes/insert/insert.md) | `Shift+a` can fail because dispatch expects `KeyModifiers::NONE` for `Char('A')` | `open` | `src/crates/kjxlkj-core-mode/src/normal_commands.rs`, `src/crates/kjxlkj-input/src/decode.rs` | Normalize shifted printable keys before Normal-mode matching |
| LIM-02 | [Terminal must be a real managed window](/docs/spec/features/terminal/terminal.md) | `Action::SpawnTerminal` is no-op in editor dispatch path | `open` | `src/crates/kjxlkj-core-state/src/action_dispatch2.rs` | Wire `SpawnTerminal` to create terminal window + PTY session |
| LIM-03 | [Split/window movement must be spatial](/docs/spec/features/window/splits-windows.md) | Window focus/move uses map iteration order, not geometric adjacency | `open` | `src/crates/kjxlkj-core-state/src/editor_window_ops.rs` | Introduce explicit layout graph and directional neighbor resolution |
| LIM-04 | [Explorer must be fully interactive](/docs/spec/features/navigation/file_explorer.md) | Explorer model exists but command/key integration is incomplete | `open` | `src/crates/kjxlkj-core-state/src/file_explorer.rs` | Bind explorer actions into runtime dispatch and rendering |
| LIM-05 | [`:w` and related commands persist real bytes](/docs/spec/commands/file/write-commands.md) | `do_write` only clears modified flag | `open` | `src/crates/kjxlkj-core-state/src/editor_file_ops.rs` | Route writes through fs service with encoding and error handling |
| LIM-06 | [Session save/load commands restore state](/docs/spec/features/session/sessions.md) | `SessionSave` and `SessionLoad` actions are placeholders in dispatch | `open` | `src/crates/kjxlkj-core-state/src/action_dispatch2.rs` | Implement runtime command handlers and persistence integration |
| LIM-07 | [Leader UX expects documented default behaviors](/docs/spec/ux/keybindings.md) | Default leader is `\\`, while docs commonly expect Space-driven chord examples | `accepted-temporary` | `src/crates/kjxlkj-core-state/src/keybinding_dsl.rs` | Add configurable leader default and align docs/examples per profile |
| LIM-08 | [Japanese IME composition must be end-to-end safe](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME model exists, but full terminal composition interception is incomplete | `open` | `src/crates/kjxlkj-core-state/src/ime.rs`, PTY tests are model-level | Integrate composition state into key-routing layer and PTY path |
| LIM-09 | [Terminal emulator should be full-scratch and production-grade](/docs/spec/features/terminal/terminal.md) | PTY and parser modules exist but service/runtime integration is partial | `open` | `src/crates/kjxlkj-service-terminal/src/pty.rs`, `src/crates/kjxlkj-service-terminal/src/escape_parser.rs` | Connect service loop to editor actions and renderer snapshots |
| LIM-10 | [Long lines must always wrap on-screen when wrap is enabled](/docs/spec/features/ui/viewport.md) | Core wrapping primitives exist; full UI path can still miss wrap behavior in some user flows | `partial` | `src/crates/kjxlkj-core-state/src/line_wrap.rs`, boundary tests | Make renderer consume wrap segments for all buffer views by default |

## Priority Order

1. LIM-01, LIM-02, LIM-03, LIM-04
2. LIM-05, LIM-06, LIM-08
3. LIM-09, LIM-10, LIM-07

## Related

- Current implementation claims: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Target specification: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction control plane: [/docs/todo/current/README.md](/docs/todo/current/README.md)
