# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current baseline state.

## Baseline Statement (2026-02-11)

Workspace reconstructed with 20 crates matching source-layout.md spec.
Text objects (iw/aw/iW/aW, bracket, quote, paragraph, sentence) implemented;
tree-sitter/tag text objects deferred. Visual mode (v/V/Ctrl-v) with operators;
blockwise delegates to charwise. Buffer management with alternate tracking,
:bn/:bp/:bd/:b N/:ls/:e/:bfirst/:blast. Race/boundary validation (17 tests).
Stage-03 exit tests (12 tests). Window command dispatch (Ctrl-w prefix, 19 tests).
Boundary focus and resize dispatch (wave-033): Ctrl-w t/b for top-left/bottom-right
focus, +/-/>/<  resize (no-op placeholder), = equalize, _/| maximize (no-op
placeholder). Explorer open/close routing: open_explorer creates
ContentKind::Explorer leaf, close_explorer removes it, :ExplorerClose ex command.
24 new tests bringing total to 295 tests.
Explorer state model and data flow (wave-034): ExplorerState with root_path, tree,
expansion_set, selected_index, cached visible rows, NodeId-based identity.
ExplorerNode tree with find/parent_of/sort_children. ExplorerAction enum for
navigation (MoveDown/MoveUp/CollapseOrParent/ExpandOrOpen/Toggle/Close). Explorer
key routing intercepts j/k/h/l/Enter/o/q in Normal mode on explorer-focused
windows. TerminalState model with id/shell/title/exited/exit_code/cols/rows.
32 new tests bringing total to 327 tests.
Explorer and terminal service crates upgraded from stubs to state models.
Command and route wiring (wave-035): Wincmd W reverse cycle, H/J/K/L move-to-edge
(placeholder), r/R rotate (placeholder), x exchange (placeholder). Terminal window
creation via open_terminal. Explorer v/s split-open keys for opening files in
horizontal/vertical splits. Focus cycle reverse. 21 new tests bringing total to
348 tests.
Boundary and error semantics (wave-036): Jumplist navigation (Ctrl-o/Ctrl-i) and
changelist navigation (g;/g,) with PositionList data structure (100 entry cap,
go_older/go_newer/push, dedup). Jump recording on GotoLine/GotoFirstLine/
GotoLastLine/Search/StarSearch. Change recording on all text-changing actions.
navigate_jumplist/navigate_changelist with buffer-bounds clamping. Boundary tests
for empty list, past-end, single window, explorer close, terminal open. 26 new
tests bringing total to 374 tests.
Unit and integration coverage (wave-037): Mark system — m{a-z} set mark at cursor,
'{a-z} goto mark line (first non-blank), `{a-z} goto mark exact position.
MarkStore with HashMap<char, MarkPos>, lowercase a-z only (uppercase ignored).
Marks persist across mode changes. Goto unset mark is no-op. Buffer-bounds
clamping on goto. SetMark/GotoMarkLine/GotoMarkExact action variants; partial key
dispatch for m/' /` prefixes. 5 unit tests + 12 integration tests bringing total
to 391 tests.
Live E2E and race validation (wave-038): Macro recording and playback system —
q{a-z} starts recording, q stops, @{a-z} plays. MacroState with recording flag,
register, and key buffer. MacroRecordStart/MacroRecordStop/MacroPlay action
variants. Key capture integrated into handle_key pipeline (before dispatch);
stop-q intercept in Normal mode when recording active. keys_to_string serializer
and parse_macro_keys deserializer for register storage (Ctrl/Escape/BS/Enter/Tab
support). Uppercase register rejected. Race/boundary tests covering macro+mark+
jumplist+changelist+split interactions under stress. 5 unit tests + 15 integration
tests bringing total to 411 tests.
Fold commands (wave-039): z-prefix dispatch for zo (open fold), zc (close fold),
za (toggle fold), zR (open all), zM (close all), zr (reduce fold level), zm
(increase fold level), zj (jump next closed fold), zk (jump prev closed fold).
FoldState with indent-based fold computation, fold_level tracking, per-line
open/close/toggle. 9 z-prefix unit tests + 6 FoldState unit tests + 16
integration tests (including macro+fold, mark+fold, reduce/more cycle, combined
stress 20x). Tree-sitter and expression fold methods deferred. 31 new tests
bringing total to 442 tests.
Terminal escape parser and screen model (wave-040): VT100/xterm parser with 13
states, CSI dispatch (cursor/erase/scroll/insert/delete/SGR/scroll-region/
save-restore), SGR attributes (basic+bright+256+RGB fg/bg, bold/dim/italic/
underline/reverse/strikethrough), private modes (DECTCEM/alt-screen/bracketed-
paste), OSC title, escape dispatch (reverse-index/linefeed/next-line/cursor-
save-restore/reset), UTF-8 multi-byte accumulation. Screen model with cell grid,
cursor, scroll region, saved cursor, alt-screen, bracketed-paste. Filetype
detection: 15 languages by extension + shebang fallback. 31 new tests bringing
total to 473 tests.
LSP lifecycle model (wave-041): LspServerState with phase machine (Starting/
Initializing/Running/ShuttingDown/Stopped/Failed), ServerCapabilities (17 boolean
fields: completion, hover, definition, references, rename, code_action, formatting,
range_formatting, signature_help, code_lens, inlay_hints, document_symbols,
workspace_symbols, declaration, type_definition, implementation, diagnostics),
LspServerConfig (language/command/root_markers/filetypes), crash tracking with
3-retry limit and restart reset. Diagnostic model: DiagnosticStore with
replace_for_file (LSP push semantics), append (incremental), sort by severity→
file→line→col, next_in_file/prev_in_file wrapping navigation, count_by_severity,
for_file filter. Severity(Error/Warning/Info/Hint), DiagnosticKind(Diagnostic/
Build/Grep/Todo/Quickfix), DiagnosticLocation with optional end position.
Theme/highlight model: HlGroup enum (35 groups: 13 syntax + 22 UI), Color RGB,
Style with builder pattern, Theme with HashMap lookup, default_dark() One Dark
inspired. 20 new tests (5 lifecycle + 8 diagnostic + 7 theme) bringing total to
493 tests.
Git sign state model (wave-042): GitSignState with per-buffer hunk tracking,
SignType(Add/Change/Delete/TopDelete/ChangeDelete) with display char and highlight
group, Hunk(start/count/sign), GitBase(Index/Head), set_hunks with auto-count
(added/modified/removed), sign_at query, next_hunk/prev_hunk wrapping, summary.
Statusline data model: Segment enum (12 variants: Mode/File/Modified/ReadOnly/
FileType/Position/Percent/Encoding/FileFormat/Diagnostics/Git/Text) with render(),
StatuslineData with left/center/right section arrays, from_state builder.
Message/notification model: MsgLevel(Debug/Info/Warn/Error, ordered), Message
(id/level/text), MessageStore (push/info/warn/error, current display, history
cap 200, clear_current/clear_history, by_level filter). 22 new tests (7 gitsigns
+ 7 statusline + 8 messages) bringing total to 515 tests.
Viewport state model (wave-043): ViewportState with per-window viewport parameters
(scrolloff/sidescrolloff/wrap/text_rows/text_cols/top_line/left_col),
ensure_visible cursor-follow with margin clamping (vertical scrolloff and
horizontal sidescrolloff), scroll_center (zz), scroll_top (zt), scroll_bottom
(zb), bottom_line, is_line_visible query, clamp_top safety. Floating window
model: FloatAnchor (Editor/Cursor/Window/NW/NE/SW/SE), BorderStyle (None/Single/
Double/Rounded/Solid/Shadow/Custom), FloatKind (Dialog/Tooltip/Preview/Completion),
FloatConfig with sizing/position/zindex/title/footer/focus/close behavior,
FloatWindow instance, FloatLayer manager with open/close/render_order (z-index
ascending with creation order tiebreak)/focusable filtering. Statusline DSL
parser: DslToken enum (Literal/Separator/FilePath/FilePathAbsolute/Modified/
ReadOnly/Line/Column/Percent/FileType/Highlight), DslVars for variable values,
parse_format tokenizer for %f/%F/%m/%r/%l/%c/%p/%y/%%/%=/%#Group# directives,
render_tokens with separator markers and variable substitution. 23 new tests
(8 viewport + 7 float_win + 8 statusline_dsl) bringing total to 538 tests.
Tab page model (wave-044): TabPage (id/layout/active_window/label/modified),
TabId, TabList with ordered tab management (tab_new inserts after current,
tab_close refuses last tab, tab_only keeps current, tab_next/tab_prev wrapping,
tab_goto 1-indexed with range validation, tab_first/tab_last, tab_move absolute
with clamping, tab_move_relative). Zoom state: ZoomState with saved_layout/
zoomed_window, zoom_in saves and replaces tree with single leaf, restore
reinstates saved layout (collapsed-unary cleanup for closed windows), toggle
cycles in/out, indicator \"[Z]\", on_window_closed removes from saved layout.
Tab/zoom ex commands added to command_parse: :tabnew/:tabe/:tabedit/:tabclose/
:tabc/:tabonly/:tabo/:tabnext/:tabn/:tabprevious/:tabprev/:tabp/:tabfirst/:tabfir/
:tabrewind/:tabr/:tablast/:tabl/:tabmove/:tabm/:ZoomToggle/:ZoomHeight/:ZoomWidth.
Action variants: TabNew/TabClose/TabCloseForce/TabOnly/TabNext/TabPrev/TabFirst/
TabLast/TabGoto/TabMove/ZoomToggle. 18 new tests (11 tabs + 7 zoom) bringing
total to 556 tests.
Mode configuration model (wave-045): CursorShape enum with DECSCUSR codes for all
modes (Normal→BlockBlink, Insert→BarBlink, Visual→BlockSteady, Replace→UnderBlink,
Command→BarBlink, OperatorPending→BlockSteady, TerminalInsert→BarBlink), mode
indicator text for statusline, cursor restore sequence for exit. Command-line
editing enhanced: Left/Right cursor movement, Home/End, Ctrl-b/Ctrl-e
beginning/end, Ctrl-w word-backward delete, Ctrl-u delete-to-start, Ctrl-c
cancel, Delete under cursor, mid-string character insertion. Insert completion
model: CompletionSource (8 sources priority-ordered), CompletionItem, CompletionState
menu machine with start/next/prev/confirm/dismiss/narrow. 30 new tests (13
mode_config + 11 completion + 7 cmdline editing—extracted from editor_cmdline to
editor_cmdline_tests) bringing total to 586 tests.
Motion system expanded to ~40 variants. Operator enum expanded to 11 variants.
Operator composition implemented with g-prefix operators, D/Y/gJ special forms,
case transforms (gu/gU/g~), and RangeType/Inclusivity classification.
RegisterStore with named/numbered/unnamed/small-delete/blackhole/clipboard registers.
Blackhole register ("_) suppresses all writes. Clipboard registers ("+, "*) store locally.
ForceModifier enum and pre-operator count multiplication implemented.
Vim regex compiler (with \c/\C case flags, \o/\O/\H atoms), ex command parser,
search system, command-line wiring. Put operations (p/P) paste from registers.
Operators wired to RegisterStore for yank/delete recording. Cursor clamping.
All source files ≤ 200 lines.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BASELINE-IMPL-04` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace and source tree reconstructed; 20 crates, compiles clean | `M2 missing feature` | closed | n/a |
| `LIM-BLOCK-KEY-04` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` normalization implemented and T1-tested; T2 PTY verification pending | `M4 verification gap` | medium | close with `KEYMODE-01`, `WR-01R` T2 screen assertions |
| `LIM-BLOCK-WIN-04` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split lifecycle implemented and T1-tested; T2 PTY verification pending | `M4 verification gap` | medium | close with `WIN-01R`..`WIN-05R` T2 screen assertions |
| `LIM-BLOCK-EXP-04` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer open/close routing, state model, tree, navigation all T1-tested; filesystem integration and PTY E2E pending | `M2 missing feature` + `M4 verification gap` | medium | implement filesystem integration, close with `EXP-01R`..`EXP-06R` |
| `LIM-BLOCK-E2E-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | T1 headless harness implemented; T2 PTY harness not yet rebuilt | `M2 missing feature` + `M4 verification gap` | high | rebuild PTY harness and enforce per-key state + frame assertions |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TOPO-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | topology matches spec; all files now ≤ 200 lines | closed | n/a |

## Deferred Items

Deferred items must not be correctness-critical or user-blocking.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| none | n/a | no deferred non-critical items are active | after baseline regeneration |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is reachable via real command/key paths
2. deterministic regression tests pass
3. matching live `*R` E2E tests pass using screen-state assertions
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
