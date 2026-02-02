# Text Manipulation Commands

Core text manipulation operations.

## Overview

Text manipulation commands modify content
efficiently across ranges and patterns.

## Basic Operations

### Delete

| Command | Action |
|---------|--------|
| `d{motion}` | Delete motion |
| `dd` | Delete line |
| `D` | Delete to end |
| `x` | Delete character |
| `X` | Delete before |

### Change

| Command | Action |
|---------|--------|
| `c{motion}` | Change motion |
| `cc` | Change line |
| `C` | Change to end |
| `s` | Substitute char |
| `S` | Substitute line |

### Yank

| Command | Action |
|---------|--------|
| `y{motion}` | Yank motion |
| `yy` | Yank line |
| `Y` | Yank line |

### Put

| Command | Action |
|---------|--------|
| `p` | Put after |
| `P` | Put before |
| `gp` | Put, cursor after |
| `gP` | Put before, cursor after |

## Line Operations

### Duplicate


### Move


### Delete Range


## Join Operations

### Join Lines

| Key | Action |
|-----|--------|
| `J` | Join with space |
| `gJ` | Join without space |

### Range Join


## Split Operations

### Split Line

| Key | Action |
|-----|--------|
| `r<CR>` | Split at cursor |
| `i<CR>` | Insert newline |

### Command


## Reformat

### Paragraph


### Width


## Indentation

### Shift

| Key | Action |
|-----|--------|
| `>>` | Indent line |
| `<<` | Outdent line |
| `>{motion}` | Indent motion |
| `={motion}` | Auto-indent |

### Range


## Replace

### Character


### Range


## Case Operations

| Key | Action |
|-----|--------|
| `~` | Toggle case |
| `g~{motion}` | Toggle motion |
| `gu{motion}` | Lowercase |
| `gU{motion}` | Uppercase |

## Rot13


## Reverse


## Unique Lines


## Number Lines


## Wrap Text


## Remove Empty Lines

