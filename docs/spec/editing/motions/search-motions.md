# Search Motions

Back: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Search motions move the cursor to a pattern match and can be used with operators.

## Forward search (normative)

| Key | Action | Motion type |
|---|---|---|
| `/{pattern}<CR>` | Move to next match of `{pattern}` forward | Exclusive |
| `n` | Move to next match in search direction | Exclusive |
| `N` | Move to next match in opposite direction | Exclusive |

## Backward search (normative)

| Key | Action | Motion type |
|---|---|---|
| `?{pattern}<CR>` | Move to next match of `{pattern}` backward | Exclusive |

`n` and `N` reverse their direction when the last search was `?`.

## Word under cursor (normative)

| Key | Action | Detail |
|---|---|---|
| `*` | Search forward for the word under cursor | Adds `\b` word boundaries automatically |
| `#` | Search backward for the word under cursor | Adds `\b` word boundaries automatically |
| `g*` | Search forward for partial match (no word boundaries) | |
| `g#` | Search backward for partial match (no word boundaries) | |

## Operator integration (normative)

Search motions combine with operators:

| Example | Action |
|---|---|
| `d/pattern<CR>` | Delete from cursor to (exclusive of) the next match |
| `y?pattern<CR>` | Yank from cursor backward to the match |
| `c/pattern<CR>` | Change from cursor to the match, enter Insert |

## Search offsets (normative)

An offset can be appended after the closing delimiter:

| Offset | Meaning |
|---|---|
| `/pattern/+{n}` | `{n}` lines below the match |
| `/pattern/-{n}` | `{n}` lines above the match |
| `/pattern/e` | End of the match (cursor on last character of match) |
| `/pattern/e+{n}` | `{n}` characters after the end of the match |
| `/pattern/b+{n}` | `{n}` characters after the beginning of the match |

## Wrapping

Searches wrap by default (`wrapscan` option). When wrapping occurs, a message "search hit BOTTOM, continuing at TOP" (or TOP/BOTTOM for `?`) MUST be displayed.

## Highlighting

When `hlsearch` is enabled, all matches of the last search pattern MUST be highlighted with the `Search` highlight group. `:nohlsearch` clears highlights until the next search.

Incremental search (`incsearch` option, default true): as the user types the pattern, the first match is highlighted and the viewport scrolls to show it.

## Related

- Search options: [/docs/spec/editing/search/search-options.md](/docs/spec/editing/search/search-options.md)
- Search highlight: [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)
- Regex: [/docs/spec/editing/regex/regex.md](/docs/spec/editing/regex/regex.md)


