# Command-Line Completion

Custom completion sources.

## Overview

Configure completion for command-line
mode with custom sources and behavior.

## Built-in Completions

### Types

| Type | Completes |
|------|-----------|
| `file` | File paths |
| `dir` | Directory paths |
| `buffer` | Buffer names |
| `command` | Commands |
| `option` | Options |
| `color` | Colorschemes |
| `highlight` | Highlight groups |
| `event` | Autocommand events |
| `help` | Help topics |
| `filetype` | Filetypes |
| `shellcmd` | Shell commands |

### Usage in Commands


## Custom Completions

### Static List


### Dynamic List


## Completion Behavior

### Configuration


### Wildmode Options

| Mode | Description |
|------|-------------|
| `""` | Complete to longest match |
| `full` | Complete to next full match |
| `longest` | Complete to longest common |
| `longest:full` | Longest, then cycle full |
| `list` | Show list of matches |
| `list:full` | List, then cycle full |

## Completion Keys

### Navigation

| Key | Action |
|-----|--------|
| `Tab` | Next completion |
| `Shift-Tab` | Previous completion |
| `Ctrl-N` | Next |
| `Ctrl-P` | Previous |
| `Ctrl-Y` | Accept |
| `Ctrl-E` | Cancel |

### Configuration


## Fuzzy Completion

### Enable


### Example

`:ed mai` matches `main.rs`

## File Completion

### Ignore Patterns


### Show Hidden


## Buffer Completion

### Options


## Path Expansion

### Home Directory


### Environment Variables


## History Completion

### Command History


### Search History


### Configuration


## Inline Completion

### Suggestions


Shows grayed suggestion inline.

## Menu Appearance

### Styling


### Position

