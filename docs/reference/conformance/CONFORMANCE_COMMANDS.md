# Conformance: Commands

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Scope

Current verification status for Ex command parsing, dispatch, and execution behavior.

## Ledger

| Requirement ID | Spec Link | Status | Evidence | Notes |
|---|---|---|---|---|
| CMD-001 | `/docs/spec/commands/essential.md` | `verified` | `ex_commands.rs` | `:q`, `:q!`, `:w`, `:wq`, `:x` |
| CMD-002 | `/docs/spec/commands/syntax.md` | `partial` | `ex_commands.rs` + `ex_parse.rs` | Range+command parsing; not all command syntax handled |
| CMD-003 | `/docs/spec/commands/ranges/ranges.md` | `partial` | `ex_parse.rs` + 6 tests | Line numbers, `.`, `$`, `%`, comma, offsets; pattern/mark ranges deferred |
| CMD-004 | `/docs/spec/commands/substitute/substitute-command.md` | `partial` | `ex_commands.rs` | Plain-text `:s/pat/repl/[gine]`; regex deferred (LIM-REGEX-01) |
| CMD-005 | `/docs/spec/commands/buffer/README.md` | `verified` | `ex_commands.rs` | `:bn`, `:bp`, `:b N` |
| CMD-006 | `/docs/spec/commands/file/README.md` | `partial` | `ex_commands.rs` | `:w` (write), `:e` (notification only; async load not wired) |
| CMD-007 | `/docs/spec/commands/cmdline/cmdline-editing.md` | `partial` | `cmdline.rs` + `editor.rs` | Insert, BS, Delete, Left/Right/Home/End, Ctrl-w/u, Up/Down history |
| CMD-008 | `/docs/spec/commands/quit-commands.md` | `verified` | `editor.rs` + `ex_commands.rs` + tests | `:q`, `:q!`, `:wq` with unsaved-buffer guard |
