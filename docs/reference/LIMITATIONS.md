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
| LIM-REGEX-01 | Regex engine for search and substitution | search_engine and :substitute use regex crate; \v very-magic prefix supported in search | `partial` | `/docs/spec/editing/regex/README.md` | Support nomagic/very-nomagic modes |
| LIM-SCRIPTING-01 | User mappings, commands, functions, events | MappingTable, UserCommandRegistry, EventRegistry implemented; Vimscript functions not implemented | `partial` | `/docs/spec/scripting/README.md` | Implement function definitions |
| LIM-TERMINAL-01 | Managed terminal windows with PTY | Stub service only | `open` | `/docs/spec/features/terminal/README.md` | Implement PTY spawn and IO |
| LIM-SESSION-01 | Session save/load per schema | SessionManager with serialize/deserialize, save/load/delete; :mksession saves buffer paths and cursor positions; :source executes file commands; :call cursor() restores cursor | `partial` | `/docs/spec/features/session/README.md` | Persist window layout in session |
| LIM-LSP-01 | LSP client with completion, diagnostics, etc. | Stub service only | `open` | `/docs/spec/features/lsp/README.md` | Implement LSP client |
| LIM-GIT-01 | Git status, blame, diff integration | Stub returning empty status | `open` | `/docs/spec/features/git/README.md` | Implement git2 or CLI integration |
| LIM-SYNTAX-01 | Syntax highlighting via tree-sitter | Not implemented | `open` | `/docs/spec/features/syntax/README.md` | Add tree-sitter dependency |
| LIM-VISUAL-01 | Visual mode selection and operations | Visual char/line/block selection with motions, operators (d/c/y/>/<), o-swap; Ctrl-V enters block mode; visual selection info (anchor, cursor, kind) included in WindowSnapshot | `partial` | `/docs/spec/modes/visual.md` | Visual block render with column highlighting |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operator+motion, doubled operators, count multiplication, gg motion, f/t/F/T motions in op-pending + ;/, repeat, text objects (iw/aw/ip/ap/iW/aW/is/as/it/at + delimiter objects), mark motions ('/`) all implemented | `partial` | `/docs/spec/editing/operators/README.md` | Remaining: argument, class, function text objects |
| LIM-CMDCOMPL-01 | Command-line Tab completion | Tab/Shift-Tab/BackTab cycle through prefix-matched candidates; file-path, option-name (:set), buffer-name (:b), and user-defined command completion wired | `partial` | `/docs/spec/commands/cmdline/completion.md` | Fuzzy matching, wildmenu display |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named, numbered, small-delete, clipboard, black-hole, expression (=), last-inserted (.), read-only (% : /); put and :registers display | `partial` | `/docs/spec/editing/registers/README.md` | Alternate file (#) register |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local, global, special marks; m{char} sets, '{char}/`{char} jumps; mark motions in op-pending; special marks; changelist navigation (g; / g,) | `partial` | `/docs/spec/editing/marks/README.md` | Jump list (:jumps, Ctrl-O, Ctrl-I) |
| LIM-MACROS-01 | Macro recording and playback | q{a-z} records, q stops, @{a-z} plays back, @@ replays last; q{A-Z} appends; macro-register unification; recursive depth limit (100); error halts playback | `partial` | `/docs/spec/editing/macros/README.md` | Count-prefixed macro playback (3@a) |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Indent/dedent, ~ toggle-case, gu{motion} lowercase, gU{motion} uppercase, gq{motion} format with textwidth+formatoptions; gqq format-line | `partial` | `/docs/spec/editing/text-manipulation/README.md` | formatprg external formatter |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete implemented; snippets, spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Implement snippets engine |
| LIM-CONFIG-01 | Configuration/options system | OptionStore with typed values (bool/int/string), :set/:set no/:set ?/:set all; config file loading with [section] headers (section.key), array values [a,b,c], XDG path | `partial` | `/docs/spec/features/config/README.md` | Modeline parsing, ftplugin integration |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | /pattern/, ?pattern?, '{mark} ranges with offsets, \/ and \? shorthand, ; separator, '<,'> visual range, +N/-N offsets; backwards range (E493) and mark-not-set (E20) errors | `partial` | `/docs/spec/commands/ranges/ranges.md` | Range confirmation prompt for backwards ranges |
| LIM-SEARCHNAV-01 | n/N search navigation, hlsearch, incsearch | SearchEngine with forward/backward, wrapping; n/N wired; hlsearch renders yellow-on-black highlight_ranges via render pipeline; \v very-magic regex in search | `partial` | `/docs/spec/editing/search/README.md` | Incremental search preview, :noh command |

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
