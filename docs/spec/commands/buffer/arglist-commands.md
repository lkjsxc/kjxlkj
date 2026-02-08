# Argument List Commands

Manage the list of files passed on the command line.

## Overview

The argument list (arglist) tracks files given as command
line arguments. It provides sequential file navigation
and batch operations.

## Commands

### Navigation

| Command | Description |
|---------|-------------|
| `:argument {n}` | Edit argument {n} |
| `:next` | Edit next argument |
| `:previous` | Edit previous argument |
| `:first` | Edit first argument |
| `:last` | Edit last argument |

### Split Variants

| Command | Description |
|---------|-------------|
| `:sargument {n}` | Split and edit argument {n} |
| `:snext` | Split and edit next argument |
| `:sprevious` | Split and edit previous |

### Modification

| Command | Description |
|---------|-------------|
| `:args {files}` | Replace arglist with {files} |
| `:argadd {file}` | Add {file} to arglist |
| `:argdelete {pattern}` | Remove matching entries |
| `:argdedupe` | Remove duplicate entries |

## Listing

### Show Arglist

`:args` with no arguments shows the current list.
The current file is shown in brackets: `[file.rs]`.

### Output Format

`[main.rs] lib.rs utils.rs tests/test.rs`

### Count

`:args` also shows position: `(1 of 4)`.

## Glob Expansion

### Wildcards

`:args src/*.rs` expands to all `.rs` files in `src/`.
`:args **/*.md` expands recursively.

### Backtick Expansion

`:args \`find . -name "*.rs"\`` uses shell output.

## Batch Operations

### argdo

`:argdo {cmd}` executes `{cmd}` in each argument file.

### Example

`:argdo %s/old/new/ge | update` replaces in all files.

### Error Handling

If `{cmd}` fails in a file, execution continues to the
next file unless `abort` is set. Errors are collected
and displayed at the end.

## Local Arglists

### Window-Local

`:argloca {files}` creates a window-local argument list.
Each window can have its own arglist independent of the
global one.

### Scope

Window-local arglists are inherited by splits from
that window.

## Integration

### Statusline

The statusline can show arglist position: `[2/5]`.

### Startup

Files passed on the command line (`kjxlkj a.rs b.rs`)
populate the initial argument list.

### Buffer List

Argument list entries are a subset of the buffer list.
All arglist files appear in the buffer list, but not
all buffers are in the arglist.

## Write Commands

### Write All

`:wall` writes all modified buffers, not just arglist.
`:argdo update` writes only modified arglist files.

### Quit All

`:qall` quits, warning about modified arglist files.
`:qall!` quits without warnings.
