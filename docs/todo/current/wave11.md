# Wave 11 — Read-only Registers, Jump List, Range Validation, Session Layout, Block Render, Macro Error Halt, Regex Modes, hlsearch Render

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Wire read-only registers (%, #, :, /), changelist navigation
(g; / g,), range validation errors, session cursor persistence,
block selection rendering hints, macro error halting, very-magic
regex flag, and render pipeline consumption of highlight_ranges.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-ROREG-01 | Read-only registers: % (filename), # (alt file), : (last command), / (last search) | `/docs/spec/editing/registers/special-registers.md` | `[x]` |
| REQ-CHANGELIST-01 | g; / g, navigate changelist | `/docs/spec/editing/marks/changelist.md` | `[x]` |
| REQ-RANGERR-01 | Range validation: backwards range error, mark not set | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
| REQ-SESSCUR-01 | Session saves/restores per-window cursor positions | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-BLOCKREN-01 | Block visual selection info included in snapshot | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-MACERR-01 | Macro playback halts on command failure | `/docs/spec/editing/macros/recursive-macros.md` | `[x]` |
| REQ-VMAGIC-01 | \v very-magic flag in search patterns | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-HLRENDER-01 | Render grid marks search-highlighted cells | `/docs/spec/editing/search/search-highlight.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 200 lines
- LIMITATIONS updated
