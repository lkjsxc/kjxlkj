# Special Marks

Automatically maintained mark positions.

## Overview

Special marks are set automatically by the editor
during editing operations. They cannot be set manually
with `m`.

## Change Marks

### Last Change Position

| Mark | Description |
|------|-------------|
| `` `. `` | Position of last change |
| `'.` | Line of last change |
| `` `[ `` | Start of last changed/yanked text |
| `` `] `` | End of last changed/yanked text |

### Usage

`` `. `` is updated after every edit (insert, delete,
change, put). It always points to the start of the
most recent modification.

## Insert Marks

### Last Insert

| Mark | Description |
|------|-------------|
| `` `^ `` | Position where insert mode was last exited |

### Purpose

Used by `gi` which goes to `` `^ `` and enters insert mode.

## Visual Marks

### Visual Selection Bounds

| Mark | Description |
|------|-------------|
| `` `< `` | Start of last visual selection |
| `` `> `` | End of last visual selection |

### Persistence

These marks persist after visual mode ends. They define
the range `'<,'>` used with Ex commands.

## Sentence/Paragraph Marks

### Context Marks

| Mark | Description |
|------|-------------|
| `` `( `` | Start of current sentence |
| `` `) `` | End of current sentence |
| `` `{ `` | Start of current paragraph |
| `` `} `` | End of current paragraph |

## File Marks

### Buffer Position

| Mark | Description |
|------|-------------|
| `'"` | Position when last exiting the buffer |
| `` `" `` | Exact position when last exiting |

### Restore

When a file is reopened, the cursor can be restored
to the position indicated by `'"`.

## Jumplist Marks

### Previous Context

| Mark | Description |
|------|-------------|
| `` `` `` | Position before last jump |
| `''` | Line before last jump |

### Usage

These marks are set before any "jump" command (search,
tag jump, marks, etc.) and provide a quick way back.

## Numbered Marks

### Position History

Marks `0`-`9` store positions across files from
previous editing sessions:
- `'0` — position when last exiting kjxlkj
- `'1` — position before that
- `'2`-`'9` — older positions (shifted down)

### Rotation

Each time kjxlkj exits, `'0` gets the current position,
`'0` shifts to `'1`, `'1` to `'2`, etc. `'9` is lost.

## Read-Only

### Cannot Set

Special marks cannot be set with `m{mark}`. They are
maintained automatically by the editor. Attempting to
set them produces no error but has no effect.

### Can Jump

All special marks can be jumped to with `` ` `` or `'`.
They can also be used in Ex command ranges.
