# Conformance: Modes and Core Keybindings

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Mode set and core keybindings (Normal, Visual) in the conformance ledger.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Mode transitions | `implemented` | 523 tests passing; dispatch_tests.rs, integration_tests.rs, boundary_tests |
| Normal-mode keys | `implemented` | feature_tests.rs, boundary_tests |
| Visual mode keys | `implemented` | dispatch_tests.rs (visual_delete, visual_yank) |
| Terminal mode | `partial` | SpawnTerminal action dispatches; no real PTY fork |

## Modes

| Mode | Entry | Exit | Notes |
|---|---|---|---|
| Normal | startup | N/A | Command/navigation mode |
| Insert | `i`, `a`, `A`, `o` | `Esc` | Text insertion |
| Command | `:` | `Esc`, `Enter` | Ex command entry |
| Visual | `v` | `Esc` | Charwise selection with operators |
| Visual Line | `V` | `Esc` | Linewise selection with operators |
| Visual Block | `Ctrl-v` | `Esc` | Block (rectangular) selection |
| Replace | `R` | `Esc` | Overwrites existing characters |
| Terminal | `<leader>t`, `:terminal` | `Esc` (to Normal) | Opens terminal scratch panel; full PTY integration pending |

## Normal-mode keys (subset)

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Cursor move left/down/up/right |
| Arrow keys | Cursor move |
| `Space` | Leader prefix (default leader); feature chords: `Space e` (explorer), `Space t` (terminal), `Space f` (find), `Space g` (livegrep), `Space b` (buffers), `Space u` (undotree) |
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
| `/` | Search forward |
| `?` | Search backward |
| `n` | Repeat last search (same direction) |
| `N` | Repeat last search (opposite direction) |
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
| `Ctrl-6` / `Ctrl-^` | Switch to alternate/previous buffer |
| `Ctrl-g` | Display file info (same as `:file`) |
| `gv` | Reselect last visual selection |
| `Ctrl-d` | Scroll half page down |
| `Ctrl-u` | Scroll half page up |
| `Ctrl-f` | Scroll full page down |
| `Ctrl-b` | Scroll full page up |
| `Ctrl-e` | Scroll one line down (cursor stays) |
| `Ctrl-y` | Scroll one line up (cursor stays) |

## Visual mode

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

## Related

- Input modes: [/docs/reference/conformance/CONFORMANCE_KEYS_INPUT.md](/docs/reference/conformance/CONFORMANCE_KEYS_INPUT.md)
- Editing semantics: [/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md](/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md)
