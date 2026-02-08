# Search Count Display

Show match position and total count during search.

## Count Format (normative)

After a search, display `[current/total]` on the command line. Example: `[3/42]` means the cursor is on the 3rd match of 42 total.

## Display Locations

| Location | When |
|---|---|
| Command line | After pressing `<Enter>` on a search, or during `n`/`N` |
| Statusline | Persistent display via statusline component |
| Virtual text | Optional inline display next to current match |

## Configuration

| Option | Default | Description |
|---|---|---|
| `show_search_count` | `true` | Show `[N/M]` after search |
| `search_count_format` | `"[%d/%d]"` | Format string for count display |
| `search_count_max` | `1000` | Max matches to count (0 = unlimited) |

## Count Update Events

The count recalculates on:

- New search pattern entered
- `n` / `N` navigation
- Buffer modification (content changed)
- Search register (`"/`) changed programmatically

## Performance

For files with many matches, counting is capped at `search_count_max`. When the limit is exceeded, the display shows `[3/>1000]` to indicate the total is approximate.

Counting runs asynchronously and does not block editing. While counting, the display shows `[3/?]`.

## Navigation Commands

| Key | Action | Count updates |
|---|---|---|
| `n` | Next match | Yes |
| `N` | Previous match | Yes |
| `[count]n` | Jump count matches forward | Yes |
| `[count]N` | Jump count matches backward | Yes |

## searchcount() Function

Access count data programmatically for statusline integration:

| Field | Type | Description |
|---|---|---|
| `current` | `Number` | 1-based index of current match |
| `total` | `Number` | Total match count |
| `incomplete` | `Number` | 0 = complete, 1 = recomputing, 2 = exceeded max |
| `maxcount` | `Number` | Configured max count |

## Wrap-Around Notification

When search wraps past the end/beginning of the buffer, display:

- `search hit BOTTOM, continuing at TOP` (with `wrapscan` enabled)
- `Pattern not found` (with `wrapscan` disabled, at boundary)

## Incremental Search

During incremental search (`incsearch = true`), the count updates live as the pattern is typed, showing matches for the partial pattern.

## Related

- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
- Search highlight: [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)
