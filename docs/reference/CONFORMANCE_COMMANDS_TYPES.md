# Conformance: Command and Editor Types

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Type definitions for search, undo, DAP, marks, substitution, completion, and more.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Search highlights | `implemented` | search module tests |
| Undo/redo | `implemented` | contract_tests.rs, headless_e2e_tests.rs |
| Marks | `implemented` | editor_auto_marks module tests |
| Substitution | `implemented` | feature_tests.rs (substitute) |
| DAP types | `scaffold-only` | Types exist, no adapter process |

## Search highlight types

| Component | Behavior |
|---|---|
| `SearchMatch` | Start/end position + is_current flag for each match |
| `SearchHighlights` | Match collection with next/prev cycling, hlsearch toggle, visible filtering |

## Undo branching types

| Component | Behavior |
|---|---|
| `BranchingUndoTree` | Tree-structured undo with parent/children, branch selection, path traversal |
| `NodeId` / `ChangeEntry` | Typed node IDs and forward/reverse patch entries |

## DAP debugging types

| Component | Behavior |
|---|---|
| `DapState` | Debug session state with breakpoints, stack frames, variables |
| `Breakpoint` | Line/conditional/logpoint/function/data breakpoints with toggle |
| `StackFrame` / `Variable` | Call stack traversal and variable inspection |

## Extended marks types

| Component | Behavior |
|---|---|
| `MarkScope` | Local (a-z, special) vs Global (A-Z) mark classification |
| `Mark` / `MarkRegistry` | Per-buffer local marks, cross-buffer global marks, special marks (`[`, `]`, `<`, `>`) |

## Substitute flags types

| Component | Behavior |
|---|---|
| `SubstituteFlags` | Parse g/c/i/I/n/& flags from `:s///flags` |
| `ConfirmState` | Interactive `:s///c` confirmation with Yes/No/All/Quit responses |
| `parse_substitute_cmd()` | Parse full `:s/pattern/replacement/flags` command string |

## Extended completion types

| Component | Behavior |
|---|---|
| `CompletionItemKind` | 25 LSP completion kinds with icon() and from_lsp() mapping |
| `CompletionItemEx` | Rich completion item with filter_text, sort_text, preselect, deprecated |
| `CompletionList` | Filterable/selectable completion list with prefix matching |

## Buffer list types

| Component | Behavior |
|---|---|
| `BufferFilter` | All/Listed/Unlisted/Modified/Active buffer filtering |
| `BufferListEntry` | Buffer metadata with flags() for `:ls` display formatting |
| `build_buffer_list()` | Build filtered buffer list from EditorState buffers |

## Visual block types

| Component | Behavior |
|---|---|
| `BlockSelection` | Two-corner block selection with line/col range, height/width |
| `BlockOp` / `BlockEdit` | Insert/Append/Change/Delete block operations with per-line edits |
| `extend_to_eol()` | Extend block selection to end-of-line per row (like `$` in visual block) |

## Command-line completion types

| Component | Behavior |
|---|---|
| `complete_command()` | Prefix-match against 55 built-in command names with scoring |
| `complete_option()` | Prefix-match against 21 `:set` option names |
| `complete_buffer()` | Match buffer names by prefix or substring |
| `detect_completion_kind()` | Infer completion type from command-line context (set->option, buffer->buffer, edit->path) |

## Keybinding DSL types

| Component | Behavior |
|---|---|
| `KeyChord` | Parsed key with ctrl/alt/shift modifiers and display() round-trip |
| `parse_key_sequence()` | Parse `<C-x>`, `<M-a>`, `<leader>`, `<CR>`, combined modifiers, plain chars |
| `validate_key_sequence()` | Check for unclosed `<` brackets and syntax errors |
| `resolve_special()` | Map CR/Esc/BS/Tab/Space/arrows/Del/Home/End/PageUp/PageDown to canonical names |

## View tree types

| Component | Behavior |
|---|---|
| `FocusTarget` | Editor/CommandLine/Explorer/Popup(id)/Notification focus targets |
| `ViewNode` | Tree node with id, kind, rect, focusable flag, and children |
| `ViewTree` | Root view tree with focus stack (push/pop), from_splits() layout builder |

