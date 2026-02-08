# Conformance: Testing Infrastructure and Integration

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Session, syntax, explorer, completion, config, profiling, and integration conformance entries.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Session commands | `implemented` | session module tests, headless_e2e_tests.rs (HE-07) |
| Buffer metadata | `implemented` | buffer module tests |
| Syntax detection | `implemented` | filetype module tests |
| File explorer | `implemented` | file_explorer module tests |
| Completion engine | `implemented` | completion module tests |
| Config options | `implemented` | buffer_options module tests |
| Plugin prevention | `implemented` | audit module tests |
| Git integration | `implemented` | git_features module tests |
| LSP types | `scaffold-only` | Types exist, no LSP process |
| Notifications | `implemented` | notifications module tests |

## Session commands

| Feature | Behavior |
|---|---|
| `SessionData` | Working dir, open files, cursor positions, window layout |
| `serialize_session()` | Saves session to script format |
| `parse_session()` | Restores session from script |
| `WindowLayout` | Single, Horizontal, Vertical split trees |

## Buffer metadata

| Feature | Behavior |
|---|---|
| `BufferInfo` | id, name, modified, readonly, listed, loaded, line_count |
| `AlternateFile` | Tracks current/alternate buffer for Ctrl-^ switching |
| `BufferVariables` | Buffer-local variable store (b:var equivalent) |
| `format_buffer_info()` | Formats buffer for `:ls` display |

## Syntax commands

| Feature | Behavior |
|---|---|
| `parse_syntax_command()` | Parses `:syntax on/off/manual/enable/disable` |
| `detect_language()` | Maps file extension to language name (21 languages) |
| `parse_filetype_command()` | Parses `:setfiletype` argument |
| `format_syntax_info()` | Displays `syntax=on/off filetype=...` status |

## Command-line parser

| Feature | Behavior |
|---|---|
| `CmdlineState` | Tracks prefix, content, cursor position, history index |
| `CmdlineAction` | 16 editing actions (insert, delete, move, history, etc.) |
| `map_cmdline_key()` | Maps keys/ctrl/special to command-line actions |
| Editing | InsertChar, DeleteBack, DeleteWord, DeleteToStart, Move* |
| History | HistoryPrev, HistoryNext via Up/Down/Ctrl-p/Ctrl-n |

## File explorer

| Feature | Behavior |
|---|---|
| `ExplorerTree` | Tree model with NodeId, expand/collapse, hidden files |
| `TreeNode` | id, name, path, kind (File/Directory/Symlink), depth |
| `visible_nodes()` | Depth-first traversal respecting expanded/hidden/filter |
| `format_node()` | Renders with indentation and directional icons |
| `GitBadge` | Modified/Added/Deleted/Untracked/Ignored/Conflict/Clean |

## File I/O commands

| Feature | Behavior |
|---|---|
| `FileCommand` | Write/Edit/SaveAs/WriteQuit/WriteAll/Reload variants |
| `parse_file_command()` | Parses `:w`, `:e`, `:saveas`, `:wq`, `:wa`, `:e!` from input |
| `validate_write()` | Checks path exists, permissions, directory validity |
| `expand_tilde()` | Expands `~/` to home directory in file paths |
| `buffer_title()` | Derives display title from path or `[No Name]` |
| `display_path()` | Shortens home-relative paths with `~/` prefix |

## Completion engine

| Feature | Behavior |
|---|---|
| `CompletionSource` | Command/Path/Option/Buffer/Help/ColorScheme/Custom |
| `CompletionState` | Tracks candidates, index; next/prev/current/reset cycling |
| `detect_source()` | Infers completion source from cmdline prefix context |
| `complete_commands()` | Filters built-in command names by prefix |
| `complete_paths()` | Filesystem path completion with directory awareness |
| `common_prefix()` | Computes longest common prefix for menu narrowing |

## Config options

| Feature | Behavior |
|---|---|
| `OptionScope` | Global/Buffer/Window scope hierarchy |
| `ConfigStore` | Define/get/set/resolve options with scope precedence |
| `parse_set_arg()` | Parses `:set` arguments into SetAction variants |
| `SetAction` | ShowAll/Query/SetBool/SetInt/SetStr/Invalid |
| `build_defaults()` | 10 built-in options (number, wrap, tabstop, etc.) |

## Session persistence

