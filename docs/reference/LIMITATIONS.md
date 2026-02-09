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
| LIM-REGEX-01 | Regex engine for search and substitution | search_engine and :substitute use regex crate via vim-magic→rust translation; fallback to plain text on invalid patterns | `partial` | `/docs/spec/editing/regex/README.md` | Support very-magic/nomagic/very-nomagic modes |
| LIM-SCRIPTING-01 | User mappings, commands, functions, events | MappingTable, UserCommandRegistry, EventRegistry implemented; Vimscript functions not implemented | `partial` | `/docs/spec/scripting/README.md` | Implement function definitions |
| LIM-TERMINAL-01 | Managed terminal windows with PTY | Stub service only | `open` | `/docs/spec/features/terminal/README.md` | Implement PTY spawn and IO |
| LIM-SESSION-01 | Session save/load per schema | SessionManager with serialize/deserialize, save/load/delete, session listing; :mksession writes buffer paths, :source reads and executes file commands | `partial` | `/docs/spec/features/session/README.md` | Persist window layout and cursor positions in session |
| LIM-LSP-01 | LSP client with completion, diagnostics, etc. | Stub service only | `open` | `/docs/spec/features/lsp/README.md` | Implement LSP client |
| LIM-GIT-01 | Git status, blame, diff integration | Stub returning empty status | `open` | `/docs/spec/features/git/README.md` | Implement git2 or CLI integration |
| LIM-SYNTAX-01 | Syntax highlighting via tree-sitter | Not implemented | `open` | `/docs/spec/features/syntax/README.md` | Add tree-sitter dependency |
| LIM-VISUAL-01 | Visual mode selection and operations | Visual char/line/block selection with motions, operators (d/c/y/>/<), o-swap; Ctrl-V enters block mode with column-wise d/c/y operations | `partial` | `/docs/spec/modes/visual.md` | Wire block selection rendering in render pipeline |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operator+motion, doubled operators, count multiplication, gg motion, f/t/F/T motions in op-pending + ;/, repeat, text objects (iw/aw/ip/ap/iW/aW/is/as/it/at + delimiter objects), mark motions ('/`) all implemented | `partial` | `/docs/spec/editing/operators/README.md` | Remaining: argument, class, function text objects |
| LIM-CMDCOMPL-01 | Command-line Tab completion | Tab/Shift-Tab/BackTab cycle through prefix-matched candidates; file-path, option-name (:set), buffer-name (:b), and user-defined command completion wired | `partial` | `/docs/spec/commands/cmdline/completion.md` | Fuzzy matching, wildmenu display |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named, numbered, small-delete, clipboard, black-hole; " prefix selects register; yank/delete/put; numbered rotation; expression register (= with arithmetic/string eval); last-inserted text (.) register | `partial` | `/docs/spec/editing/registers/README.md` | Read-only registers (%, #, :, /) |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local, global, special marks; m{char} sets, '{char}/`{char} jumps; mark motions in op-pending; special marks: . ^ on Insert exit, < > on Visual exit, [ ] on change/delete/yank operations | `partial` | `/docs/spec/editing/marks/README.md` | Jump list navigation (g; / g,) |
| LIM-MACROS-01 | Macro recording and playback | q{a-z} records, q stops, @{a-z} plays back, @@ replays last; q{A-Z} appends; macro-register unification; recursive depth limit (100) | `partial` | `/docs/spec/editing/macros/README.md` | Macro-triggered error halts playback |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Indent/dedent, ~ toggle-case, gu{motion} lowercase, gU{motion} uppercase, gq{motion} format with textwidth+formatoptions; gqq format-line | `partial` | `/docs/spec/editing/text-manipulation/README.md` | formatprg external formatter |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete implemented; snippets, spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Implement snippets engine |
| LIM-CONFIG-01 | Configuration/options system | OptionStore with typed values (bool/int/string), :set/:set no/:set ?/:set all; config file loading with [section] headers (section.key), array values [a,b,c], XDG path | `partial` | `/docs/spec/features/config/README.md` | Modeline parsing, ftplugin integration |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | /pattern/, ?pattern?, '{mark} ranges with offsets, \/ and \? last-search shorthand, semicolon (;) range separator, '<,'> visual range, bare +N/-N offset addresses | `partial` | `/docs/spec/commands/ranges/ranges.md` | Range validation error messages |
| LIM-SEARCHNAV-01 | n/N search navigation, hlsearch, incsearch | SearchEngine with forward/backward, wrapping, case modes; n/N wired; hlsearch populates highlight_ranges in render snapshot when active | `partial` | `/docs/spec/editing/search/README.md` | Render pipeline consumption of highlight_ranges |

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
