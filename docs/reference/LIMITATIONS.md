# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)
User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`.

This document records the implementation status and any remaining gaps so readers understand what is available in the current implementation.

The implementation surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Status sources (avoid stale claims)

Do not infer “implemented” from target specs or placeholder feature lists.

Authoritative sources for “what exists” are:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) (the supported surface)
- the repository’s automated tests (when an implementation workspace exists)

This limitations document exists to capture **user-visible drift** and **known rough edges** against the target spec.

## High-priority UX defects (reported and/or suspected)

These items are prioritized because they block basic usability and because they can be missed by headless-only testing.

| Issue | Expected behavior | Defining spec |
|---|---|---|
| Leader key conflicts | `Space` acts as `<leader>` in Normal mode; feature chords like `<leader>e` and `<leader>t` are reachable | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) |
| Append at end-of-line (`a`) off-by-one | When cursor is on last character, `a` enters Insert at column `N` (true EOL) | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| Soft wrap not applied | Long lines wrap by default (`wrap = true`) | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| `.c` syntax highlighting missing | Built-in language detection includes C/C++ by file extension | [/docs/spec/features/syntax/syntax-files.md](/docs/spec/features/syntax/syntax-files.md) |

For each item above, the implementation MUST include an **interactive PTY-driven E2E regression test** that drives the real TUI path and verifies behavior via persisted output (prefer file writes over screen scraping to reduce flakiness). See [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).

## Performance Limits

Performance characteristics have been tested and validated through tests:

- Large file support (10k and 100k lines) with basic navigation
- Long line handling (10k+ character lines) with grapheme counting
- Latency probes for typing bursts (200 chars), scroll bursts (200 lines), and resize storms

Target performance posture is specified in:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

The following invariants are verified by tests:

- Snapshot generation is viewport-bounded (does not clone/materialize all buffer lines per frame).
- Snapshots are deterministic (same input produces same output).
- Input ordering is preserved (no one-key lag perception).

Known gaps / not yet enforced:

- Performance baselines vs Vim/Neovim are not yet enforced by a regression harness.

## Contract Verification Notes

The following contracts from [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md) have verification plans:

| Contract | Verification Plan |
|---|---|
| Queue depth observability | Requires runtime instrumentation; validated by profiling hooks |
| Latency measurement | Requires external timing infrastructure; validated by latency probe tests |
| Service supervision restart | Requires fault injection; validated by supervisor tests with mock failures |
| Cancellation idempotence | Fully tested; multiple cancel calls produce identical behavior |

All contracts have at minimum a partial test or verification strategy in place.

## UX gaps

- No mouse support (by design).
- Terminal integration (`:terminal`, `<leader>t`) now opens a terminal scratch panel buffer, but real PTY process wiring is still pending.
- File explorer (`:explorer`, `<leader>e`) now opens an explorer scratch panel generated from directory entries, but interactive tree navigation and TUI-side panel rendering are still pending.
- LSP integration has JSON-RPC protocol types, initialize/didOpen/didChange builders, extended types (hover, signature help, code actions, navigation, rename, code lens, formatting, symbols), and message encoding but is not connected to real language servers.
- Git integration detects current branch from `.git/HEAD`, has conflict marker detection, diff viewer types, and file indicators but does not run real git commands.
- Fuzzy finder (`<leader>f`, `<leader>g`) has scoring algorithm but no UI rendering.
- `:find` and `:livegrep` command entries exist but still route to a pending command-palette integration path.
- `:undotree` currently reports undo entry count only; interactive tree UI is not implemented.
- Session persistence (`:mksession`) records metadata and macro key strokes but does not serialize to/restore from disk.
- Swap files and undo persistence types exist but are not written to disk.
- Scripting types (completion providers, user commands, user functions, timers) exist but do not execute real script logic.
- User command execution (`user_command_exec.rs`) implements dispatch, nargs validation, and argument substitution but does not integrate with the main Ex command parser loop.
- User function execution (`user_function_exec.rs`) supports let/return/concat but does not support conditionals, loops, or full Vimscript expressions.
- Debounce manager (`debounce_exec.rs`) uses a FakeClock for deterministic testing but is not wired into the tokio runtime timer.
- Mapping expansion (`mapping_expansion.rs`) handles recursive expansion with MAX_DEPTH=100 guard and prefix matching but is not connected to the key parser dispatch.
- Accessibility checks (`accessibility.rs`) verify WCAG 2.1 contrast ratios and focus indicators but are not enforced at render time.
- Profiling (`profiling.rs`) supports span timing and counters but is not instrumented into the core loop or service layer.
- Event automation (`event_automation.rs`) dispatches autocmd handlers deterministically but is not called automatically from buffer/mode transitions.
- Script loader (`script_loader.rs`) parses script files into ScriptLine variants but does not integrate with filesystem I/O or the `:source` command dispatch.
- Keyboard layout (`keyboard_layout.rs`) supports Dvorak/Colemak/Workman remapping with hjkl preservation but does not auto-detect the system layout.
- Viewport wrap model (`viewport_wrap.rs`) computes display rows and cursor-follow for `wrap = true` but is not connected to the render pipeline.
- Service supervisor (`supervisor.rs`) tracks health, restart policy, and exponential backoff but is not wired into the tokio runtime service spawning.
- Extended text objects (`text_objects_ext.rs`) implement sentence, paragraph, and argument objects but the argument text object only works on single lines.
- Notification queue supports priority, dedup, and timeout but is not rendered in the TUI yet.
- Mode transition validation table exists but is not enforced at runtime (transitions are allowed unconditionally).
- UI component model (Rect, LayoutNode, standard_layout) exists for deterministic layout but is not connected to the renderer.
- Cursor state types (CursorState, CursorHint) exist for snapshot rendering but are not emitted by the render loop.
- Mode configuration types (CursorConfig, LineNumberStyle, ModeIndicatorFormat) exist but are not persisted or applied at runtime.
- Runtime lifecycle (RuntimePhase, RestartPolicy, ServiceLifecycle, BusCapacity) is modeled but not enforced by the tokio runtime.
- Theme model (ThemePalette, ThemeRegistry, 3 built-in themes) exists with style resolution but is not connected to the render pipeline.
- Floating window types (FloatConfig, FloatBorder, FloatAnchor, ZoomState, LayoutPreset) exist as data model only; no floating windows are rendered yet.
- Search highlight model (SearchHighlights with next/prev/hlsearch) exists but is not integrated with the search dispatch or renderer.
- Undo branching tree (BranchingUndoTree) coexists with the linear UndoTree; the branching variant is not yet wired into buffer state.
- DAP debugging types (DapState, Breakpoint, StackFrame, Variable) are scaffolding; no debug adapter connection is implemented.
- Extended marks model (MarkRegistry with global/local/special marks) exists but is not wired into the main EditorState marks HashMap.
- Substitute flags model (SubstituteFlags, ConfirmState, parse_substitute_cmd) extends the basic substitute dispatch but the confirm interaction is not connected to the TUI.
- Extended completion types (CompletionItemKind 25 variants, CompletionList with filtering) exist alongside the base CompletionItem but are not connected to real LSP responses.
- Buffer list model (BufferFilter, BufferListEntry, build/format_buffer_list) exists but is not used by the `:ls` command dispatch.
- Visual block operations model (BlockSelection, BlockEdit, extend_to_eol) exists but block insert/append/change are not applied to buffer text.
- Command-line completion sources (complete_command, complete_option, complete_buffer) exist but are not wired into the command-line input handler.
- Keybinding DSL parser (parse_key_sequence, validate_key_sequence) exists but parsed key chords are not applied to the mapping table dispatch.
- View tree (ViewTree with focus stack, ViewNode hierarchy, from_splits) exists but is not connected to the render loop.
- Popup menu overlay (PopupMenu with scroll/selection, HoverTooltip) exists but is not rendered in the TUI.
- Status line layout (StatusLineLayout with sections, vim_default) exists but is not used by the actual status line renderer.
- Contract checker (ContractChecker with 6 boundary checks) exists but is not called at runtime boundaries.
- Regex engine (`compile_pattern`, `find_all_matches`, `find_next`, `find_prev`, `translate_vim_pattern`) exists but is not connected to the search/substitute dispatch pipeline.
- Insert mode extended editing (`delete_word_back`, `delete_to_line_start`, `indent_line`, `dedent_line`, `collect_completions`) exists but is not wired into the insert key handler.
- Replace mode state (`ReplaceState`, `replace_char_at`, `undo_replace_at`, `apply_single_replace`) exists but is not integrated into the mode dispatch loop.
- Command history (`CommandHistory` with push/dedup, prev/next, prefix/substring search) is now used by command-line Up/Down navigation, but prefix/substring search bindings are not wired yet.
- Notification rendering (`render_notification`, `wrap_text`, `max_visible_notifications`, `NotifPosition`) exists but is not connected to the TUI render loop.
- Cursor visibility (`cursor_for_mode`, `check_cursor_in_viewport`, `check_transition_visibility`, `cursor_shape_escape`) exists but is not wired into mode transitions or the render pass.
- Text manipulation features (`join_lines`, `convert_case`, `sort_lines`, `trim_trailing`, `reverse_chars`, `reindent`) are partially connected (`:sort`, `J`, `~`, `gU`, `gu` paths exist) but full coverage and parity are incomplete.
- Git status model (`FileStatus`, `StatusEntry`, `parse_diff_hunks`, `compute_gutter_signs`, `parse_blame_output`) exists but is not connected to git binary execution or gutter rendering.
- Terminal emulator grid (`TerminalGrid`, `put_char`, `clear`, `scroll_up`, `parse_ansi_simple`) exists but is not connected to a real PTY or the terminal pane renderer.
- Highlight groups (`HighlightGroup` 31 variants, `token_to_group`, `highlight_line`) exist but are not connected to tree-sitter or the buffer render pipeline.
- Large buffer support (`LoadStrategy`, `build_line_index`, `compute_chunks`, `extract_line_range`) exists but is not integrated into the file open/save pipeline.
- Layout invariant checker (`check_layout_invariants`, `check_vertical_coverage`) exists but is not called during the render loop.
- Range/address parser (`parse_range`, `parse_address`, `resolve_range`) exists but is not connected to the Ex command dispatch pipeline.
- Plugin prevention auditor (`audit_source`, `audit_files`, `check_dependencies`) exists but is not run as a CI check or build step.
- Keybinding coverage map (`CoverageMap` with untested/undocumented/find_duplicates) exists but is not wired into a CI gate or automated report.
- Theme engine (`Theme`, `default_dark_theme`, `default_light_theme`, `resolve_scope`, `apply_override`) exists but is not connected to the render pipeline.
- Session commands (`serialize_session`, `parse_session`, `SessionData`) exist but are not connected to `:mksession` or `:source`.
- Buffer metadata (`AlternateFile`, `BufferVariables`, `format_buffer_info`) exists but is not integrated into the buffer lifecycle or `:ls` command.

## Code structure

All source files are under the 200-line guideline from `/docs/policy/STRUCTURE.md`.
The current maximum source file is 200 lines (`src/crates/kjxlkj-core-state/src/commands.rs`).
Tests are extracted to integration test files under each crate's `tests/` directory.

## Syntax commands

- `:syntax` only supports on/off/manual/enable/disable arguments; syntax groups and match/region commands are not implemented.
- Language detection is extension-based only; shebang and modeline detection are not implemented.

## Command-line parser

- Command-line completion triggers are recognized but completion candidates require integration with command registry.
- PasteRegister action is recognized but actual register content paste requires core state integration.

## File explorer

- File explorer is a tree model only; actual filesystem I/O requires integration with FS service.
- Git badges and diagnostic indicators are defined but not populated from real service data.
- File operations (create, delete, rename, cut/copy/paste) are modeled but not wired to filesystem.

## Window splits

- Window layout uses floating-point weight-based sizing; integer cell rounding for terminal output is not implemented.
- Window navigation between splits (Ctrl-W h/j/k/l) requires integration with input dispatch.

## Visual selection

- Block selection text extraction handles short lines but does not pad with spaces for rectangular operations.
- Visual selection does not yet integrate with operator execution pipeline.

## Latency tracking

- Latency budgets are defined as constants but there is no automated enforcement or CI regression gate.
- TimingGuard requires mutable probe reference which limits usage in concurrent contexts.

## Buffer lifecycle

- Swap file creation uses path hashing but does not write actual swap content to disk.
- Auto-save policy is modeled but timer-based auto-save is not wired to the event loop.

## Config options

- Option scope resolution (Global/Buffer/Window) is implemented but per-buffer and per-window overrides are not persisted across sessions.
- Only 10 built-in options defined; Vim's full option set is not modeled.

## File I/O commands

- Path validation checks parent directory existence but does not handle race conditions with concurrent filesystem changes.
- WriteAll iterates buffers but does not implement atomic multi-file write transactions.

## Cursor overlay

- Overlay priority layering is modeled but rendering does not blend overlapping highlight colors.
- Matching bracket search scans linearly; no syntax-aware bracket matching is implemented.

## Completion engine

- Path completion reads directory entries but does not handle permission errors gracefully.
- Custom completion sources are defined in the enum but no plugin API for registering them exists.

## Viewport follow

- Viewport follow uses integer line arithmetic; sub-line scrolling for smooth scroll is not implemented.
- Horizontal follow does not account for multi-width Unicode characters in column calculations.

## Session persistence

- Session state serialization produces a summary JSON (counts) rather than full restorable JSON with all data.
- Mark and jump list capacity is capped at 100; older entries are silently evicted.

## Extended text objects

- Argument text object works on single-line expressions only; multi-line argument lists are not supported.
- IndentLevel text object uses character-count indentation; mixed tabs/spaces may give unexpected results.

## Long line handling

- Wide character detection uses simplified Unicode ranges; full East Asian Width property is not consulted.
- Line segmentation does not cache results; repeated rendering of the same long line recomputes segments.

## Insert newline

- Smart indent only recognizes `{`, `(`, `[`, `:` as indent triggers; language-specific indent rules are not supported.
- Auto-indent does not consult treesitter or LSP for context-aware indentation.

## LSP requests

- Pending request tracking is single-server; multi-server scenarios require separate PendingRequests instances.
- Diagnostic store does not support incremental (delta) diagnostic updates.

## Event automation

- Glob pattern matching supports only `*` and `*.ext` patterns; full glob syntax (brackets, ranges) is not implemented.
- Autocommand execution order is insertion order; priority-based ordering is not supported.

## Script files

- Script file parsing does not support multi-line constructs (if/endif, while/endwhile) as nested structures.
- Script execution is modeled as sequential command dispatch; conditional branching is parsed but not evaluated.

## User commands

- User command expansion handles `<args>`, `<q-args>`, `<bang>` but not `<line1>`, `<line2>`, `<count>`, `<reg>`.
- Command completion attribute is stored but not wired to the completion engine.

## User functions

- Function body is stored as raw strings; no expression evaluation or variable scoping is implemented.
- Varargs (`...`) are parsed but `a:000` list access is not implemented.

## Input timing

- Debouncer uses wall-clock Instant which may be affected by system clock adjustments.
- Input rate window eviction uses linear scan; very high event rates may cause overhead.

## Mappings engine

- Recursive mapping resolution has a hard depth limit of 20; Vim's limit is configurable.
- Buffer-local mappings are flagged but not scoped to specific buffer IDs.

## Keybinding coverage

- Coverage map is built programmatically; dynamic keybinding changes at runtime are not tracked.
- Only normal mode coverage is pre-built; insert/visual/command modes require manual registration.

## Planned Improvements

See [/docs/todo/README.md](/docs/todo/README.md) for roadmap.

## Reporting issues (local workflow)

When reporting or logging issues, capture:

- the conformance expectation (`/docs/reference/CONFORMANCE.md`)
- the spec reference (exact `/docs/spec/...` document)
- a minimal reproduction (prefer a headless script when possible)
- expected vs actual behavior

### Buffer Full

- `BufferInfo` is a metadata container — does not hold actual text content (that's in `TextBuffer`).
- `AlternateTracker` only tracks current/alternate pair — no full buffer navigation history.
- `BufferType::Popup` and `BufferType::Prompt` are type-only; no special behavior attached.

### Window Full

- `WindowOptions` is a per-window copy — no inheritance from global options.
- `CloseGuard` only checks modified and last-window; no `:confirm` integration.
- `WindowSnapshot` does not include fold state or diff mode indicators.
- `WindowOptionStore` uses clone on get — no shared reference to avoid mutation races.

### LSP Features

- `CompletionKind` is a subset of LSP spec (13 of 25 kinds).
- `filter_completions()` uses simple prefix matching — no fuzzy matching.
- `DiagnosticStore` keyed by buffer id (u64) — no URI-based lookup.
- `CodeActionKind` is a subset — no custom/extensible kinds.

### Session Full

- `serialize_session()` produces vimscript-like output — not actual vimscript.
- `parse_session_buffers()` only extracts `edit` commands — no option/mark restoration.
- Global marks stored as HashMap<char, SessionMark> — no validation of mark characters.
- No encryption or compression of session files.

### Theme Full

- `index_to_rgb()` only maps base 8 colors (0-7) and 216/24 extended ranges; colors 8-15 use gray fallback.
- `ThemeColor::Named` is not resolved — falls through to default.
- Face builder pattern returns owned values — no in-place mutation for efficiency.
- No file-based theme loading (TOML/JSON).

### Cursor Rendering

- `cursor_shape_escape()` uses DECSCUSR sequences — may not work on all terminals.
- `ModeCursorMap` uses string matching for mode names — no enum integration.
- Blink interval is global — no per-mode blink configuration.
- No cursor color customization beyond `color_override` string.

### Crate Topology

- `expected_topology()` is a static snapshot — does not parse actual Cargo.toml files.
- `validate_topology()` checks dependency direction only — no cycle detection.
- `CrateRole` is manually assigned; no automated role inference.

### Buffer Features

- `BufferVariables` stores string values only — no typed variable support.
- `AutoCmdRegistry` uses simple string pattern matching — no glob or regex.
- `BufferLocalOptions` fields are `Option<T>` — no default inheritance from global options.
- `FileFormat::from_str` only accepts lowercase names.

### UI Components

- `layout_frame()` uses fixed 4-column line numbers — no dynamic width.
- `component_at()` returns the first visible match — no z-order.
- No component event/click handling — layout only.

### Keybinding DSL (Full)

- `parse_key_notation()` supports single modifiers per prefix — `<C-A-x>` works but `<CA-x>` does not.
- F-key range limited to F1-F12; extended F-keys (F13+) not supported.
- `<leader>` resolves to a KeySpec but leader key value is not configurable here.

### Layout Acceptance

- `check_coverage()` uses total area approximation — does not verify pixel-accurate coverage.
- No diagonal or non-rectangular region support.
- `run_all_invariants()` does not check CmdLinePresent or StatusLinePresent (only 4 of 6 invariants run).

### File Flows

- `resolve_path()` performs tilde expansion only — no environment variable expansion.
- `detect_encoding()` uses BOM and UTF-8 validation heuristics — no charset detection library.
- `validate_write_target()` checks path structure — does not verify filesystem permissions at check time.
- File operation sequences (`build_edit_flow`, `build_wq_flow`) are structural — not wired to actual I/O.

### Mode Keybindings

- `build_normal_bindings()` covers 25+ keys — not the full Normal-mode key surface.
- `check_mode_coverage()` checks against a hardcoded expected count — not against the full spec table.
- Only Normal mode bindings are pre-built; other modes require manual registration.

### UI Features (Status Line / Message Area)

- `render_segment()` produces strings — not connected to TUI rendering cells.
- `StatusContext` fields are all strings — no live computation from editor state.
- `MessageArea` is a data model — display timer and positioning are not implemented.
- Status line segments are statically ordered — no user-configurable layout.

### Keybinding Tables

- `build_normal_table()` contains 60+ bindings — not all Vim Normal-mode keys are covered.
- `coverage_stats()` counts per category — no per-key gap analysis against spec.
- Only Normal mode table is pre-built; other modes are empty by default.
- Action descriptions are static strings — no localization support.

### Viewport Integrity

- `wrap_line()` uses `unicode_width` crate — does not handle all Unicode edge cases (e.g., combining marks).
- `DisplayCell::Wide` marker is set manually — no automatic detection from prior cell context.
- `validate_viewport()` checks dimensions and widths — does not verify cursor position consistency.
- Long line threshold is hardcoded at 1000 columns — not configurable.

### Leader Keys

- `LeaderConfig` default key is space — changing it requires constructing a new config.
- `LeaderRegistry` uses linear scan for resolve — no trie or hash-based lookup.
- `partial_matches()` returns all prefix matches — no ranking or priority.
- Leader timeout (1000ms) is not enforced at runtime — requires integration with input timing.

### FS Directory Listing

- `list_directory()` builds listing from in-memory data — no actual filesystem I/O.
- `sort_entries()` sorts entire listing at once — no lazy or streaming sort for large directories.
- `max_children_check()` returns a boolean — no truncation or pagination of large listings.
- `is_hidden()` only checks dot-prefix — no platform-specific hidden file attributes.

### Command-Line Window

- `CmdlineWindowState` stores history as `Vec<String>` — no shared history with command-line mode.
- `render_cmdline_window()` returns strings — not wired to TUI rendering cells.
- Only single-character prompts supported (`:`, `/`, `?`).
- No syntax highlighting or command completion within the command-line window.

### Streaming IO

- `StreamReader` processes chunks in memory — no actual async I/O integration.
- `estimate_line_count()` uses fixed 40 bytes/line — not adaptive to actual content.
- `StreamConfig::max_file_size` is 1GB — files larger than this are rejected.
- No streaming write support — only read path is modeled.

### Profiling Workflow

- `compute_stats()` requires pre-collected samples — no live instrumentation.
- `format_report()` produces plain text — no JSON/CSV export for automation.
- `meets_budget()` uses simple threshold comparison — no trend detection or regression analysis.
- Profiling targets are predefined — no custom target registration.

### UX Coverage

- `build_normal_coverage()` is statically defined — no dynamic discovery from key parser.
- Coverage entries track tested/documented booleans — no link to specific test names.
- `keyboard_only_check()` verifies entries exist — does not verify actual runtime behavior.
- Only Normal and Insert mode coverage are pre-built — Visual/Command/Replace modes need manual registration.

### Feature Reachability

- `define_core_features()` is a static list — no dynamic discovery from the crate graph.
- `check_reachability()` only checks if entry points are defined — does not verify runtime wiring.
- `EntryKind::MouseAction` entries exist as placeholders — no mouse support is implemented.
- Feature test status is manually set — no integration with test runner results.

### Feature Integration

- `validate_scenario()` is a structural dry-run only — no actual execution.
- `ScenarioStep::WaitMs` is not enforced — timing is informational.
- No parallel scenario execution or test isolation.

### Git Full Integration

- `parse_diff()` only handles unified diff format; does not parse combined/merge diffs.
- `parse_log()` expects specific `--format` output; arbitrary log formats not supported.
- `BlameEntry` is a structural placeholder — no actual `git blame` subprocess invocation.
- `compute_signs()` maps hunks to gutter signs but does not handle overlapping ranges.

### Terminal Pane Management

- `TerminalPane` resize is dimension-only — no actual PTY resize signal (SIGWINCH) sent.
- `PaneManager` tracks panes in-memory only; no persistence across sessions.
- `TmuxAction` is a dispatch enum — actual tmux binary communication not implemented.
- `scrollback_capacity()` caps at 10,000 lines; no configurable override.

### UI Views and Tabs

- `ViewManager` does not enforce maximum tab count; unbounded tab creation possible.
- `close_view()` removes from all tabs — no per-tab close semantics.
- No view serialization for session restore.

### Popup and Overlay Management

- `PopupState` supports single-column item lists only; no multi-column completion.
- `OverlayManager` uses stack ordering — no explicit z-index control.
- Popup positioning does not account for existing overlapping popups.
- `compute_popup_rect()` uses fixed popup dimensions; no dynamic sizing from content.

### Notification Dispatch

- `Dispatcher` stores notifications in a Vec — no bounded ring buffer for memory control.
- `gc()` must be called explicitly; no automatic timer-based cleanup.
- No notification persistence across editor restarts.
- Severity ordering uses Ord; custom filtering strategies not supported.

### Contracts System

- `ContractChecker` is a runtime tool only; no compile-time contract enforcement.
- Strict mode panics immediately — no structured error recovery path.
- `valid_buffer_id()` only checks non-zero; does not validate against actual buffer registry.
- Contract violations are stored in a Vec with no deduplication.

### PTY E2E Harness

- PTY scenarios are structural definitions only — no actual PTY spawning is implemented.
- `validate_scenario()` checks structure — does not verify action/expectation compatibility.
- `estimate_duration()` uses fixed per-action estimates — no adaptive timing.
- No parallel scenario execution support.

### PTY Regressions

- All regression scenarios are declarative — require PTY harness execution to actually run.
- `leader_explorer_scenario` and `leader_terminal_scenario` verify intent dispatch — not actual UI state changes.
- File-based assertions require write support — scenarios that only observe screen state are not covered.

### Golden Snapshots

- `render_snapshot()` handles basic truncation and wrapping — no tab expansion or wide character support.
- `compare_snapshot()` uses line-by-line comparison — no contextual diff or fuzzy matching.
- No automated golden file management (snapshots are built in-memory, not loaded from disk).

### Benchmark Suite

- `compute_benchmark_result()` requires pre-collected samples — no integrated timing infrastructure.
- `budget_for()` uses fixed thresholds — no machine-adaptive or percentile-based budgets.
- Benchmarks are definitions only — no actual microbenchmark runner is integrated.
- No disk I/O or network latency simulation.

### Latency Regression

- All probes use synthetic inputs — no integration with real terminal event streams.
- `probe_busy_loop()` checks frame rate — does not detect CPU usage patterns.
- `probe_input_ordering()` checks timestamps — does not verify actual event ordering in the parser.
- `run_all_probes()` uses hardcoded sample data — not parameterizable.

### Long Line Fixtures

- `long_unicode_line()` uses a limited set of Unicode characters — not exhaustive East Asian Width coverage.
- `FixtureKind::CombiningMarks` generates placeholder combining sequences — not all combining classes tested.
- `expected_display_width` is computed from generation parameters — may not match actual terminal rendering.
- Fixture verification checks non-empty and positive width — no character-level validation.
