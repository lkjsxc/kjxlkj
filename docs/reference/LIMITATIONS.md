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
| LIM-SCRIPTING-01 | User mappings, commands, functions, events | MappingTable, UserCommandRegistry, EventRegistry implemented; FunctionRegistry with function!/endfunction, parse_function_header, define/get/remove/list; body accumulation via FunctionBodyAcc; :call FuncName(args) invokes user function body lines sequentially; function parameter binding parses args from parentheses and binds to a:param options store | `partial` | `/docs/spec/scripting/README.md` | Local variables, return values |
| LIM-TERMINAL-01 | Managed terminal windows with PTY | Stub service only | `open` | `/docs/spec/features/terminal/README.md` | Implement PTY spawn and IO |
| LIM-SESSION-01 | Session save/load per schema | SessionManager with serialize/deserialize, save/load/delete; :mksession saves buffer paths, cursor positions, window split layout with per-window buffer assignments and per-pane weights (proportional sizes), global marks (A-Z), local marks (a-z) per buffer, editor options, cmdline history, search pattern; :source executes file commands including mark restore, set commands, history/search restore; :call cursor() restores cursor; layout hsplit/vsplit serialize weights as comma-separated floats; local marks stored as localmark lines after each file entry | `partial` | `/docs/spec/features/session/README.md` | Tab page session persistence |
| LIM-LSP-01 | LSP client with completion, diagnostics, etc. | Stub service only | `open` | `/docs/spec/features/lsp/README.md` | Implement LSP client |
| LIM-GIT-01 | Git status, blame, diff integration | Stub returning empty status | `open` | `/docs/spec/features/git/README.md` | Implement git2 or CLI integration |
| LIM-SYNTAX-01 | Syntax highlighting via tree-sitter | Not implemented | `open` | `/docs/spec/features/syntax/README.md` | Add tree-sitter dependency |
| LIM-VISUAL-01 | Visual mode selection and operations | Visual char/line/block selection with motions, operators (d/c/y/>/<), o-swap; Ctrl-V enters block mode; I/A in block mode inserts on each line; block change replicates text to all lines; visual p/P paste with block column-wise paste support; r{char} block/char/line replace; * and # search for selected text; gv reselects last visual area; J joins selected lines; = reindents selection; u lowercases and U uppercases selection; g? applies ROT13 to selection; ~ toggles case of selection; visual selection rendered with column highlighting (blue bg) in grid | `partial` | `/docs/spec/modes/visual.md` | Visual sort, filter operations |
| LIM-OPMOTION-01 | Operator+motion composition (d2w, c$, etc.) | Operator+motion, doubled operators, count multiplication, gg motion, f/t/F/T motions in op-pending + ;/, repeat, text objects (iw/aw/ip/ap/iW/aW/is/as/it/at/ia/aa/ic/ac/if/af + delimiter objects), mark motions ('/`) all implemented; class/function text objects use brace-matching heuristic | `partial` | `/docs/spec/editing/operators/README.md` | Tree-sitter backed class/function objects |
| LIM-CMDCOMPL-01 | Command-line Tab completion | Tab/Shift-Tab/BackTab cycle through prefix-matched candidates with fuzzy fallback; file-path, option-name (:set), buffer-name (:b), and user-defined command completion wired; wildmenu renders completions on status bar row with highlight and scrolling for large lists; history Up/Down filters by typed prefix; PopupMenu struct with items/selected/row/col/max_visible/scroll_offset; render_popup_menu draws menu with dark bg and blue selected highlight in grid | `partial` | `/docs/spec/commands/cmdline/completion.md` | Popup menu keyboard navigation |
| LIM-REGISTERS-01 | Named registers (a-z, A-Z, 0-9, etc.) | RegisterFile supports named, numbered, small-delete, clipboard, black-hole, expression (=), last-inserted (.), read-only (% # : /); Ctrl-R in insert mode inserts register content; Ctrl-R = opens expression mini-cmdline with arithmetic/string evaluation, g:/b:/w:/v: variable references, strlen/line/col/len built-in functions, comparison operators (==, !=, <, >, <=, >=) returning "1"/"0"; ternary operator (cond ? then : else); list literal [1,2,3] pass-through; len() counts list items | `partial` | `/docs/spec/editing/registers/README.md` | Dict literals, type() function |
| LIM-MARKS-01 | Local and global marks | MarkFile supports local, global, special marks; m{char} sets, '{char}/`{char} jumps with cross-buffer switching for global (A-Z) marks; mark motions in op-pending; special marks; changelist navigation (g; / g,); jump list (Ctrl-O, Ctrl-I, :jumps) with cross-buffer switching; global marks persisted in session files; local marks (a-z) serialized per-buffer in session as localmark lines and restored on :source; :marks lists all marks with line/col positions; :delmarks! clears all local marks; :delmarks a-d range notation | `partial` | `/docs/spec/editing/marks/README.md` | Numbered marks, viminfo persistence |
| LIM-MACROS-01 | Macro recording and playback | q{a-z} records, q stops, @{a-z} plays back, @@ replays last; q{A-Z} appends; macro-register unification; sync on yank/delete into named register auto-updates macro store; put from macro register pastes keystroke text; recursive depth limit (100); error halts playback; count-prefixed playback (3@a) | `partial` | `/docs/spec/editing/macros/README.md` | Macro step debugging |
| LIM-TEXTMANIP-01 | Text manipulation (indent, format, case, etc.) | Indent/dedent, ~ toggle-case, gu{motion} lowercase, gU{motion} uppercase, gq{motion} format with textwidth+formatoptions; gqq format-line; formatprg pipes selection through external process via stdin/stdout, replaces range on success, E282 error on spawn failure; formatexpr calls user function via handle_call_function before checking formatprg; :sort command with !/r reverse, i case-insensitive, n numeric, u unique flags | `partial` | `/docs/spec/editing/text-manipulation/README.md` | equalprg option for = operator |
| LIM-EDITHELP-01 | Auto-pairs, comments, snippets, spell, multicursor | Auto-pairs, comment toggle, surround add/delete implemented; SnippetRegistry with add/expand/list/expand_at; snippet expansion parses $1-$9/$0 tab-stops with offset tracking; SnippetSession with current_offset/advance for cursor positioning at first tab-stop; Tab key in insert mode advances snippet session to next tab-stop; spell, multicursor still missing | `partial` | `/docs/spec/features/editing/README.md` | Snippet placeholder editing, mirror tab-stops |
| LIM-CONFIG-01 | Configuration/options system | OptionStore with typed values (bool/int/string), :set/:set no/:set ?/:set all; config file loading with [section] headers (section.key), array values [a,b,c], XDG path; modeline parsing (vim:/vi:/ex: in first/last 5 lines); filetype auto-detection from 30+ file extensions sets `filetype` option on open | `partial` | `/docs/spec/features/config/README.md` | ftplugin integration |
| LIM-ACCESSIBILITY-01 | Screen-reader, high-contrast support | Not implemented | `open` | `/docs/spec/ux/accessibility.md` | Implement accessibility layer |
| LIM-RANGEPATTERN-01 | Pattern-based ranges (/pattern/, ?pattern?) and mark-based ranges | /pattern/, ?pattern?, '{mark} ranges with offsets, \/ and \? shorthand, ; separator, '<,'> visual range, +N/-N offsets; backwards ranges auto-swap start/end; mark-not-set (E20) error; :s///c confirmation prompt (y/n/a/q/l) with cursor positioned at first line of range on entering confirm mode; sub_confirm_advance moves cursor to each subsequent match line | `partial` | `/docs/spec/commands/ranges/ranges.md` | Interactive confirmation prompt for backwards ranges |
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
