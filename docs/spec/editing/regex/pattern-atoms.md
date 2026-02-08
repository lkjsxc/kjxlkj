# Pattern Atoms

Low-level building blocks that compose regex patterns.

## Character Classes

| Class | Matches |
|---|---|
| `.` | Any character except newline |
| `\w` | Word character: `[0-9A-Za-z_]` |
| `\W` | Non-word character |
| `\d` | Digit: `[0-9]` |
| `\D` | Non-digit |
| `\s` | Whitespace: `[ \t]` |
| `\S` | Non-whitespace |
| `\a` | Alphabetic: `[A-Za-z]` |
| `\l` | Lowercase: `[a-z]` |
| `\u` | Uppercase: `[A-Z]` |
| `\h` | Head-of-word: `[A-Za-z_]` |
| `\x` | Hex digit: `[0-9A-Fa-f]` |
| `\o` | Octal digit: `[0-7]` |

## Position Atoms (Zero-Width)

| Atom | Matches |
|---|---|
| `^` | Start of line |
| `$` | End of line |
| `\<` | Start of word |
| `\>` | End of word |
| `\zs` | Set match start (the actual match begins here) |
| `\ze` | Set match end (the actual match ends here) |
| `\_^` | Start of line, anywhere in pattern |
| `\_$` | End of line, anywhere in pattern |

## Grouping and Capturing

| Syntax | Meaning |
|---|---|
| `\(...\)` | Capturing group (numbered 1-9) |
| `\%(...\)` | Non-capturing group |
| `\1` .. `\9` | Back-reference to captured group |
| `\v(...)` | Capturing group in very-magic mode |

Groups are numbered left-to-right by opening parenthesis position.

## Alternation

`\|` separates alternatives. Matches the first successful branch.

In very-magic mode, use `|` without backslash.

Alternation has the lowest precedence of all regex operators.

## Quantifiers

| Quantifier | Meaning | Greedy |
|---|---|---|
| `*` | Zero or more | Yes |
| `\+` | One or more | Yes |
| `\?` or `\=` | Zero or one | Yes |
| `\{n,m}` | Between n and m | Yes |
| `\{n}` | Exactly n | N/A |
| `\{n,}` | At least n | Yes |
| `\{,m}` | At most m | Yes |
| `\{-n,m}` | Between n and m | No (lazy) |
| `\{-}` | Zero or more | No (lazy) |
| `\{-n,}` | At least n | No (lazy) |

## Lookaround Assertions

| Syntax | Type | Description |
|---|---|---|
| `\(...\)\@=` | Positive lookahead | Matches if group matches ahead |
| `\(...\)\@!` | Negative lookahead | Matches if group does NOT match ahead |
| `\(...\)\@<=` | Positive lookbehind | Matches if group matches behind |
| `\(...\)\@<!` | Negative lookbehind | Matches if group does NOT match behind |
| `\(...\)\@>` | Atomic group | Match and do not backtrack into group |

## Unicode Support

| Atom | Matches |
|---|---|
| `\%u{XXXX}` | Specific Unicode codepoint (hex) |
| `[[:alpha:]]` | POSIX alphabetic (Unicode-aware) |
| `[[:digit:]]` | POSIX digit |
| `[[:space:]]` | POSIX whitespace |
| `[[:upper:]]` | POSIX uppercase |
| `[[:lower:]]` | POSIX lowercase |

The regex engine operates on Unicode codepoints. Character classes (`\w`, `\d`, etc.) match ASCII ranges only by default.

## Collection (Bracket Expressions)

| Syntax | Meaning |
|---|---|
| `[abc]` | Any of a, b, c |
| `[^abc]` | None of a, b, c |
| `[a-z]` | Range a through z |
| `[[:class:]]` | POSIX class |
| `\_[...]` | Collection including newline |
| `\%[abc]` | Optional sequence: matches `a`, `ab`, or `abc` |

## Special Atoms

| Atom | Matches |
|---|---|
| `\n` | Newline in pattern (end-of-line) |
| `\r` | Carriage return |
| `\t` | Tab |
| `\e` | Escape (0x1B) |
| `\b` | Backspace (0x08) in collection, word boundary outside |
| `\_x` | Character class x plus newline (e.g., `\_s`, `\_w`) |
| `\.` | Literal dot (escaped) |

## Related

- Regex overview: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)
- Very magic: [/docs/spec/editing/regex/very-magic.md](/docs/spec/editing/regex/very-magic.md)
- Substitute specials: [/docs/spec/commands/substitute/substitute-specials.md](/docs/spec/commands/substitute/substitute-specials.md)
