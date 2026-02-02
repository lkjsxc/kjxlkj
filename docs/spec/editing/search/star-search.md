# Star Search

Search for word under cursor with * and # commands.

## Basic Commands

| Key | Action |
|-----|--------|
| `*` | Search forward for word under cursor |
| `#` | Search backward for word under cursor |
| `g*` | Search forward (partial match) |
| `g#` | Search backward (partial match) |

## Word Boundaries

### Whole Word (*/#)


### Partial Word (g*/g#)


## Visual Mode Star

| Key | Mode | Action |
|-----|------|--------|
| `*` | V | Search forward for selection |
| `#` | V | Search backward for selection |


## Behavior Details

### Cursor Positioning


### With Count


## Search Pattern Generated


## Case Sensitivity

Star search follows current settings:


Override with settings:
- Word "Test" with smartcase → case sensitive
- Word "test" with smartcase → case insensitive

## Special Characters

Star search escapes special characters:


## WORD vs word


## Navigation After Star

| Key | Action |
|-----|--------|
| `n` | Next occurrence |
| `N` | Previous occurrence |
| `gn` | Select next match |
| `gN` | Select previous match |

## Combining with Operators


## Dot-Repeat Pattern


## Configuration


## Stay on Current Match

Some users prefer `*` to stay on current word:


With this setting:
- `*` highlights all matches, cursor stays
- `n` moves to next match

## Keybinding Customization


## API Reference

