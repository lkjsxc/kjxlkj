# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current baseline state.

## Baseline Statement (2026-02-11)

Workspace reconstructed with 20 crates matching source-layout.md spec.
208 unit/integration tests pass. PTY E2E harness not yet reconstructed.
Text objects (iw/aw, bracket, quote, paragraph ip/ap, sentence is/as) implemented
with operator-pending dispatch. Tree-sitter (ic/ac, if/af) and tag (it/at)
text objects deferred.
Visual mode (v/V/Ctrl-v) with charwise/linewise selection operators, sub-mode
switching, anchor swap (o), operators d/x/y/c/s/>/</~/u/U/J/p. Blockwise visual
operations delegate to charwise (full block ops deferred).
Buffer management: alternate buffer tracking (Ctrl-^), :bn/:bp/:bd/:b N
navigation, :ls/:buffers listing, :e file opening, deletion with fallback,
:bfirst/:blast first/last buffer navigation.
223 unit/integration tests pass after wave-029.
Race and boundary validation: 17 additional tests (10 stress + 7 safety) pass
after wave-030, bringing total to 240 tests.
Stage-03 exit integration tests (wave-031): 12 tests covering ex command pipeline,
insert persistence, d$, search forward/n, :set ignorecase, star search, yy, diw,
Ctrl-a, bfirst/blast, visual yank, bracket match (%), bringing total to 252 tests.
Window command dispatch (wave-032): Ctrl-w prefix key dispatch with directional
focus (h/j/k/l geometry-based), cycle (w), previous (p), split (s/v/n), close
(c/q), only (o). Split semantics corrected for Vim convention. 19 new tests
bringing total to 271 tests.
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
