# Insert Digraphs

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Digraphs allow entering special characters by typing a two-character sequence.

## Entry method (normative)

| Key sequence | Action |
|---|---|
| `Ctrl-k {char1} {char2}` | Insert the character mapped to the digraph `{char1}{char2}` |

After pressing `Ctrl-k`, the status line MUST show `?` to indicate digraph-pending state. The next two characters are consumed as the digraph code.

## Digraph table (normative)

The editor MUST maintain a built-in digraph table compatible with Vim's `:digraphs` output. A subset of essential entries:

| Code | Character | Description |
|---|---|---|
| `e'` | `é` | e-acute |
| `a:` | `ä` | a-diaeresis |
| `o"` | `ö` | o-diaeresis |
| `u:` | `ü` | u-diaeresis |
| `ss` | `ß` | sharp s |
| `Eu` | `€` | Euro sign |
| `Pd` | `£` | Pound sign |
| `Ye` | `¥` | Yen sign |
| `a*` | `α` | Greek alpha |
| `b*` | `β` | Greek beta |
| `p*` | `π` | Greek pi |
| `->` | `→` | Right arrow |
| `<-` | `←` | Left arrow |
| `Co` | `©` | Copyright |
| `Rg` | `®` | Registered |

## Unknown digraph behavior

If the two-character code does not match any digraph entry, the editor MUST beep (or flash) and insert nothing. Pending state is cancelled.

## Listing digraphs

The command `:digraphs` MUST display the full digraph table in a scrollable view.

## Related

- Text manipulation digraphs: [/docs/spec/editing/text-manipulation/digraphs.md](/docs/spec/editing/text-manipulation/digraphs.md)
- Insert Unicode: [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)

