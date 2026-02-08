# Find Command

Find and edit files by name.

## Overview

`:find` searches for files in the `path` option
directories and opens the first match.

## Basic Usage

### Find File

`:find {filename}` opens the file if found in `path`.
`:find main.rs` searches `path` for `main.rs`.

### With Wildcards

`:find *.rs` finds the first `.rs` file.
`:find src/**/test_*` searches recursively.

## Path Option

### Configuration

`path` controls where `:find` searches.
Default: `path = [".", "/usr/include", ""]`

### Path Components

| Value | Meaning |
|-------|---------|
| `.` | Current file directory |
| `""` (empty) | Current working directory |
| `**` | Recursive search subdirectory |
| `/path/` | Absolute path |

### Common Setups

`path = [".", "src/**", "tests/**"]` searches current
file dir, all of `src/`, and all of `tests/`.

## Suffixes

### suffixesadd

`:set suffixesadd=.rs,.toml` appends these extensions
when `:find` cannot find the exact file name.

`:find module` tries `module`, `module.rs`, `module.toml`.

## Tab Completion

### Filename Completion

`:find ` followed by `<Tab>` cycles through matching
files in `path`. `<C-d>` lists all matches.

### Wildcard Completion

`:find src/**/<Tab>` lists all files under `src/`.

## Split Variants

### Horizontal Split

`:sfind {file}` opens the file in a horizontal split.

### Vertical Split

`:vert sfind {file}` opens in a vertical split.

### Tab

`:tab sfind {file}` opens in a new tab.

## Count

### nth Match

`:2find {file}` opens the 2nd match if multiple files
match the pattern.

## Integration

### gf Command

`gf` (go to file) in normal mode opens the file under
the cursor, using `path` and `suffixesadd` for lookup.

`gF` opens file under cursor and jumps to line number
if the filename is followed by `:line` or `(line)`.

### Include Search

`:isearch`, `:dsearch` use `path` for header lookup.

## Error Handling

### Not Found

If no file matches, displays: `E345: Can't find file
"{filename}" in path`.

### Multiple Matches

`:find` opens the first match. Use tab completion
to see all matches and select the desired one.