| Feature | Behavior |
|---|---|
| `SessionState` | Marks, jumps, registers, history, buffer positions |
| `add_mark()` / `add_jump()` | Capped collections (100 marks, 100 jumps) |
| `add_register()` | Deduplicates by name, stores linewise flag |
| `add_history()` | Capped at 1000 entries, supports Command/Search/Input/Debug kinds |
| `serialize_session()` | JSON summary serialization |
| `filter_history()` | Filters history entries by HistoryKind |

## Event automation

| Feature | Behavior |
|---|---|
| `AutoEvent` | 17 events: BufEnter/Leave/Read/Write, Insert*, Cursor*, Win*, Vim*, FileType, etc. |
| `AutoPattern` | All / Glob (*.ext) / FileType matching |
| `AutoCmdRegistry` | Add/match/clear_group/remove_once_fired autocommands |
| `fire_event()` | Collects matching commands for an event firing |
| `once` flag | Single-fire autocommands removed after execution |

## Script files

| Feature | Behavior |
|---|---|
| `ScriptFile` | Parsed script with path and command lines |
| `ScriptLine` | ExCommand / Comment / Blank / Conditional variants |
| `parse_script()` | Parses file content into script lines |
| `executable_commands()` | Extracts only executable (non-comment/blank) lines |
| `SourceTracker` | Tracks sourced files, prevents double-sourcing |
| `resolve_source_path()` | Searches directories for script files with .vim fallback |

## User commands

| Feature | Behavior |
|---|---|
| `UserCommandDef` | Name, replacement, nargs, range/bang/bar/complete flags |
| `NArgs` | Zero/One/Any/AtLeastOne/ZeroOrOne with validation |
| `UserCommandRegistry` | define/get/remove/list/expand with uppercase enforcement |
| `expand()` | Substitutes `<args>`, `<q-args>`, `<bang>` in replacement |
| `parse_command_def()` | Parses `:command` arguments into definition |

### Notification Dispatch

| Aspect | Status |
| --- | --- |
| `Dispatcher` | Route notifications with severity filtering, auto-dismiss, max-visible limit |
| `Severity` | Debug / Info / Warning / Error with Ord ordering |
| `NotifySource` | Editor / Lsp / Plugin / Git / System source classification |
| `dismiss()` / `dismiss_source()` | Dismiss individual or by source |
| `gc()` | Garbage-collect old notifications past auto_dismiss_ms |
| `format_notification()` | Formats notification with severity prefix [D]/[I]/[W]/[E] |

### Git Full Integration

| Aspect | Status |
| --- | --- |
| `parse_diff()` | Parse unified diff format into DiffHunks with DiffLines |
| `parse_hunk_header()` | Extract line ranges from @@ hunk headers |
| `parse_log()` | Parse git log output into LogEntry records |
| `BlameEntry` / `BranchInfo` | Structured blame and branch metadata |
| `compute_signs()` | Map diff hunks to gutter GitSign indicators |
| `count_changes()` | Aggregate added/removed line counts |

### Session Full

| Aspect | Status |
| --- | --- |
| `SessionData` | Full session state with buffers, windows, tabs, global marks |
| `serialize_session()` | Serialize to vimscript-like format (cd, edit, cursor) |
| `parse_session_buffers()` | Parse session file to extract buffer paths |
| `SessionBuffer` / `SessionWindow` / `SessionMark` | Structured references for session restore |

### LSP Features

| Aspect | Status |
| --- | --- |
| `CompletionItem` / `CompletionKind` | 13 completion kinds with insert text resolution |
| `HoverInfo` | Hover contents with optional range |
| `Diagnostic` / `DiagnosticSeverity` | Error/Warning/Information/Hint with ordered severity |
| `DiagnosticStore` | Per-buffer diagnostics with error_count and total_count |
| `CodeAction` / `CodeActionKind` | QuickFix, Refactor, Source actions |
| `filter_completions()` | Case-insensitive prefix filtering |

### Crate Topology

| Aspect | Status |
| --- | --- |
| `CrateRole` | Core / Service / UI / IO / Host role classification |
| `check_dep_direction()` | Validates dependency direction rules (no service->host, core->host, etc.) |
| `expected_topology()` | 11-crate topology with verified dependencies |
| `validate_topology()` | Full graph validation returning list of violations |

### Feature Integration

| Aspect | Status |
| --- | --- |
| `IntegrationScenario` | Multi-step test scenarios with expected end state |
| `ScenarioStep` | OpenFile / TypeText / ExecuteCommand / SendKey / WaitMs / Assert |
| `validate_scenario()` | Dry-run validation of scenario structure |
| `open_edit_save_scenario()` / `undo_redo_scenario()` | Pre-built integration scenarios |

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Testing E2E: [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)
