# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)
Current implementation surface relative to the canonical spec.

## Purpose

The canonical spec under `/docs/spec/` describes the target system.

This document records the currently implemented, user-visible surface so that:

- spec language is not misread as “already implemented”
- tests can map to explicit supported behavior
- gaps are explicit and actionable

## Current surface (implemented)

### Modes

| Mode | Entry | Exit | Notes |
|---|---|---|---|
| Normal | startup | N/A | Command/navigation mode |
| Insert | `i`, `a`, `A`, `o` | `Esc` | Text insertion |
| Command | `:` | `Esc`, `Enter` | Ex command entry |
| Visual | `v` | `Esc` | Charwise selection with operators |
| Visual Line | `V` | `Esc` | Linewise selection with operators |
| Visual Block | `Ctrl-v` | `Esc` | Block (rectangular) selection |
| Replace | `R` | `Esc` | Overwrites existing characters |

### Normal-mode keys (subset)

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Cursor move left/down/up/right |
| Arrow keys | Cursor move |
| `Space` | Move right (same as `l`) |
| `Backspace` | Move left (same as `h`) |
| `0` | Move to start of line (column 0) |
| `^` | Move to first non-blank character |
| `_` | Move to first non-blank (with count offset) |
| `g_` | Move to last non-blank character |
| `gm` | Move to middle of line |
| `$` | Move to end of line |
| `\|` | Go to column N (with count) |
| `w` | Move to next word start |
| `W` | Move to next WORD start (same as `w` currently) |
| `b` | Move to previous word start |
| `B` | Move to previous WORD start (same as `b` currently) |
| `e` | Move to word end |
| `E` | Move to WORD end (same as `e` currently) |
| `ge` | Move to previous word end |
| `gE` | Move to previous WORD end (same as `ge` currently) |
| `+` | Move to first non-blank of next line |
| `Enter` | Move to first non-blank of next line (same as `+`) |
| `-` | Move to first non-blank of previous line |
| `gg` | Move to file start |
| `G` | Move to file end |
| `{count}gg` | Go to line N |
| `{count}G` | Go to line N |
| `{count}%` | Go to N% of file |
| `H` | Move to top of visible screen |
| `M` | Move to middle of visible screen |
| `L` | Move to bottom of visible screen |
| `zz` | Scroll cursor to center of screen |
| `zt` | Scroll cursor to top of screen |
| `zb` | Scroll cursor to bottom of screen |
| `z<CR>` | Scroll cursor to top, move to first non-blank |
| `z.` | Scroll cursor to center, move to first non-blank |
| `z-` | Scroll cursor to bottom, move to first non-blank |
| `i` | Enter Insert mode |
| `I` | Enter Insert mode (first non-blank of line) |
| `a` | Enter Insert mode (after cursor) |
| `A` | Enter Insert mode (end of line) |
| `o` | Open line below and enter Insert mode |
| `O` | Open line above and enter Insert mode |
| `v` | Enter Visual mode |
| `V` | Enter Visual line mode |
| `R` | Enter Replace mode |
| `r{char}` | Replace character under cursor with {char} |
| `x` | Delete character under cursor |
| `X` | Delete character before cursor |
| `D` | Delete from cursor to end of line |
| `C` | Change from cursor to end of line |
| `s` | Substitute character under cursor (delete and enter Insert) |
| `S` | Substitute entire line (delete content and enter Insert) |
| `Y` | Yank current line (like `yy`) |
| `p` | Paste after cursor |
| `P` | Paste before cursor |
| `gp` | Paste after cursor, cursor at end of pasted text |
| `gP` | Paste before cursor, cursor at end of pasted text |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `:` | Enter Command mode |
| `.` | Repeat last change |
| `*` | Search forward for word under cursor |
| `#` | Search backward for word under cursor |
| `g*` | Search forward for partial word under cursor |
| `g#` | Search backward for partial word under cursor |
| `m{a-z}` | Set local mark |
| `` ` ``{a-z} | Jump to mark (exact position) |
| `'{a-z}` | Jump to mark (line, first non-blank) |
| `"{a-z}` | Select register for next yank/delete/paste |
| `q{a-z}` | Start/stop macro recording |
| `@{a-z}` | Play macro from register |
| `@@` | Repeat last macro |
| `Ctrl-o` | Jump to older position in jump list |
| `Ctrl-i` | Jump to newer position in jump list |
| `g;` | Jump to older position in change list |
| `g,` | Jump to newer position in change list |
| `(` | Move to previous sentence |
| `)` | Move to next sentence |
| `{` | Move to previous paragraph |
| `}` | Move to next paragraph |
| `%` | Move to matching bracket |
| `[(` | Move to previous unmatched `(` |
| `])` | Move to next unmatched `)` |
| `[{` | Move to previous unmatched `{` |
| `]}` | Move to next unmatched `}` |
| `ZZ` | Write and quit |
| `ZQ` | Quit without saving |
| `J` | Join current line with next (adds space) |
| `gJ` | Join current line with next (no space) |
| `~` | Toggle case of character under cursor |
| `g~{motion}` | Toggle case over motion |
| `g~~` | Toggle case of entire line |
| `gU{motion}` | Uppercase over motion |
| `gUU` | Uppercase entire line |
| `gu{motion}` | Lowercase over motion |
| `guu` | Lowercase entire line |
| `Ctrl-a` | Increment number under cursor |
| `Ctrl-x` | Decrement number under cursor |
| `Ctrl-d` | Scroll half page down |
| `Ctrl-u` | Scroll half page up |
| `Ctrl-f` | Scroll full page down |
| `Ctrl-b` | Scroll full page up |
| `Ctrl-e` | Scroll one line down (cursor stays) |
| `Ctrl-y` | Scroll one line up (cursor stays) |

