# Lookaround

Back: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

Zero-width assertions that match based on surrounding text without consuming it.

## Overview

Lookaround atoms assert that certain text exists (or does not exist) ahead of or behind the current position, without including that text in the match.

## Lookahead

| Atom | Name | Meaning |
|---|---|---|
| `\(pattern\)\@=` | Positive lookahead | Succeeds if `pattern` matches ahead |
| `\(pattern\)\@!` | Negative lookahead | Succeeds if `pattern` does NOT match ahead |

## Lookbehind

| Atom | Name | Meaning |
|---|---|---|
| `\(pattern\)\@<=` | Positive lookbehind | Succeeds if `pattern` matches behind |
| `\(pattern\)\@<!` | Negative lookbehind | Succeeds if `pattern` does NOT match behind |

## Performance note

Lookbehind with variable-length patterns can be expensive. The regex engine must try all possible lengths. Use `\@123<=` to limit the maximum lookbehind width to 123 bytes.

## Examples

| Pattern | Matches |
|---|---|
| `foo\(bar\)\@=` | `foo` only when followed by `bar` |
| `foo\(bar\)\@!` | `foo` only when NOT followed by `bar` |
| `\(foo\)\@<=bar` | `bar` only when preceded by `foo` |
| `\(foo\)\@<!bar` | `bar` only when NOT preceded by `foo` |

## Match boundary interaction

Lookaround atoms interact with `\zs` and `\ze`:

| Pattern | Matches | Explanation |
|---|---|---|
| `\(TODO\)\@<=:.*` | `:...` after `TODO` | Lookbehind for `TODO`, match starts at `:` |
| `.*\ze\(;\)` | Text before `;` | Lookahead for `;`, match ends before it |

## Related

- Regex atoms: [/docs/spec/editing/regex/regex-atoms.md](/docs/spec/editing/regex/regex-atoms.md)
- Grouping: [/docs/spec/editing/regex/grouping-refs.md](/docs/spec/editing/regex/grouping-refs.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)
