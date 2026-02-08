# Search Patterns

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

Pattern syntax and behavior for forward and backward search.

## Overview

Search patterns use the regex engine described in [/docs/spec/editing/regex/regex.md](/docs/spec/editing/regex/regex.md). The search command parses the pattern, compiles it, and scans the buffer for matches.

## Search commands

| Key | Action |
|---|---|
| `/` | Enter forward search mode |
| `?` | Enter backward search mode |
| `n` | Repeat last search in same direction |
| `N` | Repeat last search in opposite direction |

## Pattern entry

After pressing `/` or `?`, the command line opens with the search prompt. The user types the pattern and presses `<CR>` to execute. `<Esc>` cancels the search.

## Incremental search

| Setting | Type | Default | Description |
|---|---|---|---|
| `incsearch` | boolean | `true` | Show matches incrementally while typing |

When `incsearch = true`, the first match is highlighted and the cursor is temporarily moved to it as the user types. The viewport scrolls to show the match.

## Case sensitivity

| Setting | Effect |
|---|---|
| `ignorecase = true` | All searches are case-insensitive |
| `smartcase = true` | Case-insensitive unless the pattern contains uppercase letters |
| `\c` in pattern | Force case-insensitive for this pattern |
| `\C` in pattern | Force case-sensitive for this pattern |

`smartcase` only applies when `ignorecase` is also `true`.

## Wrap behavior

| Setting | Default | Description |
|---|---|---|
| `wrapscan` | `true` | Continue search past end of buffer, wrapping to beginning |

When wrapping occurs, a message indicates the direction of the wrap.

## Search register

The last search pattern is stored in the `/` register. It persists across searches and can be used by `n`, `N`, and `hlsearch`.

## Offset

Search patterns support an offset that positions the cursor relative to the match:

| Offset | Meaning |
|---|---|
| `/pattern/+n` | Position cursor `n` lines below the match |
| `/pattern/-n` | Position cursor `n` lines above the match |
| `/pattern/e` | Position cursor at the end of the match |
| `/pattern/e+n` | Position cursor `n` characters after match end |
| `/pattern/b+n` | Position cursor `n` characters after match beginning |

## Search count

After a search, the statusline shows the match count and current index: `[3/15]` means the cursor is on match 3 of 15 total matches.

## Related

- Regex: [/docs/spec/editing/regex/regex.md](/docs/spec/editing/regex/regex.md)
- Search highlight: [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)
- Star search: [/docs/spec/editing/search/star-search.md](/docs/spec/editing/search/star-search.md)
