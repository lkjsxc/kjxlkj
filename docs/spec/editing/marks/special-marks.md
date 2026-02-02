# Special Marks

Automatically maintained marks for common editing positions.

## Visual Selection Marks

### `'<` - Start of Visual Selection


### `'>` - End of Visual Selection


### Usage in Commands


## Change/Yank Marks

### `'[` - Start of Last Change


### `']` - End of Last Change


### After Yank


### After Paste


## Insert Position Marks

### `'^` - Last Insert Stop Position


### `'.` - Last Change Position


## Jump Marks

### `''` - Previous Position (Line)


### ``` `` ``` - Previous Position (Exact)


## Buffer Position Mark

### `'"` - Last Position in Buffer

When reopening a buffer:

### Auto-restore


## Sentence and Paragraph

### `'(` and `')` - Around Sentence


### `'{` and `'}` - Around Paragraph


## Summary Table

| Mark | Content | Updated When |
|------|---------|--------------|
| `'<` | Visual start | Visual mode exit |
| `'>` | Visual end | Visual mode exit |
| `'[` | Change/yank start | After change/yank/put |
| `']` | Change/yank end | After change/yank/put |
| `'^` | Last insert exit | Leaving insert mode |
| `'.` | Last change | After any change |
| `''` | Previous line | After jump |
| ``` `` ``` | Previous position | After jump |
| `'"` | Last buffer position | On buffer leave |

## Persistence

Special marks are session-only (not persisted):


Exception: `'"` is persisted for restore-on-open feature.

## Using with Operators

### Reselect Last Visual


### Operate on Last Change


### Operate on Last Paste


## Common Patterns

### Format Pasted Text


### Uppercase Last Insert


### Delete Last Change

