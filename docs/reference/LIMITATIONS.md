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
| LIM-REGEX-01 | Regex engine for search and substitution | search_engine and :substitute use regex crate; \v very-magic, \m magic, \M nomagic, \V very-nomagic prefix modes supported in search; magic mode \| branch alternation via regex_translate; lookahead/lookbehind (\@=, \@!, \@<=, \@<!) translated to Rust regex; variable-length lookbehind documented as unsupported (Rust regex crate limit), \zs workaround noted | `partial` | `/docs/spec/editing/regex/README.md` | Atomic groups, possessive quantifiers |
| LIM-SCRIPTING-01 | User mappings, commands, functions, events | MappingTable, UserCommandRegistry, EventRegistry implemented; FunctionRegistry with function!/endfunction; :call FuncName(args) with parameter binding to a:param; :let l:var = expr for local variables; :return expr stops execution and returns value; s: script-local variable namespace; function("name") references; autoload resolution via # separator (ns#Func resolves to Func); option store lookup for variable resolution | `partial` | `/docs/spec/scripting/README.md` | Script sourcing from files |
| LIM-TERMINAL-01 | Managed terminal windows with PTY | Stub service only | `open` | `/docs/spec/features/terminal/README.md` | Implement PTY spawn and IO |
| LIM-SESSION-01 | Session save/load per schema | SessionManager with serialize/deserialize, save/load/delete; :mksession saves buffer paths, cursor positions, window split layout with per-pane weights, global marks (A-Z), local marks (a-z) per buffer; tab page count and active tab serialized as `tabs N A` line; per-tab window layouts serialized as `tablayout` entries; per-tab buffer associations serialized as `tabbuf` entries; :source restores all state; layout hsplit/vsplit weights as comma-separated floats | `partial` | `/docs/spec/features/session/README.md` | Arglist persistence in sessions |
| LIM-LSP-01 | LSP client with completion, diagnostics, etc. | Stub service only | `open` | `/docs/spec/features/lsp/README.md` | Implement LSP client |
| LIM-GIT-01 | Git status, blame, diff integration | Stub returning empty status | `open` | `/docs/spec/features/git/README.md` | Implement git2 or CLI integration |
| LIM-SYNTAX-01 | Syntax highlighting via tree-sitter | Not implemented | `open` | `/docs/spec/features/syntax/README.md` | Add tree-sitter dependency |
| LIM-VISUAL-01 | Visual mode selection and operations | Visual char/line/block selection with motions, operators (d/c/y/>/<), o-swap; Ctrl-V enters block mode; I/A in block mode inserts on each line; block change replicates text to all lines; visual p/P paste with block column-wise paste support; r{char} block/char/line replace; * and # search for selected text; gv reselects last visual area; J joins selected lines; = reindents selection; u lowercases and U uppercases selection; g? applies ROT13 to selection; ~ toggles case of selection; visual selection rendered with column highlighting (blue bg) in grid | `partial` | `/docs/spec/modes/visual.md` | Visual sort, filter operations |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operator+motion, doubled operators, count multiplication, gg motion, f/t/F/T motions in op-pending + ;/, repeat, text objects (iw/aw/ip/ap/iW/aW/is/as/it/at/ia/aa/ic/ac/if/af + delimiter objects), mark motions ('/`) all implemented; class/function text objects use brace-matching heuristic | `partial` | `/docs/spec/editing/operators/README.md` | Tree-sitter backed class/function objects |
| LIM-CMDCOMPL-01 | Command-line Tab completion | Tab/Shift-Tab/BackTab cycle through prefix-matched candidates with fuzzy fallback; Ctrl-N/Ctrl-P navigate popup menu items; Enter on popup selection inserts candidate and stays in command mode; file-path, option-name, buffer-name, user-command, mark-name, register-name, help-topic completion; wildmenu renders completions on status bar row; PopupMenu with row/col/max_visible/scroll_offset; render_popup_menu draws menu with dark bg and blue selected highlight | `partial` | `/docs/spec/commands/cmdline/completion.md` | Filetype-specific completion |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named, numbered, small-delete, clipboard, black-hole, expression (=), last-inserted (.), read-only (% # : /); Ctrl-R = opens expression mini-cmdline; list literal [1,2,3] and dict literal {"k":"v"} pass-through; len() counts list items; type() returns "0"=number, "1"=string, "3"=list, "4"=dict; dict["key"] access; has_key(dict, "key") returns 0/1; keys(dict) returns list of keys; values(dict) returns list of values; ternary operator, comparison operators | `partial` | `/docs/spec/editing/registers/README.md` | map(), filter(), extend() functions |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local, global, special marks; m{char} sets, '{char}/`{char} jumps; numbered marks 0-9 auto-rotate on jump via rotate_numbered(); mark 0 = position before last jump, 1-9 = previous positions; viminfo serialization (serialize_viminfo/load_viminfo) for global mark persistence across sessions; auto-save viminfo on quit (save_viminfo called before quit_requested); auto-load viminfo on startup (load_viminfo_file in constructor); changelist, jump list, global marks in sessions, local marks per-buffer in sessions; :marks lists, :delmarks range notation | `partial` | `/docs/spec/editing/marks/README.md` | Viminfo merge on concurrent sessions |
| LIM-MACROS-01 | Macro recording and playback | q{a-z} records, q stops, @{a-z} plays back, @@ replays last; q{A-Z} appends; macro-register unification; sync on yank/delete into named register auto-updates macro store; put from macro register pastes keystroke text; recursive depth limit (100); error halts playback; count-prefixed playback (3@a) | `partial` | `/docs/spec/editing/macros/README.md` | Macro step debugging |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Indent/dedent, ~ toggle-case, gu/gU, gq format with textwidth+formatoptions; formatprg pipes through external process; formatexpr calls user function; :sort with !/r/i/n/u flags; equalprg option for = operator; K command looks up keyword under cursor using keywordprg option (default: man) with count as section argument; word_under_cursor extraction | `partial` | `/docs/spec/editing/text-manipulation/README.md` | Visual-range K command |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete; SnippetRegistry with add/expand/list/expand_at; snippet expansion parses $1-$9/$0 and ${N:default} placeholders with default text; mirror tab-stops (${1} after ${1:default}) repeat default text at multiple positions; nested placeholders (${1:outer ${2:inner}}) with recursive parsing; SnippetSession with tab-stop navigation; Tab key advances snippet session; spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Snippet choice nodes |
| LIM-CONFIG-01 | Configuration/options system | OptionStore with typed values (bool/int/string), :set/:set no/:set ?/:set all; config file loading with [section] headers (section.key), array values [a,b,c], XDG path; modeline parsing (vim:/vi:/ex: in first/last 5 lines); filetype auto-detection from 30+ file extensions sets `filetype` option on open | `partial` | `/docs/spec/features/config/README.md` | ftplugin integration |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | /pattern/, ?pattern?, '{mark} ranges with offsets, \/ and \? shorthand, ; separator, '<,'> visual range, +N/-N offsets; (expr) expression address evaluates arithmetic with function calls (strlen, len, line, col) to line number; backwards ranges auto-swap with "Backwards range corrected" notification; :s///c confirmation prompt with cursor positioned at first line; sub_confirm_advance moves cursor | `partial` | `/docs/spec/commands/ranges/ranges.md` | Range variables and user functions |
| LIM-SEARCHNAV-01 | n/N search navigation, hlsearch, incsearch | SearchEngine with forward/backward, wrapping; n/N wired; hlsearch renders yellow-on-black highlight_ranges via render pipeline; \v very-magic regex in search; :noh/:nohlsearch clears highlight; incremental search highlights first match while typing; [N/M] search match count displayed on cmdline row with regex support for \v patterns; \n multi-line search in full-text and highlight paths; search offsets /pattern/e, /pattern/s+N, /pattern/+N and ?pattern?e backward offset parsed and applied via apply_search_offset; search highlights also shown during operator-pending mode when search pattern exists | `partial` | `/docs/spec/editing/search/README.md` | Regex-aware search highlighting with capture groups |

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
