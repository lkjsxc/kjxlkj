# Search Commands

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

Commands for searching within buffers.

## Overview

The editor provides forward search (`/`), backward search (`?`), word search (`*`, `#`), and ex-command search (`:substitute`, `:global`).

## Forward Search

`/pattern<CR>` — search forward from cursor for `pattern`. The cursor moves to the first match.

| Key | Description |
|---|---|
| `/` | Begin forward search |
| `n` | Repeat search in same direction |
| `N` | Repeat search in opposite direction |

## Backward Search

`?pattern<CR>` — search backward from cursor. `n` continues backward; `N` searches forward.

## Incremental Search

When `incsearch` is enabled, matches are highlighted as the pattern is typed.

| Setting | Default | Description |
|---|---|---|
| `incsearch` | `true` | Show matches incrementally |
| `hlsearch` | `true` | Highlight all matches |

## Case Sensitivity

| Setting / Flag | Effect |
|---|---|
| `ignorecase` | Ignore case in patterns |
| `smartcase` | Override `ignorecase` if pattern has uppercase |
| `\c` in pattern | Force case-insensitive |
| `\C` in pattern | Force case-sensitive |

## Wrapping

| Setting | Default | Description |
|---|---|---|
| `wrapscan` | `true` | Search wraps around buffer boundaries |

When wrapping occurs, a message is shown: "search hit BOTTOM, continuing at TOP".

## Offset

A search offset moves the cursor relative to the match:

| Offset | Meaning |
|---|---|
| `/pattern/+N` | N lines below match |
| `/pattern/-N` | N lines above match |
| `/pattern/e` | End of match |
| `/pattern/e+N` | N characters after end |
| `/pattern/b+N` | N characters after beginning |

## Count

`3/pattern<CR>` — jump to the 3rd match.

## Clear Highlight

`:nohlsearch` (`:noh`) clears search highlighting until the next search.

## Related

- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
- Search history: [/docs/spec/editing/search/search-history.md](/docs/spec/editing/search/search-history.md)
- Star search: [/docs/spec/editing/search/star-search.md](/docs/spec/editing/search/star-search.md)
- Live grep: [/docs/spec/editing/search/live-grep.md](/docs/spec/editing/search/live-grep.md)
