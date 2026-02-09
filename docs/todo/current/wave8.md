# Wave 8 — Visual Block, Special Marks, Session Commands, Config Loading

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Visual block mode, special marks, :mksession/:source,
config file loading, semicolon range separator,
file-path completion, macro/register display unification.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-VBLOCK-01 | Visual block mode (Ctrl-V) with column selection | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-SMARKS-01 | Special marks (. < > [ ]) auto-set | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-SESSION-01 | :mksession and :source commands | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-CONFIG-01 | Load config.toml on startup | `/docs/spec/features/config/README.md` | `[x]` |
| REQ-SEMIRNG-01 | Semicolon range separator | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-FCOMPL-01 | File-path completion in command line | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-REGDISPLAY-01 | :registers command displays macro content | `/docs/spec/editing/registers/README.md` | `[x]` |
| REQ-IW-BIG-01 | iW/aW WORD text objects | `/docs/spec/editing/text-objects/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 199 lines
- LIMITATIONS updated
