# Path Expansion

Filename modifiers and wildcards in command-line mode.

## Overview

Special tokens `%` and `#` expand to file paths. Modifiers transform the result. Wildcards match filesystem entries.

## Current File (%)

`%` expands to the name of the current buffer's file (relative to CWD). If the buffer has no file, the expansion is empty and produces an error.

## Alternate File (#)

`#` expands to the alternate buffer's file name. `#{N}` expands to buffer number N's file name.

## Modifiers (normative)

Modifiers are appended with `:` and applied left to right.

| Modifier | Result |
|---|---|
| `:p` | Full (absolute) path |
| `:h` | Head: directory component |
| `:t` | Tail: filename component |
| `:r` | Root: remove last extension |
| `:e` | Extension only |
| `:~` | Home-relative (replace `$HOME` with `~`) |
| `:.` | CWD-relative path |
| `:s?pat?sub?` | Substitute first occurrence of `pat` with `sub` |
| `:gs?pat?sub?` | Substitute all occurrences |

## Chaining Examples

For file `/home/user/src/main.rs`:

| Expression | Result |
|---|---|
| `%` | `main.rs` (or relative path) |
| `%:p` | `/home/user/src/main.rs` |
| `%:p:h` | `/home/user/src` |
| `%:t` | `main.rs` |
| `%:r` | `main` |
| `%:e` | `rs` |
| `%:~` | `~/src/main.rs` |
| `%:p:h:t` | `src` |

Modifiers chain left-to-right: `%:p` gives the absolute path, then `:h` extracts the directory, then `:t` extracts the last directory component.

## Special Tokens (normative)

| Token | Meaning |
|---|---|
| `<cfile>` | File name under cursor |
| `<cword>` | Word under cursor |
| `<cWORD>` | WORD under cursor |
| `<sfile>` | Name of the sourced script file |
| `<afile>` | Autocmd event file |

These tokens accept the same modifiers: `<cfile>:p:h` gives the directory of the file under cursor.

## Shell Escaping

When passing expanded paths to shell commands (`:!`), special characters MUST be escaped. `shellescape()` or `fnameescape()` should be applied when constructing shell commands programmatically.

## Wildcards (normative)

| Pattern | Matches |
|---|---|
| `*` | Any characters except `/` |
| `**` | Any characters including `/` (recursive) |
| `?` | Single character |
| `[abc]` | Character class |

Wildcards are expanded when used with commands like `:edit`, `:args`, `:next`.

## Environment Variables

`$VAR` in file paths is expanded using the process environment. `~` at the start of a path expands to `$HOME`.

## Related

- Command-line entry: [/docs/spec/commands/cmdline/cmdline-entry.md](/docs/spec/commands/cmdline/cmdline-entry.md)
- File commands: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)
