# Conformance: UI, Terminal, and Buffer Systems

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

UI views, terminal, buffer, window, and theme system conformance entries.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| View management | `implemented` | view_management module tests |
| Tab pages | `implemented` | editor_tabs module tests |
| Window splits | `implemented` | integration_tests.rs (HE-06), pty_e2e_tests.rs |
| Terminal grid | `implemented` | terminal module tests |
| Buffer options | `implemented` | buffer_options module tests |

### UI Views and Tabs

| Aspect | Status |
| --- | --- |
| `ViewKind` | Buffer / Terminal / Explorer / Help / Preview / QuickFix / LocationList / Empty |
| `View` | Typed view with id, kind, active flag + buffer/terminal/explorer constructors |
| `TabPage` | Ordered view collection with add/remove |
| `ViewManager` | Multi-tab view management with create_view/close_view/new_tab/active_view |
| `tab_line_label()` | Format tab label with view count indicator |

### Terminal Pane Management

| Aspect | Status |
| --- | --- |
| `TerminalPane` | Pane with resize and scroll region support |
| `PaneManager` | Create/close/get/set_active/list panes |
| `TmuxState` / `TmuxAction` | Tmux session state and action dispatch (8 variants) |
| `map_tmux_key()` | Map key names to tmux key sequences |
| `scrollback_capacity()` | Compute scrollback buffer size with 10K cap |

### Contracts System

| Aspect | Status |
| --- | --- |
| `ContractChecker` | Collect violations (non-strict) or panic (strict mode) |
| `require()` / `ensure()` / `invariant()` | Precondition / postcondition / invariant enforcement |
| `Violation` | Structured record with level, module, message, Display impl |
| `in_range()` / `non_empty()` / `valid_buffer_id()` / `within_limit()` | Contract helper predicates |

### Buffer Full

| Aspect | Status |
| --- | --- |
| `BufferType` | Normal / Scratch / Help / QuickFix / Terminal / Prompt / Popup |
| `BufferFlags` | modified, readonly, listed, loaded, modifiable flags |
| `BufferInfo` | Full buffer metadata with type, flags, filetype, encoding |
| `AlternateTracker` | Alternate buffer tracking for `:e #` / `Ctrl-^` with swap |
| `filter_listed()` / `find_by_name()` / `modified_count()` | Buffer list operations |

### Window Full

| Aspect | Status |
| --- | --- |
| `WindowOptions` | Per-window options (number, wrap, signcolumn, scrolloff, etc.) |
| `SignColumn` | Auto / Yes / No / Number display modes |
| `CloseGuard` | Allow / NeedsSave / LastWindow close prevention |
| `WindowSnapshot` | Snapshot with options for rendering |
| `WindowOptionStore` | Per-window option overrides with default fallback |

### Theme Full

| Aspect | Status |
| --- | --- |
| `Rgb` | RGB color with hex serialization, from_hex parsing, luminance |
| `ThemeColor` | Named / Rgb / Index / Default color references |
| `Face` | Foreground + background + attributes (bold/italic/underline/strikethrough) |
| `index_to_rgb()` | 256-color to RGB mapping (16 base + 216 cube + 24 grayscale) |
| `resolve_color()` | ThemeColor to RGB resolution with default fallback |

### PTY E2E Harness

| Aspect | Status |
| --- | --- |
| `PtyConfig` | Terminal config: term type, dimensions, timeout |
| `PtyAction` | TypeText / SendKey / WaitMs / WriteFile / Quit actions |
| `PtyExpectation` | FileContains / FileExists / ExitCode assertions |
| `PtyScenario` | Named scenario with actions and expectations |
| `validate_scenario()` | Scenario validation (non-empty name, has actions) |
| `estimate_duration()` | Duration estimation from scenario actions |

### PTY Regressions

| Aspect | Status |
| --- | --- |
| `insert_newline_scenario()` | Insert mode newline E2E test |
| `leader_explorer_scenario()` / `leader_terminal_scenario()` | Leader chord E2E tests |
| `gg_motion_scenario()` | Multi-key sequence E2E test |
| `undo_redo_scenario()` | Undo/redo E2E test |
| `append_eol_scenario()` | Append at EOL E2E test |
| `all_regression_scenarios()` | Collect all regression scenarios (7 total) |

### Golden Snapshots

| Aspect | Status |
| --- | --- |
| `SnapshotMode` | NoWrap / SoftWrap / HardWrap rendering modes |
| `SnapshotConfig` | Width, height, mode, line numbers |
| `render_snapshot()` | Render lines per config (truncate/wrap) |
| `compare_snapshot()` | Diff expected vs actual output |
| `build_nowrap_test()` / `build_wrap_test()` | Test case builders |

### Benchmark Suite

| Aspect | Status |
| --- | --- |
| `BenchmarkKind` | FileOpen / Keystroke / ScrollBurst / ResizeStorm / SnapshotRender / EditBurst |
| `BenchmarkConfig` / `BenchmarkResult` | Configuration and result types with stats |
| `default_suite()` | 6 standard benchmarks with iteration counts |
| `budget_for()` | Latency budgets per kind (16ms keystroke, 8ms scroll, etc.) |
| `format_benchmark_report()` | Report formatting and pass/fail tracking |

### Latency Regression

| Aspect | Status |
| --- | --- |
| `ProbeKind` | CursorVisibility / ViewportFollow / ScrollClamp / ResizeCursor / InputOrdering / BusyLoopDetection |
| `ProbeResult` | Kind, passed, message, elapsed_us |
| `probe_cursor_visibility()` | Deterministic cursor-in-viewport probe |
| `probe_viewport_follow()` | Scrolloff-aware follow probe |
| `probe_busy_loop()` | >120fps idle detection |
| `run_all_probes()` | Full regression suite |

### Long Line Fixtures

| Aspect | Status |
| --- | --- |
| `FixtureKind` | LongAscii / LongUnicode / LongMixed / WideChars / Tabs / CombiningMarks |
| `LineFixture` | Content with expected display width |
| `generate_fixture()` | Generate fixture of given kind and length |
| `all_fixtures()` | All fixture types at 1000 columns |
| `verify_fixture()` | Validate fixture content and width |

## Related

- Input modes: [/docs/reference/CONFORMANCE_KEYS_INPUT.md](/docs/reference/CONFORMANCE_KEYS_INPUT.md)
- Infrastructure: [/docs/reference/CONFORMANCE_KEYS_INFRA.md](/docs/reference/CONFORMANCE_KEYS_INFRA.md)
