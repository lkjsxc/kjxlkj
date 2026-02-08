# Buffer Navigation

Switch between open buffers.

## Overview

Multiple files can be open simultaneously as buffers.
Navigation commands switch between them without closing.

## Basic Commands

### By Number

| Command | Description |
|---------|-------------|
| `:buffer {n}` | Switch to buffer number {n} |
| `:b {n}` | Short form |
| `:buffer {name}` | Switch by partial name match |
| `:b {name}` | Short form |

### By Position

| Command | Description |
|---------|-------------|
| `:bnext` / `:bn` | Next buffer |
| `:bprevious` / `:bp` | Previous buffer |
| `:bfirst` / `:bf` | First buffer |
| `:blast` / `:bl` | Last buffer |

## Normal Mode Keys

### Default Bindings

| Key | Action |
|-----|--------|
| `]b` | Next buffer |
| `[b` | Previous buffer |
| `]B` | Last buffer |
| `[B` | First buffer |

### With Count

`3]b` moves forward 3 buffers.
`2[b` moves backward 2 buffers.

## Alternate Buffer

### Toggle

`<C-^>` switches to the alternate (previously edited)
buffer. Each window has its own alternate buffer.

### With Count

`{n}<C-^>` switches to buffer number {n}.

## Buffer List

### Show Buffers

`:buffers` or `:ls` shows all buffers.

### Output Format

Each line: `{nr} {flags} "{name}" line {n}`

### Flags

| Flag | Meaning |
|------|---------|
| `%` | Current buffer |
| `#` | Alternate buffer |
| `a` | Active (loaded and displayed) |
| `h` | Hidden (loaded but not displayed) |
| `+` | Modified |
| `-` | Not modifiable |
| `=` | Read-only |

## Tab Completion

### Name Matching

`:buffer <Tab>` cycles through buffer names.
Partial matching: `:buffer mai<Tab>` finds `main.rs`.

### Fuzzy Match

The buffer picker supports fuzzy matching for quick
navigation when many buffers are open.

## Hidden Buffers

### Behavior

When `hidden = true`, modified buffers can be hidden
(navigated away from) without writing. They remain in
the buffer list.

### Warning

When `hidden = false`, attempting to leave a modified
buffer shows a warning: `E37: No write since last change`.

## Unloading

### Unload Buffer

`:bunload {n}` unloads buffer (frees memory, keeps in list).
`:bdelete {n}` removes from buffer list.
`:bwipeout {n}` removes completely (clears undo, marks).

### Ranges

`:3,5bdelete` deletes buffers 3 through 5.
`:bdelete file1.rs file2.rs` deletes by name.
