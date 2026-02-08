# Very Magic Mode

The `\v` pattern modifier for Perl-like regular expressions.

## Overview

Prefixing a pattern with `\v` enables "very magic" mode, where most special characters do not need backslash escaping. This makes patterns more readable, especially for users familiar with Perl/PCRE regex.

## Activation

Place `\v` at the beginning of the pattern: `/\v(foo|bar)+`

## Comparison Table (normative)

| Construct | Normal (nomagic) | Very Magic (`\v`) |
|---|---|---|
| Grouping | `\(...\)` | `(...)` |
| Alternation | `\|` | `|` |
| One or more | `\+` | `+` |
| Zero or one | `\?` or `\=` | `?` |
| Quantifier | `\{n,m\}` | `{n,m}` |
| Word start | `\<` | `<` |
| Word end | `\>` | `>` |
| Negative lookahead | `\(...\)\@!` | `(...)@!` |
| Positive lookahead | `\(...\)\@=` | `(...)@=` |
| Negative lookbehind | `\(...\)\@<!` | `(...)@<!` |
| Positive lookbehind | `\(...\)\@<=` | `(...)@<=` |

## Characters Still Requiring Escape in Very Magic

| Character | Reason |
|---|---|
| `\` | Escape character itself |
| `/` | Search delimiter (unless using alternate delimiter) |

## Four Magic Modes

| Prefix | Name | Behavior |
|---|---|---|
| `\v` | Very magic | Most chars are special |
| `\m` | Magic | Default Vim behavior |
| `\M` | Nomagic | Most chars are literal |
| `\V` | Very nomagic | Only `\` is special |

## When to Use Very Magic

Recommended for complex patterns with multiple groups, alternations, or quantifiers. Not necessary for simple literal searches.

## Related

- Regex overview: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)
- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)
