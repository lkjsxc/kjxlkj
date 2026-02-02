# Jump Marks

Using quote and backtick for mark-based navigation.

## Two Jump Styles

| Key | Target | Movement |
|-----|--------|----------|
| `'m` | Mark m | First non-blank of line |
| `` `m `` | Mark m | Exact position (line + column) |

## Line vs Exact Position

### Apostrophe (')


### Backtick (`)


## Mark Navigation Commands

### Local Marks

| Command | Action |
|---------|--------|
| `'a` | Jump to line of mark a |
| `` `a `` | Jump to position of mark a |
| `'z` | Jump to line of mark z |
| `` `z `` | Jump to position of mark z |

### Global Marks

| Command | Action |
|---------|--------|
| `'A` | Jump to mark A (may switch buffer) |
| `` `A `` | Jump to exact position of A |
| `'Z` | Jump to global mark Z |

### Special Marks

| Command | Action |
|---------|--------|
| `''` | Previous position (line) |
| ``` `` ``` | Previous position (exact) |
| `'.` | Last change position |
| `'^` | Last insert position |
| `'<` | Start of visual selection |
| `'>` | End of visual selection |
| `'[` | Start of last change |
| `']` | End of last change |

## Without Jumplist (g prefix)

These jump without adding to jumplist:

| Command | Action |
|---------|--------|
| `g'a` | Jump to mark a (no jumplist) |
| `` g`a `` | Exact position (no jumplist) |
| `g''` | Previous position (no jumplist) |

## Jumping to Files

Global marks include file information:


## Mark Motions

Marks can be used as motions with operators:


## Visual Mode Selection


## Jump Behavior

### Adding to Jumplist

Jumps with `'` or `` ` `` add to jumplist:


### Wrapping


## Common Workflows

### Mark and Return


### Mark Range for Operations


### Visual Selection to Mark


## Quick Reference

| Key | Meaning |
|-----|---------|
| `'a-z` | Local mark line |
| `` `a-z `` | Local mark exact |
| `'A-Z` | Global mark line |
| `` `A-Z `` | Global mark exact |
| `'0-9` | File history |
| `''` | Previous line |
| ``` `` ``` | Previous exact |
| `'.` | Last change line |
| `` `. `` | Last change exact |
| `'^` | Last insert line |
| `` `^ `` | Last insert exact |
| `'<` `'>` | Visual bounds |
| `'[` `']` | Change bounds |
| `'"` | Last position in file |

## Configuration


## Error Handling


## API Reference

