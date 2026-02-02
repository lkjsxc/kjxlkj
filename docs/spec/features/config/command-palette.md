# Command Palette

Quick command execution interface.

## Overview

The command palette provides fuzzy search access
to all available commands.

## Opening

| Key | Action |
|-----|--------|
| `<C-S-p>` | Open command palette |
| `:` | Command line (different) |

## Interface


## Fuzzy Search

### Matching

Type partial strings:


### Ranking

1. Exact match
2. Prefix match
3. Subsequence match

## Command Categories

### File

- Save, Save As, Save All
- Open, Open Recent
- Close, Close All

### Edit

- Undo, Redo
- Cut, Copy, Paste
- Find, Replace

### View

- Toggle Sidebar
- Zoom In/Out
- Split

### Go

- Go to Line
- Go to Symbol
- Go to Definition

### LSP

- Rename
- Format
- Code Actions

## Configuration


## Keybindings Display


## Recent Commands


### Display


## Custom Commands

### Register


### Visible in Palette


## Navigation

| Key | Action |
|-----|--------|
| `<C-n>` | Next |
| `<C-p>` | Previous |
| `<CR>` | Execute |
| `<Esc>` | Cancel |
| `<Tab>` | Complete |

## Command Arguments

### With Prompt

Some commands prompt for input:


### Direct Input


## Tips

1. Use fuzzy matching liberally
2. Check keybindings in palette
3. Recent commands save time
4. Combine with `:` for power

## Comparison

| Feature | Palette | Command Line |
|---------|---------|--------------|
| Interface | Fuzzy picker | Text input |
| Discovery | Easy | Harder |
| Speed | Quick | Faster for experts |
| Arguments | Prompted | Inline |
