# Jump List

Navigate through position history.

## Overview

The jump list tracks cursor positions for navigation
back through your editing history.

## Navigation

| Key | Action |
|-----|--------|
| `<C-o>` | Jump back |
| `<C-i>` | Jump forward |
| `<Tab>` | Jump forward (alternate) |

## What Creates Jumps

### Movements

| Movement | Creates Jump |
|----------|--------------|
| `G`, `gg` | Yes |
| `{count}G` | Yes |
| `/`, `?` | Yes |
| `n`, `N` | Yes |
| `%` | Yes |
| `(`, `)` | Yes |
| `{`, `}` | Yes |
| `[[`, `]]` | Yes |
| `H`, `M`, `L` | Yes |
| `:123` | Yes |
| `'mark` | Yes |

### Not Jumps

| Movement | Creates Jump |
|----------|--------------|
| `h`, `j`, `k`, `l` | No |
| `w`, `b`, `e` | No |
| `f`, `t` | No |
| `^`, `$` | No |

## Viewing Jump List

### Command


### Display


## Cross-File Jumps

Jumps work across files:


## Configuration


## Clearing

### Command


### Start Fresh

Clears jump history.

## Integration

### With Marks


### With Changes


## Tips

1. Use `<C-o>` after search to go back
2. Jump list persists in session
3. Works across files
4. Combine with marks

## Comparison

### Jump List vs Change List

| Feature | Jump List | Change List |
|---------|-----------|-------------|
| Tracks | Positions | Edits |
| Trigger | Large moves | Any change |
| Navigate | `<C-o>`/`<C-i>` | `g;`/`g,` |

## Workflow

### Exploring Code


### After Search


## Commands

