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
| Replace | `R` | `Esc` | Currently behaves like Insert |

### Normal-mode keys (subset)

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Cursor move left/down/up/right |
| Arrow keys | Cursor move |
| `0` | Move to start of line (column 0) |
| `^` | Move to first non-blank character |
| `$` | Move to end of line |
| `w` | Move to next word start |
| `W` | Move to next WORD start (same as `w` currently) |
| `b` | Move to previous word start |
| `B` | Move to previous WORD start (same as `b` currently) |
| `e` | Move to word end |
| `E` | Move to WORD end (same as `e` currently) |
| `gg` | Move to file start |
| `G` | Move to file end |
| `i` | Enter Insert mode |
| `a` | Enter Insert mode (after cursor) |
| `A` | Enter Insert mode (end of line) |
| `o` | Open line below and enter Insert mode |
| `v` | Enter Visual mode |
| `V` | Enter Visual line mode |
| `R` | Enter Replace mode |
| `x` | Delete character under cursor |
| `p` | Paste after cursor |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `:` | Enter Command mode |
| `.` | Repeat last change |
| `m{a-z}` | Set local mark |
| `` ` ``{a-z} | Jump to mark (exact position) |
| `'{a-z}` | Jump to mark (line, first non-blank) |
| `"{a-z}` | Select register for next yank/delete/paste |
| `q{a-z}` | Start/stop macro recording |
| `@{a-z}` | Play macro from register |
| `@@` | Repeat last macro |
| `Ctrl-o` | Jump to older position in jump list |
| `Ctrl-i` | Jump to newer position in jump list |

### Operators and motions

| Operator | Motion/target | Action |
|---|---|---|
| `d` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}` | Delete over motion, yank to register |
| `y` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}` | Yank over motion |
| `c` | `w`/`b`/`e`/`h`/`l`/`0`/`^`/`$`/`gg`/`G`/`f{c}`/`t{c}` | Change (delete then enter Insert) |
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
| `c` | Change selection (delete and enter Insert mode) |
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