### Operators and motions

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

### Find character motions

| Key | Action |
|---|---|
| `f{char}` | Move cursor to next occurrence of {char} on line |
| `F{char}` | Move cursor to previous occurrence of {char} on line |
| `t{char}` | Move cursor to just before next occurrence of {char} |
| `T{char}` | Move cursor to just after previous occurrence of {char} |
| `;` | Repeat last f/t/F/T motion |
| `,` | Repeat last f/t/F/T motion in opposite direction |

### Text objects

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

### Search

| Key | Mode | Action |
|---|---|---|
| `/` | Normal | Enter forward search mode |
| `?` | Normal | Enter backward search mode |
| `n` | Normal | Repeat last search (same direction) |
| `N` | Normal | Repeat last search (opposite direction) |

Search behavior:
- Search starts from cursor+1 position (forward) or cursor position (backward)
- Search wraps around file boundaries (`wrapscan` behavior)
- Pattern is stored and reused for `n`/`N` navigation
- Status message shows current search pattern and wrap status

### Visual mode

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Extend selection (cursor movement) |
| `w`/`b`/`e` | Extend selection by word |
| `0`/`^`/`$` | Extend selection to line boundaries |
| `gg`/`G` | Extend selection to file boundaries |
| `d`/`x` | Delete selection |
| `y` | Yank selection |
| `c`/`s` | Change selection (delete and enter Insert mode) |
| `o` | Swap cursor to other end of selection |
| `>` | Indent selection |
| `<` | Outdent selection |
| `Esc` | Cancel selection, return to Normal mode |

Visual Line mode (`V`) operates on entire lines for all operators.

Visual Block mode (`Ctrl-v`) operates on rectangular regions:

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Extend block selection |
| `d`/`x` | Delete block (rectangular region from each line) |
| `y` | Yank block |
| `c` | Change block (delete and enter Insert mode) |
| `Esc` | Cancel selection, return to Normal mode |

### Insert mode

| Key | Action |
|---|---|
| `Esc` | Return to Normal mode |
| (any char) | Insert character at cursor |
| `Backspace` | Delete character before cursor |
| `Ctrl-h` | Delete character before cursor (same as Backspace) |
| `Enter` | Insert newline |
| `Ctrl-j` | Insert newline (same as Enter) |
| `Ctrl-m` | Insert newline (same as Enter) |
| Arrow keys | Move cursor |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete to start of line |
| `Ctrl-t` | Indent current line |
| `Ctrl-d` | Outdent current line |
| `Ctrl-y` | Copy character from line above |
| `Ctrl-e` | Copy character from line below |
| `Ctrl-r {reg}` | Insert contents of register |

### Replace mode

| Key | Action |
|---|---|
| `Esc` | Return to Normal mode |
| (any char) | Replace character at cursor and advance |
| `Backspace` | Move cursor left |

At end of line, typed characters are inserted rather than replacing.

### Command-line (Ex) commands (subset)

| Command | Behavior |
|---|---|
| `:q` / `:q!` | Quit (forced with `!`) |
| `:qa` / `:qa!` | Alias for quit / forced quit |
| `:w` | Write to current buffer path (if set) |
| `:w {file}` | Write to `{file}` |
| `:wa` | Alias for `:w` |
| `:wq` / `:x` | Write then quit |
| `:wq {file}` | Write to `{file}` then quit |
| `:e {file}` / `:e! {file}` | Edit file (forced with `!`) |
| `:! {cmd}` | Run `{cmd}` via terminal service and display first output line as status |
| `:s/pattern/replacement/` | Substitute on current line |
| `:s/pattern/replacement/g` | Substitute all occurrences on current line |
| `:g/pattern/d` | Delete all lines matching pattern |
| `:g/pattern/command` | Execute command on matching lines |
| `:v/pattern/d` | Delete all lines NOT matching pattern (inverted global) |

### Headless test runner

The shipped binary supports a deterministic headless mode for E2E tests:

- `--headless --script {path}` runs an event script without terminal UI.
- The script MAY be either:
  - a JSON array of keys, where each item is a `Key` object with `code` and `mods`
  - a JSON array of steps, where each item is a tagged object with `kind`

## Related

- Limitations: [docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Keybindings (target): [docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
