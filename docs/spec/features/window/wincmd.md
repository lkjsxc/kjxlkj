# Window Commands (:wincmd)

Complete window command reference.

## Overview

The `:wincmd` command executes window
operations, equivalent to `<C-w>` keys.

## Syntax


## Navigation

### Direction

| Command | Action |
|---------|--------|
| `:wincmd h` | Go to left window |
| `:wincmd j` | Go to below window |
| `:wincmd k` | Go to above window |
| `:wincmd l` | Go to right window |

### Other Navigation

| Command | Action |
|---------|--------|
| `:wincmd w` | Next window (wrap) |
| `:wincmd W` | Previous window (wrap) |
| `:wincmd t` | Go to top-left window |
| `:wincmd b` | Go to bottom-right window |
| `:wincmd p` | Go to previous window |

## Creating Windows

### Splits

| Command | Action |
|---------|--------|
| `:wincmd s` | Split horizontally |
| `:wincmd v` | Split vertically |
| `:wincmd n` | New window |

### With Count


## Closing Windows

### Close Commands

| Command | Action |
|---------|--------|
| `:wincmd c` | Close current window |
| `:wincmd q` | Quit current window |
| `:wincmd o` | Only: close others |

### Close Specific


## Moving Windows

### Position

| Command | Action |
|---------|--------|
| `:wincmd H` | Move window far left |
| `:wincmd J` | Move window far bottom |
| `:wincmd K` | Move window far top |
| `:wincmd L` | Move window far right |
| `:wincmd T` | Move window to new tab |

## Rotating Windows

### Rotation

| Command | Action |
|---------|--------|
| `:wincmd r` | Rotate downwards/rightwards |
| `:wincmd R` | Rotate upwards/leftwards |
| `:wincmd x` | Exchange with next window |

## Resizing Windows

### Size Adjustments

| Command | Action |
|---------|--------|
| `:wincmd +` | Increase height |
| `:wincmd -` | Decrease height |
| `:wincmd >` | Increase width |
| `:wincmd <` | Decrease width |
| `:wincmd =` | Make all equal size |
| `:wincmd _` | Maximize height |
| `:wincmd \|` | Maximize width |

### With Count


## Special Windows

### Preview Window

| Command | Action |
|---------|--------|
| `:wincmd P` | Go to preview window |
| `:wincmd z` | Close preview window |

### Quickfix

Go to quickfix window:

## Window Information

### Current Window


### All Windows


## Keybinding Equivalents

### Normal Mode

| Key | Equivalent |
|-----|------------|
| `<C-w>h` | `:wincmd h` |
| `<C-w>j` | `:wincmd j` |
| `<C-w>k` | `:wincmd k` |
| `<C-w>l` | `:wincmd l` |
| `<C-w>s` | `:wincmd s` |
| `<C-w>v` | `:wincmd v` |
| `<C-w>c` | `:wincmd c` |
| `<C-w>o` | `:wincmd o` |

## Scripting

### In Commands


### In Autocommands


## Window Variables

### Set Variable


### Check Variable


## Window Options

### List Options


### Window-Local
