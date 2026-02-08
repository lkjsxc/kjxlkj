# Read Command

Insert file or command output into the buffer.

## Overview

`:read` inserts the contents of a file or the output
of a shell command below the current line.

## File Read

### Basic Usage

`:read {file}` inserts the contents of `{file}` below
the current line.

`:0read {file}` inserts above the first line.

### With Line Number

`:{n}read {file}` inserts below line `{n}`.

### Current File

`:read` without arguments re-reads the current file
and inserts below the current line.

## Command Read

### Shell Output

`:read !{command}` inserts the output of the shell
command below the current line.

### Examples

`:read !date` inserts the current date.
`:read !ls` inserts the directory listing.
`:read !sort %` inserts sorted version of current file.

### Error Handling

If the command fails (non-zero exit), the error output
is still inserted with an error message shown.

## Range

### Insert at Line

`:5read file.txt` inserts after line 5.
`:$read file.txt` inserts at end of file.
`:.read file.txt` inserts after current line (default).

### Replace Range

`:1,$read` replaces the entire buffer — not directly.
Use `:%d | read file.txt` to replace all content.

## Encoding

### Specify Encoding

`:read ++enc=utf-16 file.txt` reads with specified encoding.

### File Format

`:read ++ff=dos file.txt` reads with DOS line endings.

## Integration

### Pipe

`:read !grep pattern file.txt` inserts grep results.
`:read !curl -s url` inserts HTTP response.

### With Ranges

`:0read !head -1 file.txt` inserts first line at top.

## Undo

### Single Operation

The entire `:read` insertion is a single undo unit.
`u` removes all inserted lines.

## Related

### Write

`:write` is the inverse — writes buffer to file.
`:write !{command}` pipes buffer content to a command.

### Edit

`:edit {file}` opens a file (replaces buffer).
`:read` appends to the current buffer.
