# Git Signs (Gutter Integration)

kjxlkj shows git changes in the gutter.

## Overview

Visual indicators for line changes:


Symbols:
- `+` Added lines
- `~` Modified lines
- `-` Deleted lines indicator

## Configuration


### Colors


## Features

### Blame Line

Show git blame for current line:


Displays: `John Doe • 2 days ago • feat: add feature`

### Blame Virtual Text


## Navigation

Jump between hunks:


## Hunk Operations

### Preview Hunk


Shows diff popup for current hunk.

### Stage Hunk


### Stage Buffer


## Word Diff

Highlight changed words within lines:


## Staging from Visual

In visual mode, stage selected lines:


## Commands

| Command | Description |
|---------|-------------|
| `:Gitsigns toggle_signs` | Toggle signs |
| `:Gitsigns toggle_current_line_blame` | Toggle blame |
| `:Gitsigns diffthis` | Diff with index |
| `:Gitsigns diffthis ~` | Diff with HEAD |

## Status Integration

Statusline shows git info:


Displays: ` main +2 ~1 -0`

## Diff Mode

Open side-by-side diff:


Split view showing working copy vs index.
