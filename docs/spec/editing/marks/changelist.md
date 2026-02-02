# Changelist

Navigation through position history of buffer changes.

## Overview

The changelist tracks positions where changes occurred in a buffer. Each buffer maintains its own changelist.

## Navigation

| Key | Action |
|-----|--------|
| `g;` | Jump to older change position |
| `g,` | Jump to newer change position |

## How It Works


## What Creates Entries

Entries added for:
- Insert mode changes
- Delete operations
- Substitute commands
- Put/paste operations
- Indent/format operations

NOT added for:
- Cursor movement
- Search
- Yank (without change)
- Undo/redo

## Viewing the Changelist


Output:

## Changelist vs Jumplist

| Feature | Changelist | Jumplist |
|---------|------------|----------|
| Scope | Per buffer | Global |
| Tracks | Change positions | Navigation positions |
| Keys | `g;` / `g,` | `Ctrl-O` / `Ctrl-I` |
| Command | `:changes` | `:jumps` |

## Relationship with `'.`

- `'.` jumps to the most recent change
- `g;` traverses the entire history
- They work together for change navigation


## Count with Changelist


## Changelist Limits


## Merging Nearby Changes

Multiple edits on same/adjacent lines combine:


## Persistence


## Use Cases

### Return to Last Edit


### Review All Changes


### After Undo


## Related Marks

| Mark | Purpose |
|------|---------|
| `'.` | Last change position |
| `'[` | Start of last change |
| `']` | End of last change |

## Configuration


## API Reference

