# Conformance: Editing Features and Rendering

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Editing-related features, rendering, and UI subsystems in the conformance ledger.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Completion engine | `implemented` | completion module tests |
| CJK cursor handling | `implemented` | cjk_support tests, REG-07 |
| Line wrapping | `implemented` | line_wrap tests, REG-02, REG-08 |
| Theming | `implemented` | theming module tests |
| Statusline | `implemented` | statusline_dsl module tests |

## Completion

| Feature | Behavior |
|---|---|
| Buffer-word completion | `collect_buffer_words()` -- unique words from buffer matching prefix |
| Line completion | `collect_line_completions()` -- matching lines for Ctrl-X Ctrl-L |
| CompletionMenu | Open/close/select_next/select_prev/filter/current |
| CompletionSource | Buffer, Path, Line, Lsp, Dictionary, Command |
| CompletionKind | 13 variants (Variable, Function, Method, Class, etc.) |

## Regex engine

| Feature | Behavior |
|---|---|
| `compile_pattern()` | Compiles Vim-flavored regex with case sensitivity flag |
| `find_all_matches()` | Iterates all matches with byte offsets and capture groups |
| `find_next()` | First match at or after offset (with global position adjustment) |
| `find_prev()` | Last match before offset |
| `translate_vim_pattern()` | Converts `\<`->`\b`, `\(`->`(`, `\)`->`)`, `\+`->`+`, `\|`->`|`, `\{`->`{`, `\}`->`}` |
| Case-insensitive | Prepends `(?i)` when `case_sensitive=false` |

## Notification rendering

| Feature | Behavior |
|---|---|
| `NotifPosition` | TopRight, BottomRight, TopCenter, BottomCenter |
| `render_notification()` | Generates `RenderedNotif` with row/col, content lines, wrapped text |
| `wrap_text()` | Word-wraps notification text at specified width |
| `max_visible_notifications()` | Computes max visible based on terminal height |

## Cursor visibility

| Feature | Behavior |
|---|---|
| `CursorShape` | Block, Line, Underline |
| `BlinkState` | On, Off |
| `ModeCursorConfig` | Per-mode shape + blink settings |
| `cursor_for_mode()` | Returns cursor shape for given mode |
| `check_cursor_in_viewport()` | Validates cursor within viewport bounds |
| `check_transition_visibility()` | Ensures cursor visible after mode transition |
| `cursor_shape_escape()` | Generates terminal escape sequence for cursor shape |

## Text manipulation

| Feature | Behavior |
|---|---|
| `join_lines()` | Joins lines with separator, trims trailing whitespace |
| `convert_case()` | Upper, Lower, Toggle, Title case conversion |
| `sort_lines()` | Alphabetical sort with unique/reverse options |
| `trim_trailing()` | Remove trailing whitespace from each line |
| `reverse_chars()` | Reverse character order |
| `indent_level()` | Compute indentation level (spaces + tabs) |
| `reindent()` | Set line to target indent level with tabs or spaces |

## Syntax highlight groups

| Feature | Behavior |
|---|---|
| `HighlightGroup` | 31 standard groups (Comment, String, Keyword, Function, Type, etc.) |
| `token_to_group()` | Maps token type strings to highlight groups |
| `default_highlight_styles()` | 7 default dark-theme styles with fg/bold/italic |
| `highlight_line()` | Produces `HighlightSpan` list from tokenized text |

## Layout invariants

| Feature | Behavior |
|---|---|
| `LayoutRect.overlaps()` | Detects region overlap |
| `LayoutRect.fits_in()` | Validates region fits within terminal bounds |
| `check_layout_invariants()` | Checks overlaps, bounds, cursor, statusline |
| `check_vertical_coverage()` | Verifies regions tile without vertical gaps |

## Keybinding coverage

| Feature | Behavior |
|---|---|
| `CoverageMap` | Tracks bindings with mode, keys, tested, documented flags |
| `untested()` / `undocumented()` | Find gaps in coverage |
| `coverage_pct()` | Percentage of tested bindings |
| `find_duplicates()` | Detect duplicate bindings within same mode |
| `build_default_normal_coverage()` | Default Normal mode map with 23 core keys |

## Theme engine

| Feature | Behavior |
|---|---|
| `ThemeColor` | RGBA color with rgb/rgba constructors and `to_ansi256()` |
| `StyleRule` | Scope-to-color mapping with bold/italic/underline |
| `Theme` | Full theme definition (dark/light) with editor, cursor, statusline colors |
| `default_dark_theme()` / `default_light_theme()` | Built-in themes |
| `resolve_scope()` | Look up style for a scope name |
| `apply_override()` | Merge user override into theme |

## Visual selection

| Feature | Behavior |
|---|---|
| `VisualSelection` | Tracks kind (Char/Line/Block), anchor, cursor positions |
| `contains()` | Tests if position is within selection for all three kinds |
| `extract_selection()` | Extracts selected text as line fragments |
| `swap_ends()` | Implements `o` command -- swap anchor and cursor |
| `block_cols()` | Returns column range for block selections |
| `switch_kind()` | Switches between Char/Line/Block visual modes |

## Cursor overlay

| Feature | Behavior |
|---|---|
| `OverlayPriority` | Base < Selection < Search < Diagnostic < Cursor layering |
| `HighlightRegion` | Start/end/kind region with containment test |
| `effective_overlay()` | Resolves highest-priority overlay at a position |
| `BoundaryAction` | Clamp/Wrap/NoOp for cursor boundary resolution |
| `resolve_cursor_col()` | Clamps or wraps cursor column within line length |
| `cursor_in_viewport()` | Tests if cursor position is visible in viewport |
| `matching_bracket()` | Finds matching bracket (parens/brackets/braces) |

## Keybinding coverage (extended)

| Feature | Behavior |
|---|---|
| `BindingCoverage` | Coverage tracker with register/set_status/add_test |
| `BindingStatus` | Implemented / Partial / Stub / NotImplemented |
| `coverage_percent()` | Reports implementation coverage percentage |
| `untested()` / `not_implemented()` | Lists gaps in testing and implementation |
| `build_normal_coverage()` | 35+ normal-mode keybindings mapped with descriptions |

## Popup and overlay management

| Aspect | Status |
| --- | --- |
| `PopupState` | State for completion/hover/signature/context/wildmenu/command-palette popups |
| `show()` / `hide()` | Show items with auto-select first, hide and clear |
| `select_next()` / `select_prev()` | Navigate selection with scroll offset tracking |
| `visible_items()` | Return windowed slice respecting max_visible and scroll_offset |
| `OverlayManager` | Layered popup stack with open/close_kind/close_all |
| `compute_popup_rect()` | Position popup by anchor (Cursor/TopLeft/Center/CmdLine) with screen bounds |

## Cursor rendering

| Aspect | Status |
| --- | --- |
| `CursorShape` | Block / Line / Underline / Hidden |
| `ModeCursorMap` | Mode-specific shapes (normal=Block, insert=Line, replace=Underline) |
| `cursor_shape_escape()` | Terminal escape sequences for all shape+blink combinations |
| `blink_state()` | Timestamp-based blink on/off calculation |
| `cursor_visible()` | Visibility check combining shape and blink state |

## Related

- Editing operators: [/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md](/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md)
- Modes and keys: [/docs/reference/conformance/CONFORMANCE_MODES.md](/docs/reference/conformance/CONFORMANCE_MODES.md)
