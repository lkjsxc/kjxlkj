# Search Highlighting

Visual feedback for search matches.

## Highlight Commands

| Command | Action |
|---------|--------|
| `:set hlsearch` | Enable search highlighting |
| `:set nohlsearch` | Disable highlighting |
| `:noh` / `:nohlsearch` | Clear current highlights |
| `:set hls!` | Toggle highlighting |

## Highlight Behavior

### During Search

With `incsearch` enabled:
- Current match highlighted as you type
- `Ctrl-G` / `Ctrl-T` cycle through matches

### After Search

With `hlsearch` enabled:
- All matches highlighted
- Persists until `:noh` or new search

## Keybindings

| Key | Action |
|-----|--------|
| `<Leader>h` | Toggle hlsearch |
| `<Esc><Esc>` | Clear highlights |
| `<Leader>/` | Clear search highlights |
| `n` | Next match (re-highlights) |
| `N` | Previous match |

## Highlight Groups

| Group | Purpose |
|-------|---------|
| `Search` | Current search matches |
| `IncSearch` | Incremental search match |
| `CurSearch` | Match under cursor |
| `Substitute` | Substitute preview |

## Theme Configuration


## Match Blinking

Visual feedback when jumping to match:


## Match Under Cursor

Distinct highlight for current match:


## Multi-Match Display

### Show Count


### Configuration


## Incremental Highlighting

### Preview All Matches


### Cursor Line Only


## Performance Options

### Limit Highlight Scope


### Large File Handling


## Virtual Text Preview

Show replacement preview in buffer:


## Scrolling Behavior


## Highlight Timeout

Auto-clear highlights after time:


## Event Triggers

Clear highlights on specific events:


## statusline Integration



## Keymap Examples


## API Reference

