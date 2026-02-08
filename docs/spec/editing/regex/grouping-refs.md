# Grouping and References

Capturing groups, back-references, and non-capturing groups.

## Capturing Groups

| Syntax | Magic Mode | Very Magic |
|---|---|---|
| `\(pattern\)` | Magic | `\v(pattern)` |

Groups are numbered 1-9 from left to right by opening parenthesis position. Up to 9 numbered groups are supported.

## Non-Capturing Groups

| Syntax | Magic Mode | Very Magic |
|---|---|---|
| `\%(pattern\)` | Magic | `\v%(pattern)` |

Non-capturing groups are used for grouping alternation or applying quantifiers without consuming a capture slot.

## Back-References

| Syntax | Meaning |
|---|---|
| `\1` .. `\9` | Match the same text as captured by group N |

Back-references match the exact text captured, not the pattern. If group 1 captured `foo`, then `\1` matches literal `foo`.

## Back-References in Replacement

In `:s` replacement strings:

| Syntax | Meaning |
|---|---|
| `\1` .. `\9` | Insert text captured by group N |
| `\0` or `&` | Insert entire match |

## Atomic Groups

| Syntax | Magic Mode | Very Magic |
|---|---|---|
| `\(pattern\)\@>` | Magic | `\v(pattern)@>` |

Once an atomic group matches, the regex engine does not backtrack into it. Useful for performance optimization.

## Lookaround (Zero-Width Assertions)

| Syntax | Type | Description |
|---|---|---|
| `\(pattern\)\@=` | Positive lookahead | Assert pattern matches ahead |
| `\(pattern\)\@!` | Negative lookahead | Assert pattern does NOT match ahead |
| `\(pattern\)\@<=` | Positive lookbehind | Assert pattern matches behind |
| `\(pattern\)\@<!` | Negative lookbehind | Assert pattern does NOT match behind |

Lookaround assertions are zero-width: they assert a condition but consume no characters.

## Match Position Modifiers

| Atom | Effect |
|---|---|
| `\zs` | Set the start of the match (text before is checked but not included) |
| `\ze` | Set the end of the match (text after is checked but not included) |

## Related

- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)
