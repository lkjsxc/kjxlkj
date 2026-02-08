# Auto-Pairs

Automatic bracket and quote pairing in insert mode.

## Supported Pairs

| Open | Close | Context |
|---|---|---|
| `(` | `)` | Always |
| `[` | `]` | Always |
| `{` | `}` | Always |
| `"` | `"` | Smart (not after `\` or word char) |
| `'` | `'` | Smart (not in words like `don't`) |
| `` ` `` | `` ` `` | Smart |
| `<` | `>` | Only in HTML/XML/JSX filetypes |

## Behavior Rules (normative)

1. **On typing open char**: Insert both open and close, place cursor between them
2. **On typing close char when next char is matching close**: Skip over the existing close character instead of inserting a duplicate
3. **On Backspace between empty pair**: Delete both characters
4. **On Enter between pair**: Insert newline, indent, place closing char on next line (expand pair)

## Smart Quote Logic

Quotes (`"`, `'`, `` ` ``) only auto-pair when:

- Previous character is whitespace, punctuation, or start of line
- Next character is whitespace, punctuation, or end of line
- Not inside a string where the quote would be an escape

## Configuration

| Setting | Default | Description |
|---|---|---|
| `editor.auto_pairs` | `true` | Enable/disable globally |
| `editor.auto_pairs.pairs` | (built-in set) | Custom pair definitions |

## Per-Filetype Overrides

Additional pairs can be enabled per filetype (e.g., `<`/`>` for HTML). Pairs can also be disabled per filetype.

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
