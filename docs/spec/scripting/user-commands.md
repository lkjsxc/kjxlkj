# User-Defined Commands

Back: [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md)

Users can define custom ex commands.

## Definition (normative)

| Command | Description |
|---|---|
| `:command {Name} {replacement}` | Define command `:{Name}` that executes `{replacement}` |
| `:command! {Name} {replacement}` | Define (overwrite existing) |
| `:delcommand {Name}` | Remove user command `{Name}` |
| `:command` | List all user-defined commands |

User command names MUST start with an uppercase letter to distinguish from built-in commands.

## Arguments (normative)

User commands can accept arguments via the `-nargs` flag:

| Flag | Meaning |
|---|---|
| `-nargs=0` | No arguments (default) |
| `-nargs=1` | Exactly one argument |
| `-nargs=*` | Any number of arguments |
| `-nargs=?` | Zero or one argument |
| `-nargs=+` | One or more arguments |

In the replacement text, `<args>` expands to the user-supplied arguments. `<q-args>` quotes them.

## Range support

| Flag | Meaning |
|---|---|
| `-range` | Command accepts a range; `<line1>` and `<line2>` expand to range bounds |
| `-range=%` | Default range is the whole file |
| `-count={N}` | Command accepts a count with default N |

## Bang support

With the `-bang` flag, the command accepts `!`. In the replacement, `<bang>` expands to `!` if the user supplied it, or empty otherwise.

## Completion

The `-complete={type}` flag enables tab-completion for the command's arguments:

| type | Completes |
|---|---|
| `file` | File names |
| `dir` | Directory names |
| `buffer` | Buffer names |
| `command` | Ex commands |
| `color` | Colorscheme names |
| `custom,{func}` | User-defined completion function |

## Related

- Commands overview: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)
- User functions: [/docs/spec/scripting/user-functions.md](/docs/spec/scripting/user-functions.md)

