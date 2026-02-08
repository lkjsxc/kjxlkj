# Conformance: Editing Operators, Motions, and Text Objects

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Operator, motion, and text-object semantics in the conformance ledger.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Operators (d/y/c) | `implemented` | dispatch_tests.rs, boundary_tests (BD-39) |
| Linewise operators (dd/yy/cc) | `implemented` | DoubleOperator tests |
| Motions (hjkl, w/b/e, 0/$, gg/G) | `implemented` | integration_tests.rs, boundary_tests (BD-38) |
| Find char (f/t/F/T) | `implemented` | dispatch_tests.rs |
| Text objects (iw/aw/i"/a") | `implemented` | text_object tests |
| Search motions (/,?) | `implemented` | feature_tests.rs, boundary_tests (BD-29) |

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

## Extended text objects

| Feature | Behavior |
|---|---|
| `ExtTextObject` | Argument/IndentLevel/EntireBuffer/Line/Number/Url |
| `find_argument()` | Comma-delimited argument within parens/brackets, inner/outer |
| `find_indent_level()` | Contiguous lines with same or deeper indentation |
| `find_entire_buffer()` | Entire buffer content, inner skips leading/trailing blank lines |
| `find_number()` | Numeric literal under cursor |
| `TextRange` | start/end line/col with contains() and is_empty() |

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

- Search starts from cursor+1 position (forward) or cursor position (backward)
- Search wraps around file boundaries (`wrapscan` behavior)
- Pattern is stored and reused for `n`/`N` navigation
- Status message shows current search pattern and wrap status
- `*` / `#` search for word under cursor (forward/backward) with `\b` word boundaries
- `:noh` / `:nohlsearch` clears search highlight
- Regex support: Vim magic-mode patterns (`.`, `*`, `^`, `$`, `[]`, `\d`, `\w`, `\s`, `\<`, `\>`, `\+`, `\?`, `\|`, `\(`, `\)`)
- Case-insensitive search with `ignorecase` option, smart-case with `smartcase` option
- Literal fallback when pattern has no metacharacters

## Related

- Editing features: [/docs/reference/CONFORMANCE_EDITING_FEATURES.md](/docs/reference/CONFORMANCE_EDITING_FEATURES.md)
- Modes and keys: [/docs/reference/CONFORMANCE_MODES.md](/docs/reference/CONFORMANCE_MODES.md)
