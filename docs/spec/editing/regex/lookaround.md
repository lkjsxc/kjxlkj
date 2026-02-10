# Lookaround

Back: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

Zero-width assertions that match by context without consuming text.

## Lookahead

| Atom | Name | Meaning |
|---|---|---|
| `\(pattern\)\@=` | Positive lookahead | succeeds if `pattern` matches ahead |
| `\(pattern\)\@!` | Negative lookahead | succeeds if `pattern` does not match ahead |

## Lookbehind

| Atom | Name | Meaning |
|---|---|---|
| `\(pattern\)\@<=` | Positive lookbehind | succeeds if `pattern` matches behind |
| `\(pattern\)\@<!` | Negative lookbehind | succeeds if `pattern` does not match behind |

## Performance Note

Variable-length lookbehind is expensive. Prefer bounded patterns when possible.

## Implementation Limitation

The underlying Rust `regex` engine does not support variable-length lookbehind.
Patterns like `\(foo\|foobar\)\@<=bar` may fail to compile.

Preferred rewrite uses `\zs` match-start control:

| Original | Rewrite |
|---|---|
| `\(foo\|foobar\)bar` | `\(foo\|foobar\)\zsbar` |

## Examples

| Pattern | Matches |
|---|---|
| `foo\(bar\)\@=` | `foo` only when followed by `bar` |
| `foo\(bar\)\@!` | `foo` only when not followed by `bar` |
| `\(foo\)\@<=bar` | `bar` only when preceded by `foo` |
| `\(foo\)\@<!bar` | `bar` only when not preceded by `foo` |

## Boundary Interaction

| Pattern | Behavior |
|---|---|
| `\(TODO\)\@<=:.*` | matches text after `TODO` starting at `:` |
| `.*\ze\(;\)` | matches text before `;` |

## Related

- Regex atoms: [/docs/spec/editing/regex/regex-atoms.md](/docs/spec/editing/regex/regex-atoms.md)
- Grouping: [/docs/spec/editing/regex/grouping-refs.md](/docs/spec/editing/regex/grouping-refs.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)
