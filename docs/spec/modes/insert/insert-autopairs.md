# Insert Auto-Pairs

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Automatic insertion of matching bracket/quote pairs.

## Overview

Auto-pairs automatically inserts the closing character when an opening character is typed. The cursor is placed between the pair.

## Paired Characters

| Open | Close |
|---|---|
| `(` | `)` |
| `[` | `]` |
| `{` | `}` |
| `"` | `"` |
| `'` | `'` |
| `` ` `` | `` ` `` |

## Behavior

| Scenario | Action |
|---|---|
| Type `(` | Insert `()`, cursor between |
| Type `)` when next char is `)` | Skip over the `)` (no duplicate) |
| Type `<BS>` between empty pair | Delete both characters |
| Type `<CR>` between `{}` | Open braces with indentation |

## Smart Quotes

Quote auto-pairing is context-aware:

- Don't pair when inside a word (e.g., `can't`).
- Don't pair when the character before is a backslash.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `autopairs.enabled` | `true` | Enable auto-pairs |
| `autopairs.pairs` | default set | Custom pair definitions |

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
