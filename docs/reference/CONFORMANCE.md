# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports the strongest verified state as of the snapshot date.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior exists but reachability or evidence quality is incomplete |
| `blocked` | high-severity mismatch is known and not yet closed |
| `unverified` | no trustworthy evidence currently exists |

## Current Snapshot (2026-02-11)

Workspace reconstructed with 20 crates. Runtime conformance is partially verified
through 538 deterministic unit and integration tests covering key normalization,
mode dispatch, cursor motion, text buffer operations, layout tree, editor state,
multi-key sequences, operator composition, motion execution, motion type
classification, case operators, g-prefix operator dispatch, register system,
force modifiers, count multiplication, Vim regex compilation, ex command parsing,
search forward/backward with wrapping, command-line input handling, blackhole
register suppression, clipboard register stubs, \c/\C case sensitivity flags,
\o/\O octal and \H non-head atoms, put (p/P) paste operations, register-wired
yank/delete operators, cursor boundary clamping, and window command dispatch.
Multi-task runtime architecture implemented (input/core/render tasks with bounded
channels, signal handlers, proper shutdown).
All source files comply with ≤ 200 line limit.
Motion system expanded to ~40 variants including find/till/paragraph/match-paren.
Operator enum expanded to 11 variants (Delete, Change, Yank, Indent, Dedent,
Reindent, Format, Lowercase, Uppercase, ToggleCase, Filter).
Operator composition implemented (linewise dd/yy/cc/guu/gUU/g~~/gqq,
operator+motion d3w/cw, D/Y/gJ special forms).
RangeType/Inclusivity classification system for motions implemented and tested.
Case transform operators (gu/gU/g~) on lines and ranges implemented.
PendingState system for multi-key normal mode sequences (count, g/z/f/t/r/m).
RegisterStore with named (a-z), numbered (0-9), unnamed, and small-delete
registers; yank records to unnamed+0, delete rotates 1-9 for linewise or writes
small-delete for non-linewise; A-Z append supported. Blackhole register ("_)
suppresses all register writes. Clipboard registers ("+, "*) store locally
(real clipboard integration deferred).
ForceModifier enum (Characterwise, Linewise, Blockwise) for v/V/Ctrl-v between
operator and motion in operator-pending mode.
Pre-operator count multiplication (e.g. 2d3w → count 6).
Dot-repeat recording via last_change tracking in EditorState.
Vim regex compiler translating magic-mode patterns to Rust regex (shortcut atoms,
word boundaries, grouping, alternation, quantifiers, \v very-magic switch, \c/\C
case sensitivity flags, \o/\O octal atoms, \H non-head atom, \= synonym for \?).
Ex command parser with abbreviation-based dispatch and ! force flag support.
Search system with forward/backward wrapping and compiled Vim regex patterns.
Command-line input handling for :, /, ? prefixes with mode transitions.
Put operations (p/P) paste from effective register with linewise/characterwise
handling. Operators wired to RegisterStore for yank/delete recording. Cursor
boundary clamping for post-edit safety.
Read-only registers: `"."` (last insert text), `"%"` (current filename),
`":"` (last ex command), `"/"` (last search pattern).
Insert-text session tracking for dot register. `:registers`/`:reg`/`:display`
command parsed. Last-command and last-search register wiring.
Star search (`*`) and hash search (`#`) for word under cursor with word-boundary
matching. `:nohlsearch`/`:noh` clears search highlighting. `hlsearch` state
tracks whether matches should be highlighted; new search re-enables highlight.
`match_count()` for total match reporting. `word_at()` word-under-cursor
extraction with comprehensive boundary tests. Search integration tests cover
multiline wrapping, empty buffer, non-word cursor, and search register wiring.
g*/g# partial match star search (no word boundaries). Search history tracking
with deduplication (capped at 100). ignorecase/smartcase settings with in-pattern
\c/\C override. Bracket matching (%) scans forward on current line when cursor
is not on a bracket character. bracket_pair() helper for bracket-type lookup.
Ctrl-a/Ctrl-x increment/decrement numbers under/after cursor with forward scan,
negative number and multi-digit support. :set/:se/:setlocal command for
ignorecase/smartcase/hlsearch options with no-prefix and key=value parsing.
Stage-03 exit integration tests: ex command pipeline (bn/bp/bd), insert text
persistence, d$ operator+motion, search forward with /pattern and n, :set
ignorecase toggle, star search pattern, yy register recording, diw text object,
Ctrl-a increment, bfirst/blast navigation, visual yank, bracket match (%).
Text objects (iw/aw/iW/aW, i(/a(/i{/a{/i[/a[/i</a</i>/a>, i"/a"/i'/a'/i`/a`)
with word, bracket (nesting-aware, multiline), and quote range computation.
Operator-pending text object dispatch via 'i'/'a' prefix keys.
Paragraph text objects (ip/ap) with contiguous non-blank line detection and
trailing blank inclusion for around variant. Sentence text objects (is/as) with
period/exclamation/question boundary detection and trailing whitespace inclusion.
Tree-sitter text objects (ic/ac, if/af) and tag objects (it/at) deferred.
Visual mode (v charwise, V linewise, Ctrl-v blockwise stub) with anchor/cursor
selection model, sub-mode switching, operator dispatch (d/x/y/c/s/>/</~/u/U/J/p),
anchor swap (o), and Escape exit. Blockwise visual operations delegate to
charwise (full block ops deferred).
Buffer management: alternate buffer tracking (Ctrl-^/Ctrl-6), :bn/:bp/:bd/:b N
buffer navigation, :ls/:buffers listing, :e file opening with dedup, buffer
deletion with alternate/next fallback, deterministic sorted buffer ordering,
:bfirst/:bf and :blast/:bl first/last buffer navigation. 13 integration tests
covering buffer cycling, alternate toggle, deletion fallback, Ctrl-6 via
handle_key, ex command parsing, and boundary conditions.
Race and boundary validation: 10 stress tests (rapid mode toggle 100 cycles,
visual toggle 100 cycles, command mode enter/exit 100 cycles, insert escape
preserves text, split-close cycle 10 times, buffer create-delete cycle 20,
alternate buffer stress 50 cycles, resize boundary 1x1, resize boundary large,
resize churn 50 cycles) and 7 boundary safety tests (deterministic replay
insert-delete, delete on empty buffer, motion on empty buffer, unknown ex
command is noop, sequential ex commands, Ctrl-6 without alternate, force quit
flag).
Window command dispatch (Ctrl-w prefix): PartialKey::WinCmd enables two-key
Ctrl-w sequences. Directional focus (h/j/k/l) with geometry-based resolution,
focus cycle (w), focus previous (p), focus top-left (t), focus bottom-right (b),
split horizontal (s/n), split vertical (v), close window (c/q), window only (o),
resize (+/-/>/<, no-op placeholder), equalize (=), maximize height/width (_/|,
no-op placeholder). Explorer routing: open_explorer creates ContentKind::Explorer
leaf, close_explorer removes it, :ExplorerClose ex command. Split semantics
corrected: :split creates top/bottom layout, :vsplit creates side-by-side layout.
15 unit tests + 26 integration tests covering all wincmd paths.
Explorer state model: ExplorerState with root_path, tree, expansion_set,
selected_index, cached visible rows, NodeId-based identity. ExplorerNode tree
with find/parent_of/sort_children. ExplorerAction enum (MoveDown/MoveUp/
CollapseOrParent/ExpandOrOpen/Toggle/Close). Explorer key routing intercepts
j/k/h/l/Enter/o/v/s/q in Normal mode on explorer-focused windows; v/s open
selected file in vertical/horizontal split. 13 explorer service unit tests + 5
editor_explorer unit tests + 27 integration tests.
Terminal state model: TerminalState with id, shell, title, exited, exit_code,
cols, rows. Terminal window creation via open_terminal (ContentKind::Terminal leaf
in layout tree). TerminalService stub. 2 terminal service unit tests + 5
integration tests.
Wincmd expanded: W reverse cycle, H/J/K/L move-to-edge (placeholder), r/R rotate
(placeholder), x exchange (placeholder). Focus cycle reverse implemented.
21 wincmd unit tests + 40 wincmd integration tests.
Jumplist navigation (Ctrl-o/Ctrl-i) and changelist navigation (g;/g,) with
PositionList data structure (100 entry cap, go_older/go_newer/push, dedup).
Jump recording on GotoLine/GotoFirstLine/GotoLastLine/Search/StarSearch.
Change recording on all text-changing actions.
Boundary tests: empty list, past-end, single window, explorer close, terminal open.
Mark system: m{a-z} set mark at cursor, '{a-z} goto mark line (first non-blank),
`{a-z} goto mark exact position. MarkStore with HashMap<char, MarkPos>, lowercase
a-z only (uppercase ignored). Marks persist across mode changes. Goto unset mark
is no-op. Buffer-bounds clamping on goto when lines deleted. 5 unit tests + 12
integration tests.
Macro recording and playback: q{a-z} starts recording into register, subsequent q
stops recording and saves to register. @{a-z} plays macro by replaying captured
key sequence. MacroState tracks recording flag, register, and key buffer.
Uppercase register rejected. Keys serialized via keys_to_string/parse_macro_keys
with support for Ctrl, Escape, Backspace, Enter, Tab. Macro capture hooks into
handle_key pipeline before dispatch; stop-q intercept in Normal mode. Race
validation with 15 integration tests covering record/play, insert mode recording,
unset register playback, overwrite, uppercase rejection, mode switch stress, mark
+ macro interaction, jumplist + macro interaction, split interaction, rapid
record/stop cycles (100×), deterministic replay, combined mark/split/jumplist
stress, mode churn with marks (50×), changelist stress (20 deletes + navigate).
5 unit tests + 15 integration tests.
Fold commands (z-prefix): zo open fold, zc close fold, za toggle fold, zR open all
folds, zM close all folds, zr reduce fold level, zm increase fold level, zj jump
to next closed fold, zk jump to previous closed fold. FoldState with indent-based
fold computation (compute_indent_folds), FoldRegion tracking, fold_level with
reduce/more, open_all/close_all/toggle per-line. 9 z-prefix unit tests + 6
FoldState unit tests + 16 integration tests (fold dispatch, navigation, empty
buffer safety, macro+fold interaction, mark+fold interaction, reduce/more cycle,
combined stress 20x). Tree-sitter and expression fold methods deferred (only
indent-based implemented).
Terminal escape parser: VT100/xterm state machine with 13 parser states (Ground,
Escape, EscapeIntermediate, CsiEntry, CsiParam, CsiIntermediate, CsiIgnore,
OscString, DcsEntry, DcsParam, DcsPassthrough, DcsIgnore, SosPmApc). CSI dispatch
for cursor movement (CUU/CUD/CUF/CUB/CUP/CNL/CPL/CHA/VPA), erase (ED/EL/ECH),
scroll (SU/SD), line insert/delete (IL/DL), char insert/delete (ICH/DCH),
scroll region (DECSTBM), cursor save/restore. SGR attribute dispatch with basic
8 + bright 8 + 256-color + RGB for fg/bg, bold/dim/italic/underline/reverse/
strikethrough. Private mode dispatch for DECTCEM (cursor visibility), alternate
screen (47/1049), bracketed paste (2004). OSC title handling (0;/2; prefix).
Escape dispatch for reverse index (M), line feed (D), next line (E), cursor
save/restore (7/8), full reset (c). UTF-8 multi-byte accumulation and replacement
character fallback. Screen model with cell grid (char + fg/bg Color + 6 style
attributes), cursor position, saved cursor, scroll region, title, alt-screen
flag, bracketed-paste flag. Screen operations: put_char with line wrap, linefeed
with scroll, carriage return, backspace, tab (8-col stops), reverse index, erase
display (below/above/all/scrollback), erase line (right/left/all), erase/insert/
delete chars, scroll up/down, insert/delete lines, cursor save/restore, reset.
Filetype detection: extension-based language mapping for 15 languages (rust,
python, javascript, typescript, go, c, cpp, markdown, json, yaml, toml, html,
css, bash, lua) with shebang fallback (python, node, bash, lua).
LSP lifecycle model: LspServerState with phase machine (Starting/Initializing/
Running/ShuttingDown/Stopped/Failed), ServerCapabilities (17 boolean fields),
LspServerConfig (language/command/root_markers/filetypes), crash tracking with
3-retry limit, restart reset. Diagnostic model: DiagnosticStore with
replace_for_file (LSP push semantics), append (incremental), alloc_id auto-
increment, sort by severity→file→line→col, next_in_file/prev_in_file wrapping
navigation, count_by_severity, for_file filter. Severity enum (Error/Warning/
Info/Hint, ordered), DiagnosticKind (Diagnostic/Build/Grep/Todo/Quickfix),
DiagnosticLocation with optional end position. Theme/highlight model: HlGroup
enum (35 groups: 13 syntax + 22 UI), Color(u8,u8,u8) RGB, Style with builder
pattern (fg/bg/bold/italic/underline/strikethrough/reverse), Theme with
HashMap<HlGroup,Style>, default_dark() One Dark inspired with 30 styled groups.
Git sign state model: GitSignState with per-buffer hunk tracking, SignType (Add/
Change/Delete/TopDelete/ChangeDelete) with display char and highlight group,
Hunk (start/count/sign), GitBase (Index/Head), set_hunks with auto-count
computation (added/modified/removed), sign_at line query, next_hunk/prev_hunk
wrapping navigation, summary string. Statusline data model: Segment enum (Mode/
File/Modified/ReadOnly/FileType/Position/Percent/Encoding/FileFormat/Diagnostics/
Git/Text) with render method, StatuslineData with left/center/right section
arrays, separator, active flag, from_state builder for default layout.
Message/notification model: MsgLevel (Debug/Info/Warn/Error, ordered) with
highlight group mapping, Message (id/level/text), MessageStore (push/info/warn/
error, current command-line message, history with 200-entry cap, clear_current/
clear_history, by_level filter).
Viewport state model: ViewportState with per-window scrolloff/sidescrolloff/wrap/
text_rows/text_cols/top_line/left_col, ensure_visible cursor-follow with margin
clamping, scroll_center (zz), scroll_top (zt), scroll_bottom (zb), is_line_visible
query, horizontal scroll for nowrap mode.
Floating window model: FloatAnchor (Editor/Cursor/Window/NW/NE/SW/SE), BorderStyle
(None/Single/Double/Rounded/Solid/Shadow/Custom), FloatKind (Dialog/Tooltip/
Preview/Completion), FloatConfig with sizing/position/zindex/title/footer/focus
behavior, FloatWindow instance, FloatLayer manager with open/close/render_order
(z-index sorted with creation tiebreak)/focusable query.
Statusline DSL parser: DslToken (Literal/Separator/FilePath/FilePathAbsolute/
Modified/ReadOnly/Line/Column/Percent/FileType/Highlight), DslVars for variable
substitution, parse_format tokenizer for %f/%F/%m/%r/%l/%c/%p/%y/%%/%=/
%#Group# directives, render_tokens with separator markers.
PTY-level E2E verification pending harness reconstruction.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| TODO reconstruction chain is present | `verified` | 2026-02-11 | [/docs/todo/README.md](/docs/todo/README.md), [/docs/todo/waves/README.md](/docs/todo/waves/README.md) |
| Implementation workspace is present | `verified` | 2026-02-11 | 20-crate workspace, `cargo check --workspace` and `cargo test --workspace` (538 pass) |
| Runtime blocker behavior (`Shift+a`, split, explorer) | `partial` | 2026-02-11 | T1 headless harness tests pass; T2 PTY harness pending |
| Live E2E screen-oracle closure | `unverified` | 2026-02-11 | PTY harness not yet reconstructed |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `partial` | Shift+a normalization implemented and unit-tested; T2 pending |
| Window tree and split lifecycle | `partial` | layout tree with split/close/rebalance implemented and unit-tested; T2 pending |
| Explorer window and actions | `partial` | explorer open/close routing, state model (ExplorerState/ExplorerNode/NodeId), tree with expansion/flattening, navigation actions (j/k/h/l/Enter/o/q), key routing implemented and T1-tested; filesystem integration pending |
| Terminal window integration | `partial` | TerminalState model with id/shell/title/exited/exit_code/cols/rows implemented; PTY spawn/read/write not yet implemented |
| Viewport wrap and cursor safety | `unverified` | basic cursor motion; wrap not yet implemented |
| Test harness fidelity | `partial` | T1 headless harness with step dumps; T2 PTY harness pending |
| Source topology and workspace policy | `verified` | 20-crate grouped tree matches spec; all files ≤ 200 lines; multi-task runtime |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

A release may proceed only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed
2. matching `*R` E2E rows in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) pass using screen-state assertions
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are synchronized in the same change

Current state (2026-02-11): blocked (docs-only baseline).

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
