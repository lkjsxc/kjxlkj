# Change List

Navigate through edit positions.

## Overview

The change list tracks positions where edits were
made, allowing navigation through edit history.

## Navigation

| Key | Action |
|-----|--------|
| `g;` | Go to older change |
| `g,` | Go to newer change |

## What Creates Entries

### Any Edit

- Insert text
- Delete text
- Change text
- Paste
- Format

### Each Undo Block

One change list entry per undo block.

## Viewing Change List

### Command


### Display


## Example Workflow


## Cross-File

Change list includes edits in all buffers:


Navigation crosses files.

## Configuration


## Marks

### Special Marks

| Mark | Description |
|------|-------------|
| `'.` | Last change position |
| `` `. `` | Last change (exact) |
| `'^` | Last insert position |
| `` `^ `` | Last insert (exact) |

## Comparison

### vs Jump List

| Feature | Jump List | Change List |
|---------|-----------|-------------|
| Purpose | Navigation | Edits |
| Trigger | Large moves | Changes |
| Keys | `<C-o>`/`<C-i>` | `g;`/`g,` |

### vs Undo

| Feature | Change List | Undo |
|---------|-------------|------|
| Purpose | Position | Content |
| Action | Move cursor | Revert |
| Keys | `g;`/`g,` | `u`/`<C-r>` |

## Use Cases

### Return to Edit

After viewing code:


### Review Edits


## Tips

1. Use after exploring code
2. Combine with undo for review
3. Check `:changes` for overview
4. Works across files

## Commands


## Integration

### With Sessions

Change list persisted in sessions.

### With Undo Tree

Navigate changes without undoing.
