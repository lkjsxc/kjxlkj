# Conformance: Input Infrastructure and Coverage

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Input handling, keybinding tables, layout, coverage, and profiling conformance entries.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Buffer features | `implemented` | buffer_options module tests |
| Keybinding DSL | `implemented` | keybinding_dsl module tests, REG-03 |
| Mapping registry | `implemented` | mappings module tests |
| Autocommands | `implemented` | autocmd module tests |
| Profiling structure | `implemented` | contract_tests.rs (profiling_metrics) |

### Buffer Features

| Aspect | Status |
| --- | --- |
| `BufferVariables` | Buffer-local variable store (set/get/remove/keys) |
| `BufferLocalOptions` | Per-buffer option overrides (tabstop, shiftwidth, expandtab, etc.) |
| `FileFormat` | Unix / Dos / Mac with line ending strings |
| `BufEvent` | 9 autocommand events (BufEnter/Leave/Read/Write/New/Delete/WinEnter/WinLeave/Modified) |
| `AutoCmdRegistry` | Register/query/remove autocommands by event and pattern |

### UI Components

| Aspect | Status |
| --- | --- |
| `ComponentKind` | 10 kinds: StatusLine, TabLine, CommandLine, LineNumbers, SignColumn, etc. |
| `Component` | Positioned UI element with visibility and hit testing |
| `layout_frame()` | Build standard editor frame from dimensions and toggle options |
| `component_at()` | Find visible component at screen position |

### Keybinding DSL

| Aspect | Status |
| --- | --- |
| `SpecialKey` | 16+ special keys (Space, Enter, Escape, F1-F12, arrows, etc.) |
| `Modifiers` | Ctrl / Alt / Shift / Meta modifier flags |
| `parse_key_notation()` | Parse `<C-x>`, `<A-Space>`, `<leader>`, `<F12>` notation |
| `parse_key_sequence()` | Parse full sequences like `<C-w>h` or `<leader>ff` |

### Layout Acceptance

| Aspect | Status |
| --- | --- |
| `InvariantKind` | NoOverlap / FullCoverage / MinSize / CursorVisible / CmdLinePresent / StatusLinePresent |
| `LayoutRegion` | Rectangular region with overlap detection |
| `check_no_overlap()` | Verify no two layout regions overlap |
| `check_coverage()` | Verify regions cover full screen area |
| `check_cursor_visible()` | Verify cursor is within a visible region |
| `run_all_invariants()` | Run all 4 layout invariant checks |

### File Flows

| Aspect | Status |
| --- | --- |
| `FileResult` | Success / NotFound / PermissionDenied / IoError result types |
| `OpenOptions` / `WriteOptions` | Encoding, line ending, readonly, create, force, backup |
| `FileOp` | Open / Edit / Write / WriteQuit / SaveAs enum |
| `validate_write_target()` | Path validation for writes (empty, directory, exists checks) |
| `resolve_path()` | Tilde expansion and path canonicalization |
| `detect_encoding()` / `detect_line_ending()` | Auto-detect UTF-8/Latin1 and LF/CRLF/CR |
| `build_edit_flow()` / `build_wq_flow()` | Multi-step file operation sequences |

### Mode Keybindings

| Aspect | Status |
| --- | --- |
| `UxMode` | 9 variants: Normal, Insert, Visual, VisualLine, VisualBlock, Replace, Command, OperatorPending, Terminal |
| `UxBinding` / `ModeBindingTable` | Per-mode binding table with add/count/for_mode/find_key/undocumented |
| `build_normal_bindings()` | 25+ Normal-mode key bindings covering hjkl, 0/$, w/b/e, i/a, v/V, operators |
| `check_mode_coverage()` | Verifies mode binding table completeness |

### UI Features

| Aspect | Status |
| --- | --- |
| `StatusSegment` | 12 segment types: Mode, FileName, FileType, Encoding, Position, Percent, etc. |
| `StatusLine` | Default layout with left/right alignment |
| `render_segment()` | Segment rendering with context-based data |
| `StatusContext` | Runtime context (mode, filename, filetype, encoding, position, etc.) |
| `MessageArea` | Info/error message display with clear |

### Keybinding Tables

| Aspect | Status |
| --- | --- |
| `ActionCategory` | 12 categories: Motion, Operator, ModeSwitch, Command, Search, Scroll, etc. |
| `BindingEntry` / `BindingTable` | Key-to-action mappings with category, description |
| `build_normal_table()` | 60+ Normal-mode bindings across all categories |
| `coverage_stats()` | Coverage statistics per category via HashMap |

