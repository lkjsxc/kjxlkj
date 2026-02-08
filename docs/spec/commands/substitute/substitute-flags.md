# Substitute Flags

Modifiers for the `:s` command.

## Flag Reference (normative)

| Flag | Name | Effect |
|---|---|---|
| `g` | Global | Replace all occurrences on the line (not just first) |
| `c` | Confirm | Prompt for each replacement |
| `i` | Ignore case | Case-insensitive matching (overrides `ignorecase`) |
| `I` | No ignore case | Case-sensitive matching (overrides `ignorecase`) |
| `n` | Count | Report match count without replacing |
| `e` | Error suppress | Suppress "pattern not found" error |
| `&` | Reuse flags | Use flags from previous `:s` command |
| `p` | Print | Print each changed line |
| `#` | Number | Print each changed line with line number |
| `l` | List | Print each changed line in list format |

## Global Flag (`g`)

Without `g`: only the first match on each line is replaced.
With `g`: all matches on each line are replaced.

## Confirm Flag (`c`)

Displays each match highlighted and prompts for action:

| Response | Action |
|---|---|
| `y` | Replace this match |
| `n` | Skip this match |
| `a` | Replace all remaining matches |
| `q` | Quit substitution |
| `l` | Replace this match and quit (last) |
| `<C-e>` | Scroll up |
| `<C-y>` | Scroll down |

## Count Flag (`n`)

Reports the number of matches without modifying the buffer. Useful for previewing how many replacements would occur.

## Error Suppression (`e`)

Normally, `:s/pattern/replacement/` errors if `pattern` is not found. With `e`, no error is raised. Essential for `:argdo` / `:bufdo` scripts where some buffers may not contain the pattern.

## Flags Reuse (`&`)

`&` reuses the flags from the last `:s` command. Can be combined with additional flags: `:s/old/new/&g` reuses previous flags plus adds `g`.

## Flag Combinations

Common combinations:

| Flags | Use Case |
|---|---|
| `gc` | Interactive global replace |
| `ge` | Silent global replace (no error if missing) |
| `gn` | Preview count of global matches |
| `gi` | Case-insensitive global replace |

## Related

- Substitute command: [/docs/spec/commands/substitute/substitute.md](/docs/spec/commands/substitute/substitute.md)
- Substitute specials: [/docs/spec/commands/substitute/substitute-specials.md](/docs/spec/commands/substitute/substitute-specials.md)
