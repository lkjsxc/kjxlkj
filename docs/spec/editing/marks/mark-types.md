# Mark Types

Different categories of marks for position tracking.

## Overview

| Type | Range | Scope | Persistence |
|------|-------|-------|-------------|
| Lowercase (a-z) | Local | Buffer only | Session |
| Uppercase (A-Z) | Global | Cross-buffer | Persistent |
| Numbered (0-9) | Global | Jump history | Persistent |
| Special | Various | Context-dependent | Session |

## Lowercase Marks (a-z)

Local to current buffer only.

### Setting


### Jumping


### Properties

- 26 marks per buffer (a-z)
- Cleared when buffer is closed
- Not visible in other buffers
- Line and column tracked

## Uppercase Marks (A-Z)

Global marks accessible from any buffer.

### Setting


### Jumping


### Properties

- 26 global marks (A-Z)
- Remembered across buffers
- Persisted across sessions
- Include file path

## Numbered Marks (0-9)

Special global marks for file history.

| Mark | Content |
|------|---------|
| `'0` | Last file edited (before exit) |
| `'1` | Second to last file |
| `'2` | Third to last file |
| ... | ... |
| `'9` | Ninth to last file |

### Usage


### Properties

- Automatically set on exit
- Rotate when new files added
- Useful for "continue where I left off"

## Special Marks

| Mark | Description |
|------|-------------|
| `'<` | Start of last visual selection |
| `'>` | End of last visual selection |
| `'[` | Start of last change/yank |
| `']` | End of last change/yank |
| `'^` | Last insert mode position |
| `'.` | Last change position |
| `''` | Previous position (before jump) |
| `` ` `` | Same as `''` (backtick version) |
| `'"` | Position when last editing buffer |

See [special-marks.md](special-marks.md) for details.

## Mark Storage

### In Memory


### Persistence


## Viewing Marks


## Mark Lifetime

| Type | Created | Deleted |
|------|---------|---------|
| Lowercase | User sets | Buffer close or :delmarks |
| Uppercase | User sets | :delmarks or never |
| Numbered | On exit | Automatically rotated |
| Special | Automatically | On new operation |

## Configuration


## Jump Behavior

| Key | Movement |
|-----|----------|
| `'m` | First non-blank of mark line |
| `` `m `` | Exact column of mark |
| `g'm` | Without adding to jumplist |
| `` g`m `` | Exact, no jumplist |

## Mark vs Cursor Position


## API Reference

