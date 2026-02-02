# Automatic Marks

Marks that are set automatically by the editor.

## List of Automatic Marks

| Mark | Set When | Content |
|------|----------|---------|
| `'.` | After change | Position of last change |
| `'^` | Insert exit | Last insert mode exit position |
| `'[` | After change/yank/put | Start of affected text |
| `']` | After change/yank/put | End of affected text |
| `'<` | Visual exit | Start of selection |
| `'>` | Visual exit | End of selection |
| `''` | After jump | Previous cursor position |
| `'"` | Buffer leave | Last position in buffer |
| `'0`-`'9` | Editor exit | Recent file positions |

## Change Marks

### Last Change Position (`'.`)


Updated by:
- Insert mode changes
- Delete operations
- Substitute commands
- Put/paste operations

### Change Bounds (`'[` and `']`)


### After Put


Useful pattern:

## Insert Marks

### Last Insert Position (`'^`)


## Visual Marks

### Selection Bounds (`'<` and `'>`)


## Jump Mark

### Previous Position (`''`)

Set automatically when you jump:


Triggered by:
- `G` commands
- Search (`n`, `N`, `/`, `?`)
- Mark jumps (`'a`, `` `a ``)
- `:number` line jumps
- `%` matching bracket

## Buffer Position (`'"`)


### Auto-restore


## File History (`'0`-`'9`)

Set on editor exit:

| Mark | Content |
|------|---------|
| `'0` | Last edited file position |
| `'1` | Second to last file |
| `'2` | Third to last file |
| ... | Rotates older positions |

Usage:

## When Marks Update

### Immediately

| Mark | Updated On |
|------|------------|
| `'.` | Any change |
| `'^` | Insert mode exit |
| `''` | Any jump |

### On Mode Exit

| Mark | Updated On |
|------|------------|
| `'<`, `'>` | Visual mode exit |
| `'[`, `']` | After operation complete |

### On Buffer/Editor Events

| Mark | Updated On |
|------|------------|
| `'"` | Buffer leave |
| `'0`-`'9` | Editor exit |

## Configuration


## Disabling Automatic Marks


Or selectively:

## API Reference

