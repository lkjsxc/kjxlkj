# Jump Marks

Mark-based cursor navigation using `'` and backtick.

## Two Jump Styles (normative)

| Key | Target | Movement type |
|---|---|---|
| `'m` | Line of mark m | First non-blank of that line (linewise) |
| `` `m `` | Position of mark m | Exact line + column (characterwise) |

## Local Mark Jumps

| Command | Action |
|---|---|
| `'a` – `'z` | Jump to line of local mark |
| `` `a `` – `` `z `` | Jump to exact position of local mark |

Local marks are per-buffer. Each buffer has its own `a`-`z` marks.

## Global Mark Jumps

| Command | Action |
|---|---|
| `'A` – `'Z` | Jump to line of global mark (may switch buffer) |
| `` `A `` – `` `Z `` | Jump to exact position of global mark |

Global marks remember both the file and position. Jumping to a global mark opens the file if not already open.

## Special Mark Jumps (normative)

| Command | Target |
|---|---|
| `''` / ``` `` ``` | Previous cursor position (before last jump) |
| `'.` / `` `. `` | Position of last change |
| `'^` / `` `^ `` | Position where insert mode was last exited |
| `'<` / `` `< `` | Start of last visual selection |
| `'>` / `` `> `` | End of last visual selection |
| `'[` / `` `[ `` | Start of last change/yank |
| `']` / `` `] `` | End of last change/yank |
| `'"` / `` `" `` | Last cursor position when file was last closed |

## Jumplist Interaction

Jumps with `'` and `` ` `` add to the jumplist. Use `<C-o>` / `<C-i>` to navigate back and forward through the jumplist.

### Without Jumplist (g prefix)

`g'a` and `` g`a `` jump to mark `a` without adding to the jumplist.

## Marks as Motions

Marks can serve as the motion in operator commands:

| Command | Effect |
|---|---|
| `d'a` | Delete from cursor line to line of mark `a` |
| `` d`a `` | Delete from cursor position to exact position of mark `a` |
| `y'a` | Yank from cursor line to line of mark `a` |

## Visual Mode

In visual mode, `'a` or `` `a `` extends the selection to the mark position.

## Related

- Mark persistence: [/docs/spec/editing/marks/mark-persistence.md](/docs/spec/editing/marks/mark-persistence.md)
- Jumplist: [/docs/spec/features/navigation/jumplist.md](/docs/spec/features/navigation/jumplist.md)
- Marks overview: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
