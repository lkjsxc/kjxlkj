# Search Commands

Forward and backward pattern searching with navigation.

## Forward Search (/)


## Backward Search (?)


## Search Navigation

| Key | Action |
|-----|--------|
| `n` | Repeat last search (same direction) |
| `N` | Repeat last search (opposite direction) |
| `/<CR>` | Repeat forward search |
| `?<CR>` | Repeat backward search |
| `gn` | Select next match (visual mode) |
| `gN` | Select previous match |

## Search Offsets


## Offset Examples


## Compound Searches


## Search in Selection


## Case Sensitivity


## Search Wrapping

| Option | Effect |
|--------|--------|
| `wrapscan` | Search wraps around file |
| `nowrapscan` | Stop at file boundaries |

## Search Count

Display match position information:


## Keybindings Summary

| Key | Mode | Action |
|-----|------|--------|
| `/` | N | Forward search |
| `?` | N | Backward search |
| `n` | N | Next match |
| `N` | N | Previous match |
| `gn` | N,V,O | Next match visual |
| `gN` | N,V,O | Previous match visual |
| `Ctrl-G` | / | Next search result |
| `Ctrl-T` | / | Previous search result |

## Search Registers


## Configuration


## API Integration


## See Also

- [search-options.md](search-options.md) - Search configuration
- [star-search.md](star-search.md) - Word under cursor
- [search-highlight.md](search-highlight.md) - Match highlighting
- [search-history.md](search-history.md) - History navigation
