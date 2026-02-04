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

## Search behavior

The currently shipped search behavior is:

- Search starts from cursor+1 position (forward) or cursor position (backward)
- Search wraps around file boundaries (`wrapscan` behavior)
- Pattern is stored and reused for `n`/`N` navigation
- Status message shows current search pattern and wrap status

## Related

- Modes and keys: [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
