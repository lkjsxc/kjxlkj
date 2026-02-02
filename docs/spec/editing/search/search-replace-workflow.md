# Search and Replace Workflow

Common patterns for finding and replacing text efficiently.

## Basic Workflow

### 1. Search First, Then Replace


### 2. Confirm Each Change


Confirmation prompts:
- `y` - Replace this match
- `n` - Skip this match
- `a` - Replace all remaining
- `q` - Quit substitution
- `l` - Replace and quit (last)
- `Ctrl-E` - Scroll up
- `Ctrl-Y` - Scroll down

## Visual Feedback Workflow

### Using gn Motion


### Advantages of cgn

- Repeatable with `.`
- Visual selection of match
- Skip matches with `n`
- No need to type pattern twice

## Multi-File Search and Replace

### Using argdo


### Using bufdo


### Using cfdo (Quickfix)


## Preserving Case

### Smart Case Replace


| Original | Replacement |
|----------|-------------|
| word | replacement |
| Word | Replacement |
| WORD | REPLACEMENT |

### Using \U, \L, \u, \l


## Pattern Capture Groups

### Basic Capture


### Named Groups (Very Magic)


### Backreferences


## Expression Substitution

### Increment Numbers


### Custom Functions


## Range-Based Replace


## Interactive Preview


Preview mode shows:
- Original text (struck through)
- Replacement text (highlighted)
- Match count per line

## Undo Integration


## Best Practices

### Test Pattern First


### Use Very Magic for Complex Patterns


### Backup Before Large Changes


### Use Confirmation for Risky Changes


## Common Patterns

| Task | Command |
|------|---------|
| Remove trailing whitespace | `:%s/\s\+$//g` |
| Convert tabs to spaces | `:%s/\t/    /g` |
| Delete empty lines | `:g/^$/d` |
| Remove duplicate lines | `:sort u` |
| Wrap lines in quotes | `:%s/.*/"\0"/g` |
| Add semicolons | `:%s/$/;/g` |

## Configuration

