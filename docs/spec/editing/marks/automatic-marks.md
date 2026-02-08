# Automatic Marks

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Marks set automatically by the editor during editing operations.

## Change marks

| Mark | Set when | Position |
|---|---|---|
| `.` | Any text change | Position of the last change |
| `[` | Text change or yank | Start of the affected region |
| `]` | Text change or yank | End of the affected region |

## Insert marks

| Mark | Set when | Position |
|---|---|---|
| `^` | Leaving Insert mode | Cursor position when `Esc` was pressed |

## Visual marks

| Mark | Set when | Position |
|---|---|---|
| `<` | Visual selection | Start of the last visual selection |
| `>` | Visual selection | End of the last visual selection |

## Jump marks

| Mark | Set when | Position |
|---|---|---|
| `` ` `` | Jump motion | Cursor position before the jump |

This mark is updated by every jump motion (see [/docs/spec/editing/motions/jumps/jump-motions.md](/docs/spec/editing/motions/jumps/jump-motions.md)).

## Buffer exit marks

| Mark | Set when | Position |
|---|---|---|
| `"` | Leaving a buffer | Cursor position when the buffer was last left |

When returning to a buffer, jumping to `'"` restores the previous cursor position.

## Sentence/paragraph marks

These are not stored as marks but are implicit positions computed on demand by the `(`, `)`, `{`, `}` motions.

## Updating behavior

Automatic marks are updated every time their triggering event occurs. They cannot be set manually by the user with `:mark` or `m{letter}`. However, `` ` `` and `'` can be accessed like other marks using `` ` ` `` and `''`.

## Use cases

| Pattern | Effect |
|---|---|
| `` `. `` | Jump to position of last change |
| `'[` / `']` | Select the range of the last change (useful for `gv`-like re-selection after paste) |
| `g;` / `g,` | Navigate the change list (similar to `.` but with history) |

## Related

- Mark types: [/docs/spec/editing/marks/mark-types.md](/docs/spec/editing/marks/mark-types.md)
- Special marks: [/docs/spec/editing/marks/special-marks.md](/docs/spec/editing/marks/special-marks.md)
- Change list: [/docs/spec/editing/marks/changelist.md](/docs/spec/editing/marks/changelist.md)
