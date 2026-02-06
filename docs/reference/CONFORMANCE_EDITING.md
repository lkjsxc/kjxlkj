# Conformance: Editing Semantics

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
Editing-related semantics in the conformance ledger.

In a docs-only baseline, treat this as the intended initial reconstruction target (update it after regenerating the implementation).

## Operators and motions

| Operator | Motion/target | Action |
|---|---|---|
| `d` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}`/`(`/`)`/`{`/`}`/`%` | Delete over motion, yank to register |
| `y` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}`/`(`/`)`/`{`/`}`/`%` | Yank over motion |
| `c` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}`/`(`/`)`/`{`/`}`/`%` | Change (delete then enter Insert) |
| `dd` | (line) | Delete current line (yanks deleted text) |
| `yy` | (line) | Yank current line |
| `cc` | (line) | Change current line |
| `>>` | (line) | Indent current line (4 spaces) |
| `<<` | (line) | Outdent current line (up to 4 spaces) |
| `x` | (char) | Delete character under cursor |
| `X` | (char) | Delete character before cursor |
| `D` | (to end) | Delete from cursor to end of line |
| `C` | (to end) | Change from cursor to end of line |
| `s` | (char) | Substitute character (delete and enter Insert) |
| `S` | (line) | Substitute line (delete line and enter Insert) |
| `J` | (join) | Join current line with next (add space) |
| `gJ` | (join) | Join current line with next (no space) |
| `~` | (char) | Toggle case of character under cursor |
| `gU{motion}` | (uppercase) | Uppercase over motion |
| `gu{motion}` | (lowercase) | Lowercase over motion |

## Find character motions

| Key | Action |
|---|---|
| `f{char}` | Move cursor to next occurrence of {char} on line |
| `F{char}` | Move cursor to previous occurrence of {char} on line |
| `t{char}` | Move cursor to just before next occurrence of {char} |
| `T{char}` | Move cursor to just after previous occurrence of {char} |
| `;` | Repeat last f/t/F/T motion |
| `,` | Repeat last f/t/F/T motion in opposite direction |

## Text objects

Operators (`d`, `y`, `c`) can be combined with text objects:

| Text object | Description |
|---|---|
| `iw` | Inner word (word characters only) |
| `aw` | Around word (word + trailing/leading whitespace) |
| `iW` | Inner WORD (non-whitespace sequence) |
| `aW` | Around WORD (WORD + whitespace) |
| `i"` | Inner double quotes (content between quotes) |
| `a"` | Around double quotes (content including quotes) |
| `i'` | Inner single quotes |
| `a'` | Around single quotes |
| `i(` / `i)` / `ib` | Inner parentheses |
| `a(` / `a)` / `ab` | Around parentheses |
| `i[` / `i]` | Inner brackets |
| `a[` / `a]` | Around brackets |
| `i{` / `i}` / `iB` | Inner braces |
| `a{` / `a}` / `aB` | Around braces |
| `` i` `` | Inner backtick quotes |
| `` a` `` | Around backtick quotes |
| `i<` / `i>` | Inner angle brackets |
| `a<` / `a>` | Around angle brackets |
| `ip` | Inner paragraph |
| `ap` | Around paragraph |
| `is` | Inner sentence |
| `as` | Around sentence |
| `it` | Inner tag (HTML/XML) |
| `at` | Around tag (HTML/XML) |

## Autopairs

| Feature | Behavior |
|---|---|
| Auto-close `(` | Inserts `)` after cursor when `autopairs` enabled |
| Auto-close `[` | Inserts `]` after cursor when `autopairs` enabled |
| Auto-close `{` | Inserts `}` after cursor when `autopairs` enabled |
| Skip-over `)` | When typing `)` at `)`, skips over instead of inserting |
| Skip-over `]` | When typing `]` at `]`, skips over instead of inserting |
| Skip-over `}` | When typing `}` at `}`, skips over instead of inserting |
| Off by default | `:set autopairs` enables, `:set noautopairs` disables |

## Digraphs

| Key | Action |
|---|---|
| `Ctrl-K {c1} {c2}` | Insert digraph character in Insert mode |
| `:digraphs` / `:dig` | Display digraph table |

## Search behavior

Search behavior in this conformance target is:

- Search starts from cursor+1 position (forward) or cursor position (backward)
- Search wraps around file boundaries (`wrapscan` behavior)
- Pattern is stored and reused for `n`/`N` navigation
- Status message shows current search pattern and wrap status
- `*` / `#` search for word under cursor (forward/backward) with `\b` word boundaries
- `:noh` / `:nohlsearch` clears search highlight
- Regex support: Vim magic-mode patterns (`.`, `*`, `^`, `$`, `[]`, `\d`, `\w`, `\s`, `\<`, `\>`, `\+`, `\?`, `\|`, `\(`, `\)`)
- Case-insensitive search with `ignorecase` option, smart-case with `smartcase` option
- Literal fallback when pattern has no metacharacters

## Completion

| Feature | Behavior |
|---|---|
| Buffer-word completion | `collect_buffer_words()` — unique words from buffer matching prefix |
| Line completion | `collect_line_completions()` — matching lines for Ctrl-X Ctrl-L |
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
| `translate_vim_pattern()` | Converts `\<`→`\b`, `\(`→`(`, `\)`→`)`, `\+`→`+`, `\|`→`|`, `\{`→`{`, `\}`→`}` |
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
| `swap_ends()` | Implements `o` command — swap anchor and cursor |
| `block_cols()` | Returns column range for block selections |
| `switch_kind()` | Switches between Char/Line/Block visual modes |

## Related

- Modes and keys: [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
