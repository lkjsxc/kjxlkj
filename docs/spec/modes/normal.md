# Normal mode

Normal mode is the primary composition mode: motions + operators + commands.

## Requirements

- Normal mode interprets keys into typed intents only.
- All edits are applied by core as transactions.
- Async results (LSP, git, syntax) may influence UI, but not block input.

## Responsibilities

- Navigation (motions)
- Range composition (operator + motion/text object)
- Mode entry (insert/visual/command/replace)
- Repeatability (counts, dot-repeat)

## Key dispatch model

Normal mode dispatches keys through a tree of key handlers in this priority order:

1. **Count prefix** -- digits 1-9 start a count; `0` is a motion (start-of-line) unless preceded by digits.
2. **Register prefix** -- `"x` selects register x for the next operator or put command.
3. **Operator key** -- `d`, `c`, `y`, `>`, `<`, `=` enter OperatorPending mode awaiting a motion or text-object.
4. **Standalone command** -- single-key or multi-key sequence executed immediately.

## Count handling

| Input pattern | Effective count | Example |
|---|---|---|
| count + motion | count applied to motion | `3w` = 3 words forward |
| count + operator + motion | count applied to operator | `2dw` = delete 2 words |
| operator + count + motion | count applied to motion | `d3w` = delete 3 words |
| count1 + operator + count2 + motion | counts multiply | `2d3w` = delete 6 words |

## Insert entry keys

| Key | Action | Cursor position |
|---|---|---|
| `i` | Insert before cursor | At current position |
| `a` | Append after cursor | One right of current |
| `I` | Insert at first non-blank | First non-blank of line |
| `A` | Append at end of line | Past last character |
| `o` | Open line below | New line below |
| `O` | Open line above | New line above |
| `s` | Substitute character | Delete char under cursor, enter Insert |
| `S` | Substitute line | Delete line content, enter Insert |
| `C` | Change to end of line | Delete to EOL, enter Insert |
| `cc` | Change whole line | Delete line content, enter Insert |

Cursor rules: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Single-key commands

| Key | Action |
|---|---|
| `x` | Delete character under cursor (forward) |
| `X` | Delete character before cursor (backward) |
| `r{char}` | Replace character under cursor with {char} |
| `J` | Join current line with next, inserting a space |
| `~` | Toggle case of character under cursor, advance cursor |
| `.` | Repeat last text-changing command |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `p` | Put register contents after cursor |
| `P` | Put register contents before cursor |
| `&` | Repeat last `:s` substitution on current line |
| `ZZ` | Write buffer and quit |
| `ZQ` | Quit without writing |

## g-prefix commands

| Key | Action |
|---|---|
| `gg` | Go to first line (or line N with count) |
| `gd` | Go to local definition |
| `gD` | Go to global declaration |
| `gf` | Go to file under cursor |
| `ga` | Show character info (codepoint, byte values) |
| `gq{motion}` | Format text over motion |
| `gw{motion}` | Format text over motion, keep cursor position |
| `gv` | Reselect last visual selection |
| `g~{motion}` | Toggle case over motion (operator) |
| `gu{motion}` | Lowercase over motion (operator) |
| `gU{motion}` | Uppercase over motion (operator) |
| `gn` | Search forward and visually select match |
| `gN` | Search backward and visually select match |
| `g;` | Jump to older position in change list |
| `g,` | Jump to newer position in change list |
| `gi` | Resume insert at last insert position |
| `gJ` | Join lines without inserting a space |

## z-prefix commands

| Key | Action |
|---|---|
| `zz` | Center cursor line in viewport |
| `zt` | Scroll cursor line to top of viewport |
| `zb` | Scroll cursor line to bottom of viewport |
| `zo` | Open fold under cursor |
| `zc` | Close fold under cursor |
| `za` | Toggle fold under cursor |
| `zM` | Close all folds in buffer |
| `zR` | Open all folds in buffer |
| `zf{motion}` | Create fold over motion |

## Bracket commands

| Key | Action |
|---|---|
| `[c` / `]c` | Previous / next diff hunk |
| `[d` / `]d` | Previous / next diagnostic |
| `[[` / `]]` | Previous / next section start |
| `[{` / `]}` | Previous / next unmatched `{` |
| `[(` / `])` | Previous / next unmatched `(` |
| `[m` / `]m` | Previous / next method start |

## Mark commands

| Key | Action |
|---|---|
| `m{a-z}` | Set buffer-local mark |
| `m{A-Z}` | Set global (cross-buffer) mark |
| `'{mark}` | Jump to first non-blank on mark's line |
| `` `{mark} `` | Jump to exact mark position (line and column) |
| `''` | Jump to line of previous context |
| ` `` `` ` | Jump to exact position of previous context |

## Macro recording

| Key | Action |
|---|---|
| `q{a-z}` | Start recording keystrokes into register {a-z} |
| `q` | Stop recording (when recording is active) |
| `@{a-z}` | Execute macro stored in register {a-z} |
| `@@` | Replay last executed macro |
| `{count}@{a-z}` | Execute macro {count} times |

## Invariants

1. No key sequence in Normal mode MAY block on external IO.
2. Count is always optional and defaults to 1.
3. Register prefix is consumed by the next operator or put command only.
4. Dot-repeat replays the last text-changing command with its original count and register unless overridden.

## Related

- Motions/operators/text objects: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- Cursor rules: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Mode transitions: [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- Visual mode: [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- Commands: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)
- Keybinding hints: [/docs/spec/features/config/keybinding_hints.md](/docs/spec/features/config/keybinding_hints.md)
