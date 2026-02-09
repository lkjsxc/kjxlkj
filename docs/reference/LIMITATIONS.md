# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records open, user-visible mismatches between current behavior and target spec.

## Baseline Entry

| ID | Expected Behavior | Observed Behavior | Status | Evidence | Next Action |
|---|---|---|---|---|---|
| LIM-BASELINE-01 | Full implementation matches `/docs/spec/` | Partial reconstruction; see scoped limitations below | `open` | `/docs/reference/CONFORMANCE.md` | Continue reconstruction waves |

## Scoped Limitations

| ID | Expected Behavior | Observed Behavior | Status | Spec Link | Next Action |
|---|---|---|---|---|---|
| LIM-REGEX-01 | Regex engine for search and substitution | Substitution uses plain-text matching only | `open` | `/docs/spec/editing/regex/README.md` | Implement regex crate integration |
| LIM-SCRIPTING-01 | User mappings, commands, functions, events | MappingTable, UserCommandRegistry, EventRegistry implemented; Vimscript functions not implemented | `partial` | `/docs/spec/scripting/README.md` | Implement function definitions |
| LIM-TERMINAL-01 | Managed terminal windows with PTY | Stub service only | `open` | `/docs/spec/features/terminal/README.md` | Implement PTY spawn and IO |
| LIM-SESSION-01 | Session save/load per schema | SessionManager with serialize/deserialize, save/load/delete, session listing implemented | `partial` | `/docs/spec/features/session/README.md` | Wire :mksession/:source commands |
| LIM-LSP-01 | LSP client with completion, diagnostics, etc. | Stub service only | `open` | `/docs/spec/features/lsp/README.md` | Implement LSP client |
| LIM-GIT-01 | Git status, blame, diff integration | Stub returning empty status | `open` | `/docs/spec/features/git/README.md` | Implement git2 or CLI integration |
| LIM-SYNTAX-01 | Syntax highlighting via tree-sitter | Not implemented | `open` | `/docs/spec/features/syntax/README.md` | Add tree-sitter dependency |
| LIM-VISUAL-01 | Visual mode selection and operations | Visual char/line selection with motions, operators (d/c/y/>/<), o-swap; block mode selection rendering not yet wired | `partial` | `/docs/spec/modes/visual.md` | Implement block selection, visual highlighting in render |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operator+motion, doubled operators (dd/cc/yy/>>/<<), count multiplication, gg motion in op-pending implemented; text objects and f/t motions not yet wired | `partial` | `/docs/spec/editing/operators/README.md` | Wire text objects (iw/aw/ip/ap) and f/t motions |
| LIM-CMDCOMPL-01 | Command-line Tab completion | Tab/Shift-Tab/BackTab cycle through prefix-matched command name candidates; file/option/buffer completion not yet wired | `partial` | `/docs/spec/commands/cmdline/completion.md` | Add file path and option completion contexts |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named, numbered, small-delete, clipboard, black-hole; " prefix wired to select register for next operation | `partial` | `/docs/spec/editing/registers/README.md` | Wire selected register into yank/delete/put operations |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local, global, special marks; m{char} sets marks, '{char}/`{char} jumps to marks | `partial` | `/docs/spec/editing/marks/README.md` | Wire mark jumps as motions in operator-pending |
| LIM-MACROS-01 | Macro recording and playback | q{a-z} records, q stops, @{a-z} plays back, @@ replays last; separate macro_store (not shared with text registers yet) | `partial` | `/docs/spec/editing/macros/README.md` | Share macro storage with register file; support q{A-Z} append |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Indent/dedent line ranges implemented (>>/<< and operator+motion); case/format operators not yet wired | `partial` | `/docs/spec/editing/text-manipulation/README.md` | Implement case change and format operators |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete implemented; snippets, spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Implement snippets engine |
| LIM-CONFIG-01 | Configuration/options system | OptionStore with typed values (bool/int/string), :set/:set no/:set ?/:set all implemented; config file loading not yet wired | `partial` | `/docs/spec/features/config/README.md` | Wire config.toml loading on startup |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | /pattern/, ?pattern?, '{mark} ranges with offsets implemented; \/ and \? shorthand not yet supported | `partial` | `/docs/spec/commands/ranges/ranges.md` | Add \/ and \? last-search shorthand |
| LIM-SEARCHNAV-01 | n/N search navigation, hlsearch, incsearch | SearchEngine with forward/backward, wrapping, case modes; n/N wired to dispatch; hlsearch rendering not wired | `partial` | `/docs/spec/editing/search/README.md` | Wire hlsearch to render pipeline |

## Entry Rules

Each limitation entry MUST include:

- link to expected behavior (`/docs/spec/...`)
- concise observed behavior statement
- deterministic evidence pointer
- concrete next action

## Lifecycle Rules

- Close or update limitations in the same change that modifies behavior.
- Do not remove a limitation without evidence that the gap is closed.
- If behavior is intentionally deferred, keep limitation open with dated rationale.

## Related

- Current-state claims: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Reconstruction gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
