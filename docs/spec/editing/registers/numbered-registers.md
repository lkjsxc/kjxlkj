# Numbered Registers

Automatic registers 0-9 for delete and yank history.

## Register 0 - Last Yank

Always contains the most recent yank:


### Use Case: Replace Without Losing


## Registers 1-9 - Delete History

| Register | Content |
|----------|---------|
| `"1` | Most recent delete |
| `"2` | Second most recent |
| `"3` | Third most recent |
| ... | ... |
| `"9` | Ninth most recent |

### Rotation

When you delete text:
1. Contents of 9 are lost
2. 8 moves to 9
3. 7 moves to 8
4. ... and so on
5. New delete goes to 1


## Small vs Large Deletes

| Operation | Destination |
|-----------|-------------|
| Delete ≥1 line | Registers 1-9 |
| Delete <1 line | Small delete register `-` |
| Change operations | Follows same rules |


## Viewing Numbered Registers


## Accessing History


### Undo and Cycle Pattern


## Expression Access


## Command Line


## Insert Mode

| Key | Action |
|-----|--------|
| `Ctrl-R 0` | Insert yank register |
| `Ctrl-R 1` | Insert recent delete |
| `Ctrl-R 9` | Insert oldest delete |

## Configuration


## Interaction with Named Registers


Both specified register and numbered register receive content.

## Register Priority

When deleting/yanking:
1. If named register specified: Use that + numbered
2. If no register: Use unnamed + numbered
3. Yank always updates 0
4. Delete always rotates 1-9 (if line+)

## Practical Examples

### Recover Deleted Text


### Access Old Deletes


### Replace Multiple Times


## API Reference


## See Also

- [named-registers.md](named-registers.md) - Registers a-z