## Popup menu types

| Component | Behavior |
|---|---|
| `PopupMenu` | Item list with selection cycling, scroll window, anchor-based positioning |
| `HoverTooltip` | Positioned text tooltip with dismiss |
| `PopupAnchor` | AboveCursor/BelowCursor/ScreenCenter/AtPosition anchor modes |
| `compute_rect()` | Calculate popup rectangle constrained to screen bounds |

## Status line layout types

| Component | Behavior |
|---|---|
| `StatusSection` | Left/Center/Right aligned section with priority |
| `StatusLineLayout` | Compose sections into fixed-width rendered line |
| `vim_default()` | Standard Vim-like status format: mode, filename, modified, line:col, percent |

## Contract checker types

| Component | Behavior |
|---|---|
| `ContractChecker` | Accumulate pass/fail contract results with summary |
| `check_viewport_bounded()` | Verify snapshot doesn't clone entire buffer |
| `check_input_ordering()` | Verify monotonic input sequence numbers |
| `check_bus_utilization()` | Warn when message bus >90% capacity |
| `check_no_plugin_loading()` | Assert no dynamic plugin loading (built-in only) |
| `check_restart_limit()` | Verify service restart count within policy |

## User command execution types

| Component | Behavior |
|---|---|
| `ExecResult` | Ok / Error(message) / NoSuchCommand result from user command dispatch |
| `validate_nargs()` | Validate argument count against NArgs spec (0/1/*/+/?) |
| `substitute_args()` | Replace `<args>`, `<bang>`, `<line1>`, `<line2>`, `<count>`, `<q-args>` in command template |
| `execute_user_command()` | Look up registered command + validate nargs + substitute args |
| `dispatch_user_command()` | Full dispatch: find command, validate, substitute, return expanded command body |

## User function execution types

| Component | Behavior |
|---|---|
| `FuncResult` | Value(String) / Void / Error(String) result from function execution |
| `FuncContext` | Function execution context with args, locals, return value tracking |
| `execute_function()` | Interpret function body: let assignments, return, concat expressions |
| `parse_let()` | Parse `let var = expr` assignments within function body |
| `resolve_expression()` | Resolve variable references (`a:`, `l:` scope) and string concatenation |

## Debounce manager types

| Component | Behavior |
|---|---|
| `FakeClock` | Deterministic fake clock for testing with advance() |
| `PendingAction` | Scheduled action with deadline and coalesced_count |
| `DebounceManager` | Schedule/cancel/tick debounce actions with coalescing |
| `fired_actions()` | Report which actions fired after tick based on fake clock time |

## Mapping expansion types

| Component | Behavior |
|---|---|
| `MappingEntry` | Mode-scoped mapping from trigger keys to replacement keys |
| `ExpansionResult` | Expanded(keys) / NoMapping / RecursionLimit result |
| `expand_mapping()` | Longest-prefix match for one-level mapping expansion |
| `expand_recursive()` | Recursive expansion with MAX_DEPTH=100 guard |
| `has_prefix_match()` | Check if partial input has a potential mapping prefix |
| `list_mappings()` | List all mappings for a given mode |

## Accessibility types

| Component | Behavior |
|---|---|
| `ContrastRatio` | WCAG 2.1 relative luminance and contrast ratio computation |
| `luminance()` | Compute relative luminance from sRGB (0-255) tuple |
| `contrast_ratio()` | Compute L1/L2 contrast ratio between two colors |
| `FocusIndicator` | Underline/Reverse/Bold/HighContrast focus indicator styles |
| `A11yCheck` | pass/fail named accessibility check result |
| `check_color_scheme()` | Verify foreground/background meet WCAG AA 4.5:1 minimum |
| `check_focus_visible()` | Verify focus indicator is not None |
| `AriaHint` | Status/Editor/Menu ARIA role hints for screen readers |

## Related

- Ex commands: [/docs/reference/CONFORMANCE_COMMANDS.md](/docs/reference/CONFORMANCE_COMMANDS.md)
- Testing: [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)
