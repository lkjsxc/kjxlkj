# Buffer Listing

Displaying and filtering buffers.

## Overview

View all open buffers with
status and information.

## Basic Commands

### List Buffers


## Output Format

### Example


### Columns

1. Buffer number
2. Flags
3. Filename
4. Current line

## Buffer Flags

### Indicators

| Flag | Meaning                |
|------|------------------------|
| `%`  | Current buffer         |
| `#`  | Alternate buffer       |
| `a`  | Active (in window)     |
| `h`  | Hidden (loaded)        |
| ` `  | Not loaded             |
| `+`  | Modified               |
| `-`  | Read-only (modifiable) |
| `=`  | Read-only (file perms) |
| `R`  | Running terminal       |
| `F`  | Finished terminal      |
| `?`  | Not read yet           |
| `u`  | Unlisted buffer        |

## List Variants

### Include Unlisted


### Filter Modified


### Filter Active


## Filter Command

### Pattern Matching


### Inverse Filter


## Buffer Information

### Get Info


### All Info


## Buffer Types

### Regular Files

Normal editing buffers.

### Special Buffers

| Type      | Purpose            |
|-----------|-------------------|
| nofile    | Scratch           |
| nowrite   | Can't save        |
| help      | Help buffer       |
| quickfix  | Error list        |
| terminal  | Terminal          |
| prompt    | Input prompt      |

### Check Type


## Unlisted Buffers

### What's Unlisted

- Help buffers
- Quickfix
- Preview
- Terminal

### Make Listed


## Buffer States

### Loaded

Content in memory.

### Hidden

Loaded but no window.

### Unloaded

Not in memory.

### Deleted

Removed from list.

## Commands

### By Number


### By Name


## Buffer Picker

### Fuzzy Find


### Features

- Search by name
- Preview content
- Delete option

## Buffer Line UI

### Enable
