# Search Patterns

Advanced search techniques.

## Overview

Comprehensive search pattern features for finding
text efficiently with forward/backward search,
regex, offsets, and history.

## Basic Search

### Forward Search

`/pattern<CR>` searches forward from cursor.
The cursor moves to the first match.

### Backward Search

`?pattern<CR>` searches backward from cursor.

### Next/Previous

`n` repeats the last search in the same direction.
`N` repeats the last search in the opposite direction.

## Search Options

### Case Sensitivity

`\c` anywhere in the pattern forces case-insensitive.
`\C` anywhere in the pattern forces case-sensitive.

### Configuration

| Option | Default | Effect |
|--------|---------|--------|
| `ignorecase` | `false` | Case-insensitive search |
| `smartcase` | `true` | Case-sensitive if uppercase used |

## Word Search

### Current Word

`*` searches forward for word under cursor.
`#` searches backward for word under cursor.

### Word Boundaries

`*` and `#` add word boundary markers (`\<` and `\>`)
automatically. `g*` and `g#` search without boundaries,
matching partial words.

## Incremental Search

### Enable

`incsearch = true` highlights matches as you type.
The cursor jumps to the first match during input and
returns to the original position on `<Esc>`.

## Search Highlighting

### Toggle

`:nohlsearch` (`:noh`) clears current highlights.
`hlsearch = true` enables persistent highlights.

### Configuration

| Option | Default | Effect |
|--------|---------|--------|
| `hlsearch` | `true` | Highlight all matches |
| `incsearch` | `true` | Highlight during typing |

## Search History

### Navigate History

In search mode, `<Up>` and `<Down>` navigate through
previous search patterns. History is filtered by the
prefix already typed.

### Clear History

No built-in command to clear history. History is stored
in the session file.

## Offset

### After Match

`/pattern/e` places cursor at end of match.
`/pattern/e+{n}` places cursor {n} chars after end.
`/pattern/e-{n}` places cursor {n} chars before end.

### Before Match

`/pattern/b+{n}` places cursor {n} chars after start.
`/pattern/b-{n}` places cursor {n} chars before start.
`/pattern/s+{n}` is a synonym for `/b+{n}`.

### Line Offset

`/pattern/+{n}` places cursor {n} lines below match.
`/pattern/-{n}` places cursor {n} lines above match.

## Multi-Line Search

### Across Lines

`\_s` matches any whitespace including newlines.
`\n` matches a newline character specifically.

### Any Character Including Newline

`\_.` matches any character including newline.
This enables multi-line pattern matching.

## Search Flags

### In Command

`/pattern/flags` where flags modify behavior.

### Common Flags

| Flag | Meaning |
|------|---------|
| `e` | Move to end of match |
| `n` | Do not move cursor (report count) |
| `s` | Set previous context mark |

## Very Magic Search

### Simplified Regex

`/\v pattern` makes most characters special (regex-like).
Less escaping needed: `/\v(foo|bar)\d+` instead of
`/\(foo\|bar\)\d\+`.

### Magic Levels

| Prefix | Level | Description |
|--------|-------|-------------|
| `\v` | Very magic | Most chars are special |
| `\m` | Magic | Default Vim behavior |
| `\M` | No magic | Few chars are special |
| `\V` | Very nomagic | Only `\` is special |

## Literal Search

### Very No Magic

`/\V literal text` searches for exact text.
Only `\` has special meaning, everything else literal.

## Substitution Integration

### Last Search

`:s//replacement/` uses the last search pattern.
`:%s//new/g` replaces all matches of the last search.

### Confirm

`:s/pattern/replace/c` prompts for each match:
`y` (yes), `n` (no), `a` (all remaining), `q` (quit),
`l` (last: substitute this one and quit).

## Visual Selection Search

### Search Selection

In visual mode, pressing `*` or `#` can be mapped to
search for the selected text. This is not built-in
but commonly configured.

### Escape Special Characters

When searching for selected text, special regex
characters must be escaped: `\`, `/`, `[`, `]`, `*`.
