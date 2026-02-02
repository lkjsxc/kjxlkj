# Statusline DSL

Domain-specific language for statusline.

## Overview

The statusline DSL provides a flexible way
to customize the status bar appearance.

## Basic Syntax


## Variables

### File Information

| Variable | Description |
|----------|-------------|
| `%{filename}` | File name |
| `%{filepath}` | Full path |
| `%{filetype}` | File type |
| `%{fileformat}` | Line endings |
| `%{encoding}` | File encoding |
| `%{filesize}` | File size |

### Buffer State

| Variable | Description |
|----------|-------------|
| `%{modified}` | Modified flag |
| `%{readonly}` | Read-only flag |
| `%{bufnr}` | Buffer number |
| `%{bufcount}` | Total buffers |

### Position

| Variable | Description |
|----------|-------------|
| `%{line}` | Current line |
| `%{col}` | Current column |
| `%{lines}` | Total lines |
| `%{percent}` | Position % |

### Editor State

| Variable | Description |
|----------|-------------|
| `%{mode}` | Current mode |
| `%{paste}` | Paste mode |
| `%{spell}` | Spell check |
| `%{recording}` | Macro recording |

### Git Information

| Variable | Description |
|----------|-------------|
| `%{branch}` | Git branch |
| `%{diff_added}` | Lines added |
| `%{diff_modified}` | Lines changed |
| `%{diff_removed}` | Lines removed |

### Diagnostics

| Variable | Description |
|----------|-------------|
| `%{errors}` | Error count |
| `%{warnings}` | Warning count |
| `%{hints}` | Hint count |
| `%{info}` | Info count |

## Formatting

### Padding


### Truncation


### Conditional


## Styling

### Colors


### Highlight Groups

| Group | Description |
|-------|-------------|
| `StatusLine` | Normal statusline |
| `StatusLineNC` | Inactive window |
| `StatusMode` | Mode indicator |
| `StatusFile` | File info |
| `StatusGit` | Git info |
| `StatusDiag` | Diagnostics |

## Components

### Pre-built Components


### Available Components

| Component | Shows |
|-----------|-------|
| `mode` | Mode indicator |
| `filename` | File name |
| `filepath` | Full path |
| `filetype` | Language |
| `position` | Line:Col |
| `percent` | Scroll % |
| `git` | Branch + diff |
| `diagnostics` | LSP issues |
| `encoding` | UTF-8 etc |
| `indent` | Tab/space |
| `lsp` | LSP status |

## Separators


## Sections


## Per-Mode Colors


## Inactive Windows


## Examples

### Minimal


### Full-Featured

