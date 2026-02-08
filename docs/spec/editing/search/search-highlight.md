# Search Highlighting

Visual feedback for search matches.

## Commands (normative)

| Command | Action |
|---|---|
| `:set hlsearch` | Enable persistent search highlighting |
| `:set nohlsearch` | Disable highlighting |
| `:nohlsearch` (`:noh`) | Clear current highlights (until next search) |

## Behavior

How search highlighting works during and after searches.

### During Incremental Search

When `incsearch = true`, the current match is highlighted as you type the pattern. `<C-g>` and `<C-t>` cycle forward/backward through matches during typing.

### After Search

When `hlsearch = true`, all matches remain highlighted after pressing `<Enter>`. Highlights persist until `:noh` is executed or a new search replaces the pattern.

## Keybindings

| Key | Action |
|---|---|
| `<leader>h` | Toggle `hlsearch` |
| `<Esc><Esc>` | Clear current highlights |
| `n` / `N` | Next/previous match (re-activates highlighting) |

## Highlight Groups (normative)

| Group | Purpose |
|---|---|
| `Search` | All matches of the current search pattern |
| `IncSearch` | The match under cursor during incremental search |
| `CurSearch` | The specific match the cursor is on after search |
| `Substitute` | Preview highlight during `:s` with `inccommand` |

## Current Match Distinction

The match under the cursor uses the `CurSearch` highlight group, which is visually distinct from other `Search` matches (e.g., brighter or different color).

## Match Blinking

On `n`/`N` jumps, the current match briefly blinks (100ms off/on) to draw attention to the new cursor position.

## Performance

For very large files, highlights are computed only for the visible viewport plus a small margin. Scrolling incrementally highlights new regions.

## Highlight Timeout

Optional auto-clear after a timeout: `search_highlight_timeout = 5000` (ms). Set to `0` to disable.

## Substitute Preview

When `inccommand = "split"` or `inccommand = "nosplit"`, `:s` commands show a live preview of replacements using the `Substitute` highlight group.

## Related

- Search count: [/docs/spec/editing/search/search-count.md](/docs/spec/editing/search/search-count.md)
- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
