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

## Counts

All motions accept counts:

