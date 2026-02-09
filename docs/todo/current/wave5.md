# Wave 5 — Macros, Command Completion, Pattern Ranges, Options

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Macro recording/playback, command-line completion, pattern-based
range parsing, and basic options system.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-MAC-01 | Macro recording with q{a-z} and q to stop | `/docs/spec/editing/macros/register-macros.md` | `[x]` |
| REQ-MAC-02 | Macro playback with @{a-z} | `/docs/spec/editing/macros/register-macros.md` | `[x]` |
| REQ-MAC-03 | @@ replays last executed macro | `/docs/spec/editing/macros/macros-advanced.md` | `[x]` |
| REQ-MAC-04 | Count prefix on macro playback (10@a) | `/docs/spec/editing/macros/macros-advanced.md` | `[x]` |
| REQ-MAC-05 | Macro stops on error (end of file, etc.) | `/docs/spec/editing/macros/recursive-macros.md` | `[x]` |
| REQ-COMPL-01 | Tab completion for ex command names | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-COMPL-02 | Tab/Shift-Tab cycle through candidates | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-RANGE-01 | /pattern/ range (next matching line) | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-RANGE-02 | ?pattern? range (previous matching line) | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-RANGE-03 | '{mark} range (line of mark) | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-OPT-01 | :set option=value sets options | `/docs/spec/features/config/implementation.md` | `[x]` |
| REQ-OPT-02 | :set nooption unsets boolean options | `/docs/spec/features/config/implementation.md` | `[x]` |
| REQ-OPT-03 | :set option? shows current value | `/docs/spec/features/config/implementation.md` | `[x]` |

## Implementation Plan

1. Add macro state fields to EditorState
2. Create `macros.rs` — recording/playback methods
3. Wire `q` and `@` keys in dispatch and editor_modes
4. Create `cmdline_completion.rs` — Tab completion logic
5. Add completion state to CmdlineHandler
6. Extend `ex_parse.rs` for pattern/mark ranges
7. Create `options.rs` — option storage and :set parsing
8. Wire `:set` in ex_dispatch
9. Add tests

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all new + existing tests
- `cargo clippy` zero warnings
- All files ≤ 199 lines
- CONFORMANCE and LIMITATIONS updated
