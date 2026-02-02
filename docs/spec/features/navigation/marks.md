# Marks and Bookmarks

kjxlkj supports vim-style marks and enhanced bookmarks.

## Vim Marks

### Setting Marks


Examples:
- `ma` - Set mark 'a' at cursor
- `mA` - Set global mark 'A'

### Jumping to Marks


## Special Marks

| Mark | Description |
|------|-------------|
| `'` or `` ` `` | Previous jump position |
| `.` | Last change position |
| `"` | Last exit position in buffer |
| `^` | Last insert position |
| `[` | Start of last yank/change |
| `]` | End of last yank/change |
| `<` | Start of last visual selection |
| `>` | End of last visual selection |

## Mark Navigation


## Listing Marks


## Bookmarks (Enhanced)

Beyond vim marks, kjxlkj provides bookmarks:


### Annotated Bookmarks

Add notes to bookmarks:


Prompts for annotation text.

### Bookmark Groups


## Configuration


## Persistence


## Jump List

Separate from marks, tracks jump history:


## Finder Integration

Find marks/bookmarks via fuzzy finder:


## Commands

| Command | Description |
|---------|-------------|
| `:marks` | List marks |
| `:delmarks {marks}` | Delete marks |
| `:BookmarkToggle` | Toggle bookmark |
| `:BookmarkList` | List bookmarks |
| `:BookmarkClear` | Clear all bookmarks |
