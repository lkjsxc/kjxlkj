# Read-Only Registers

Registers that reflect editor state and cannot be written directly.

## Register Reference (normative)

| Register | Name | Content |
|---|---|---|
| `".` | Last insert | Text from the most recent insert mode session |
| `"%` | Filename | Current buffer's relative filename |
| `"#` | Alternate | Alternate (previous) buffer filename |
| `":` | Last command | Most recently executed ex command (without `:`) |
| `"/` | Last search | Last search pattern |

## Last Insert Register (`.`)

Updated on every insert mode exit. Contains only the text inserted during the most recent insert session (not cumulative). If insert mode types `hello`, `".` contains `hello`.

## Filename Register (`%`)

Contains the current buffer's filename as a relative path. For path manipulation in commands, use expand modifiers:

| Expression | Result |
|---|---|
| `@%` | Relative filename |
| `expand("%:p")` | Absolute path |
| `expand("%:h")` | Directory part |
| `expand("%:t")` | Filename only (tail) |
| `expand("%:r")` | Without extension (root) |
| `expand("%:e")` | Extension only |

## Alternate File Register (`#`)

Contains the filename of the previous buffer. Updated on buffer switch. `<C-^>` toggles between current and alternate buffers.

## Last Command Register (`:`)

Contains the last ex command text. Useful for repeating complex commands with `@:`.

## Last Search Register (`/`)

Contains the last search pattern. Unlike other read-only registers, `@/` can be set programmatically to change the active search pattern.

## Accessing Read-Only Registers

| Context | Method |
|---|---|
| Normal mode | `"rp` where `r` is the register |
| Insert mode | `<C-r>r` |
| Command line | `<C-r>r` |

## Related

- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
