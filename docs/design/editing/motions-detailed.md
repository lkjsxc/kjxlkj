# Motion Commands

All motion commands for cursor navigation.

## Character Motions

| Motion | Description |
|--------|-------------|
| `h` | Left |
| `j` | Down |
| `k` | Up |
| `l` | Right |

## Word Motions

| Motion | Description |
|--------|-------------|
| `w` | Next word start |
| `W` | Next WORD start |
| `e` | Next word end |
| `E` | Next WORD end |
| `b` | Previous word start |
| `B` | Previous WORD start |
| `ge` | Previous word end |
| `gE` | Previous WORD end |

## Line Motions

| Motion | Description |
|--------|-------------|
| `0` | Line start |
| `^` | First non-blank |
| `$` | Line end |
| `g_` | Last non-blank |
| `+` | First non-blank next line |
| `-` | First non-blank prev line |

## Screen Line Motions

| Motion | Description |
|--------|-------------|
| `gj` | Down (screen line) |
| `gk` | Up (screen line) |
| `g0` | Screen line start |
| `g$` | Screen line end |
| `gm` | Screen line middle |

## File Motions

| Motion | Description |
|--------|-------------|
| `gg` | File start |
| `G` | File end |
| `{count}G` | Go to line |
| `{count}%` | Go to percentage |

## Paragraph Motions

| Motion | Description |
|--------|-------------|
| `{` | Previous paragraph |
| `}` | Next paragraph |

## Sentence Motions

| Motion | Description |
|--------|-------------|
| `(` | Previous sentence |
| `)` | Next sentence |

## Search Motions

| Motion | Description |
|--------|-------------|
| `f{char}` | Find char forward |
| `F{char}` | Find char backward |
| `t{char}` | Till char forward |
| `T{char}` | Till char backward |
| `;` | Repeat f/t |
| `,` | Repeat f/t reverse |

## Word Search

| Motion | Description |
|--------|-------------|
| `*` | Search word forward |
| `#` | Search word backward |
| `g*` | Partial word forward |
| `g#` | Partial word backward |

## Pattern Search

| Motion | Description |
|--------|-------------|
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Next match |
| `N` | Previous match |

## Mark Motions

| Motion | Description |
|--------|-------------|
| `` `{mark} `` | Go to mark (exact) |
| `'{mark}` | Go to mark (line) |
| `` `` `` | Previous position |
| `''` | Previous position (line) |

## Bracket Motions

| Motion | Description |
|--------|-------------|
| `%` | Matching bracket |
| `[(` | Previous unmatched `(` |
| `])` | Next unmatched `)` |
| `[{` | Previous unmatched `{` |
| `]}` | Next unmatched `}` |

## Method Motions

| Motion | Description |
|--------|-------------|
| `]m` | Next method start |
| `[m` | Previous method start |
| `]M` | Next method end |
| `[M` | Previous method end |

## Motion Categories

Motions are classified by the type of text unit they operate on, which determines
their characterwise vs linewise behavior when used with operators:

| Category | Motions | Operator Behavior |
|-----------|---------|-------------------|
| Character | `h`, `l`, `f`, `F`, `t`, `T` | Characterwise |
| Word | `w`, `W`, `e`, `E`, `b`, `B`, `ge`, `gE` | Characterwise |
| Line | `j`, `k`, `+`, `-`, `G`, `gg` | Linewise |
| Sentence | `(`, `)` | Characterwise |
| Paragraph | `{`, `}` | Characterwise (exclusive) |
| Search | `/`, `?`, `n`, `N`, `*`, `#` | Characterwise |
| Mark | `` ` ``(characterwise), `'`(linewise) | Depends on variant |

Exclusive motions do not include the final character in the operated range.
Inclusive motions include it. Example: `w` is exclusive, `e` is inclusive.

## Counts

All motions accept a numeric prefix count that repeats the motion:

| Input | Effect |
|-------|--------|
| `3w` | Move forward 3 word starts |
| `5j` | Move down 5 lines |
| `2f.` | Find the 2nd `.` forward on the line |
| `10l` | Move 10 characters right |
| `3}` | Move forward 3 paragraphs |
| `4B` | Move back 4 WORDs |

Counts are parsed as decimal digits before the motion key. Leading zeros are not
special (`03w` = `3w`, since `0` alone is line-start motion only without a
preceding digit). When a count exceeds available text, the cursor stops at the
boundary (start/end of file) and the motion fails (beep, no operator applied).

## Interaction with Operators

Motions define the range an operator acts on. Grammar: `{operator}[count]{motion}`.
Counts on operator and motion multiply: `2d3w` deletes 6 words.

| Command | Operator | Motion | Effect |
|---------|----------|--------|--------|
| `d3w` | delete | 3 words | Delete next 3 words |
| `c2e` | change | 2 word-ends | Change through 2 word ends, enter insert |
| `y5j` | yank | 5 lines down | Yank current line plus 5 below (linewise) |
| `gU2w` | uppercase | 2 words | Uppercase next 2 words |
| `>3j` | indent | 3 lines down | Indent current + 3 lines below |
| `gqap` | format | a paragraph | Reflow the paragraph to `textwidth` |
| `d/foo` | delete | search forward | Delete up to (exclusive) next "foo" match |
| `` d`a `` | delete | to mark a | Delete from cursor to mark a (characterwise) |

See also: [/docs/design/editing/operators-detailed.md](/docs/design/editing/operators-detailed.md),
[/docs/design/editing/text-objects-detailed.md](/docs/design/editing/text-objects-detailed.md)
