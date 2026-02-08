# Substitute Command

Find and replace text with :s.

## Overview

The substitute command replaces
text matching a pattern.

## Basic Syntax

```
:[range]s/pattern/replacement/[flags]
```

All parts after the pattern are optional. Without a range, operates on the current line.

## Simple Examples

### Basic Replace

| Command | Effect |
|---------|--------|
| `:s/foo/bar/` | Replace first `foo` with `bar` on current line |
| `:%s/foo/bar/g` | Replace all `foo` with `bar` in entire file |
| `:5,10s/old/new/g` | Replace all `old` with `new` on lines 5-10 |
| `:%s/\t/  /g` | Replace all tabs with two spaces |

### Current Line

With no range, `:s` operates on the current line only and replaces the first occurrence. Add `g` to replace all occurrences on that line.

## Range

### Common Ranges

| Range | Scope |
|-------|-------|
| (none) | Current line only |
| `%` | Entire file (alias for `1,$`) |
| `.` | Current line |
| `$` | Last line |
| `1,$` | Entire file |
| `'<,'>` | Visual selection |
| `'a,'b` | Between marks `a` and `b` |
| `.,.+5` | Current line plus next 5 lines |
| `-3,+3` | 3 lines above through 3 lines below cursor |

### Patterns

Patterns use magic mode regex by default.

| Atom | Matches |
|------|---------|
| `.` | Any single character |
| `*` | Zero or more of preceding |
| `\+` | One or more of preceding |
| `\?` | Zero or one of preceding |
| `^` | Start of line |
| `$` | End of line |
| `\n` | Newline (in search); inserts newline (in replacement) |
| `\t` | Tab character |
| `\r` | Carriage return |
| `\s` | Whitespace (space or tab) |
| `\d` | Digit `[0-9]` |
| `\w` | Word character `[0-9A-Za-z_]` |
| `\(` `\)` | Capture group boundaries |

## Delimiter

### Default Slash

The default delimiter is `/`. The full form is `:s/pattern/replacement/flags`.

### Alternatives

For paths with slashes, use any single-byte character as delimiter:

| Example | Delimiter |
|---------|-----------|
| `:s#/usr/bin#/usr/local/bin#g` | `#` |
| `:s+path/old+path/new+g` | `+` |
| `:s\|foo\|bar\|g` | `\|` |

## Flags

### Common Flags

| Flag | Meaning              |
|------|---------------------|
| `g`  | Global (all on line)|
| `c`  | Confirm each        |
| `i`  | Case insensitive    |
| `I`  | Case sensitive      |
| `n`  | Count only          |
| `e`  | No error if none    |
| `&`  | Reuse previous flags|

### Combinations

Flags compose freely. `:%s/foo/bar/gci` replaces all occurrences, case-insensitively, with confirmation. The `&` flag appends flags from the previous `:s` command to the current invocation.

## Confirm Mode

### Interactive

When the `c` flag is set, each match is highlighted and the editor prompts for an action before proceeding.

Prompts:

### Confirm Options

| Key  | Action              |
|------|---------------------|
| `y`  | Replace this        |
| `n`  | Skip this           |
| `a`  | Replace all rest    |
| `q`  | Quit substituting   |
| `l`  | Replace and quit    |
| `^E` | Scroll up           |
| `^Y` | Scroll down         |

## Special Replacement

### Entire Match

Use `&` or `\0` in the replacement string to insert the entire matched text. Example: `:s/word/(&)/g` wraps each `word` in parentheses.

### Captured Groups

Use `\(` and `\)` in the pattern to define capture groups. Reference them in the replacement with `\1` through `\9`.

### Numbered Groups

| Token | Inserts |
|-------|---------|
| `&` or `\0` | Entire match |
| `\1` | First captured group |
| `\2` | Second captured group |
| `\3` .. `\9` | Third through ninth group |

## Case Conversion

### Flags in Replacement

| Sequence | Effect              |
|----------|---------------------|
| `\u`     | Next char uppercase |
| `\U`     | Rest uppercase      |
| `\l`     | Next char lowercase |
| `\L`     | Rest lowercase      |
| `\e`     | End case change     |
| `\E`     | End case change     |

### Examples

| Command | Result |
|---------|--------|
| `:s/\(foo\)/\U\1/g` | `foo` becomes `FOO` |
| `:s/\(foo\)/\u\1/g` | `foo` becomes `Foo` |
| `:s/\(FOO\)/\L\1/g` | `FOO` becomes `foo` |

## Empty Pattern

### Reuse Last Search

When the pattern is omitted (`:s//replacement/`), the last used search pattern is reused. This applies whether the last search was from `/`, `?`, or a prior `:s`.

### After Search

Search with `/pattern` first, then run `:%s//replacement/g` to replace all occurrences of that pattern across the file.

## Empty Replacement

### Delete Matches

When the replacement is empty (`:s/pattern//g`), all matches on the line are deleted. This removes the matched text entirely.

## Repeat Substitute

### Repeat Last

`:s` with no arguments repeats the last substitute on the current line without flags. Use `:s//~/&` to repeat with the same replacement and flags.

### Different Range

`:%s` repeats the last substitute across the entire file. Any new range applies: `:5,10s` repeats the last substitute on lines 5 through 10.

## Line Addressing

Lines are addressed by number, mark, or relative offset. The substitute command processes each addressed line sequentially from first to last. If a replacement changes line count, subsequent line addresses are not adjusted.
