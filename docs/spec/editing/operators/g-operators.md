# g-Prefixed Operators

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

Operators and commands accessed via the `g` prefix key.

## Case operators (normative)

| Key sequence | Action | Type |
|---|---|---|
| `g~{motion}` | Toggle case over motion range | Operator |
| `gu{motion}` | Lowercase over motion range | Operator |
| `gU{motion}` | Uppercase over motion range | Operator |
| `g~~` | Toggle case of current line | Linewise variant |
| `guu` | Lowercase current line | Linewise variant |
| `gUU` | Uppercase current line | Linewise variant |

These follow the standard operator grammar: count + operator + motion/text-object.

## Format operator (normative)

| Key sequence | Action |
|---|---|
| `gq{motion}` | Format text over motion range (cursor moves to last formatted line) |
| `gw{motion}` | Format text over motion range (cursor position preserved) |
| `gqq` | Format current line |

Formatting wraps text at `textwidth` (default 80). It joins short lines and respects `formatoptions` settings.

## Join operator (normative)

| Key sequence | Action |
|---|---|
| `gJ` | Join current line with next WITHOUT inserting a space |
| `{count}gJ` | Join current line with next `{count}` lines without spaces |

Compare with `J` which inserts a space at the join point.

## Put with indent (normative)

| Key sequence | Action |
|---|---|
| `]p` | Put after cursor with auto-indent adjustment |
| `[p` | Put before cursor with auto-indent adjustment |

## Related

- Normal mode g-prefix commands: [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- Operators overview: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)


