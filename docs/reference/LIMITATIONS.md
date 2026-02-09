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
| LIM-VISUAL-01 | Visual mode selection and operations | Mode transition exists; no visual selection rendering | `open` | `/docs/spec/modes/visual.md` | Implement visual selection state |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operators exist standalone; full composition grammar not wired | `open` | `/docs/spec/editing/operators/README.md` | Wire operator-pending state |
| LIM-CMDCOMPL-01 | Command-line Tab completion | History navigation works; completion not implemented | `open` | `/docs/spec/commands/cmdline/completion.md` | Implement completion provider |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named (a-z, A-Z), numbered (0-9), small-delete, expression, read-only, clipboard, last-search, black-hole | `partial` | `/docs/spec/editing/registers/README.md` | Wire register selection in operator-pending |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local (a-z), global (A-Z), special (., ^, [, ], <, >) marks with buffer tracking | `partial` | `/docs/spec/editing/marks/README.md` | Wire mark-based navigation (', `) |
| LIM-MACROS-01 | Macro recording and playback | Not implemented | `open` | `/docs/spec/editing/macros/README.md` | Add macro recording state |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Not implemented | `open` | `/docs/spec/editing/text-manipulation/README.md` | Implement operators |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete implemented; snippets, spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Implement snippets engine |
| LIM-CONFIG-01 | Configuration/options system | Not implemented | `open` | `/docs/spec/features/config/README.md` | Implement option storage |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | Only line numbers, `.`, `$`, `%`, offsets implemented | `open` | `/docs/spec/commands/ranges/ranges.md` | Extend range parser |
| LIM-SEARCHNAV-01 | n/N search navigation, hlsearch, incsearch | SearchEngine with forward/backward, wrapping, case modes, match collection implemented; hlsearch rendering not wired | `partial` | `/docs/spec/editing/search/README.md` | Wire hlsearch to render pipeline |

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