### Viewport Integrity

| Aspect | Status |
| --- | --- |
| `DisplayCell` | Normal / Wide / Continuation cell types |
| `DisplayRow` | Row of display cells with width computation |
| `wrap_line()` | Unicode-width-aware line wrapping |
| `is_long_line()` / `truncate_line()` | Long line detection (1000+ cols) and truncation |
| `validate_viewport()` | Full viewport validation (dimensions, row widths) |

### Leader Keys

| Aspect | Status |
| --- | --- |
| `LeaderConfig` | Default leader key (Space), timeout (1000ms) |
| `LeaderBinding` / `LeaderRegistry` | Leader chord binding and resolution (bind/resolve/partial_matches) |
| `default_leader_bindings()` | 17 default bindings: find, buffer, git, LSP, terminal, explorer, etc. |

### FS Directory Listing

| Aspect | Status |
| --- | --- |
| `DirEntry` | Name, is_dir, size, hidden flags per filesystem entry |
| `SortOrder` | Name / NameDesc / Size / SizeDesc / Type / TypeDesc sorting |
| `DirListing` | Path, entries, truncated flag for directory listing results |
| `sort_entries()` | Sort by criteria with dirs-first ordering |
| `filter_hidden()` / `is_hidden()` | Dot-file filtering |
| `max_children_check()` | Large directory guard |

### Command-Line Window

| Aspect | Status |
| --- | --- |
| `CmdlineWindowState` | History, cursor position, prompt character, active flag |
| `CmdlineViewport` | Viewport state for command-line window (top_line, visible_lines, width) |
| `open()` / `close()` | Open with history, close returns selected command |
| `move_cursor()` / `edit_line()` | Cursor navigation and line editing |
| `follow_cmdline_cursor()` | Viewport follow for command-line window |
| `render_cmdline_window()` | Render visible lines with prompt prefix |

### Streaming IO

| Aspect | Status |
| --- | --- |
| `StreamState` | Idle / Reading / Complete / Error state machine |
| `ReadChunk` | Data chunk with offset and is_last flag |
| `StreamConfig` | Chunk size (64KB default), max file size (1GB default) |
| `StreamReader` | Chunk-based file reading with progress tracking |
| `validate_file_size()` | File size validation against limit |
| `estimate_line_count()` | Line count estimation from byte count |

### Profiling Workflow

| Aspect | Status |
| --- | --- |
| `ProfileTarget` | Startup / FileOpen / Keystroke / Scroll / Resize / Render / FullSession |
| `ProfileConfig` | Target, iterations (100), warmup (10), output path |
| `ProfileResult` | Samples with min/max/avg/p95 statistics |
| `compute_stats()` | Statistical computation from sample data |
| `format_report()` | Human-readable profiling report |
| `meets_budget()` / `default_budgets()` | Latency budget checking (16ms keystroke, 8ms scroll, etc.) |

### UX Coverage

| Aspect | Status |
| --- | --- |
| `CoverageEntry` | Key, action, tested/documented flags, mode |
| `CoverageSummary` | Total/tested/documented counts with gap list |
| `build_normal_coverage()` | 30+ Normal-mode key coverage entries |
| `build_insert_coverage()` | Insert-mode key coverage entries |
| `compute_summary()` | Coverage summary statistics |
| `find_untested()` / `find_undocumented()` | Gap detection |
| `keyboard_only_check()` | Verify keyboard-only accessibility invariant |

### Feature Reachability

| Aspect | Status |
| --- | --- |
| `FeatureSpec` | Named feature with entry points and test flag |
| `EntryKind` | Keybinding / ExCommand / LeaderChord / MouseAction |
| `ReachabilityReport` | Features with reachable/unreachable counts |
| `define_core_features()` | 15+ core features (open, save, quit, search, undo, etc.) |
| `check_reachability()` | Feature reachability analysis |
| `has_keybinding_entry()` / `has_command_entry()` | Entry point type checks |

## Related

- Key systems: [/docs/reference/conformance/CONFORMANCE_KEYS_SYSTEMS.md](/docs/reference/conformance/CONFORMANCE_KEYS_SYSTEMS.md)
- Modes: [/docs/reference/conformance/CONFORMANCE_MODES.md](/docs/reference/conformance/CONFORMANCE_MODES.md)
