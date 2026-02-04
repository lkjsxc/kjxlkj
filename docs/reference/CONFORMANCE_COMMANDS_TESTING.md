# Conformance: Ex Commands and Testing

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
Command-line (Ex) command subset and headless/E2E surface currently implemented.

## Command-line (Ex) commands (subset)

| Command | Behavior |
|---|---|
| `:q` / `:q!` | Quit (forced with `!`) |
| `:qa` / `:qa!` | Alias for quit / forced quit |
| `:w` | Write to current buffer path (if set) |
| `:w {file}` | Write to `{file}` |
| `:wa` | Alias for `:w` |
| `:wq` / `:x` | Write then quit |
| `:wq {file}` | Write to `{file}` then quit |
| `:e {file}` / `:e! {file}` | Edit file (forced with `!`) |
| `:! {cmd}` | Run `{cmd}` via terminal service and display first output line as status |
| `:s/pattern/replacement/` | Substitute on current line |
| `:s/pattern/replacement/g` | Substitute all occurrences on current line |
| `:g/pattern/d` | Delete all lines matching pattern |
| `:g/pattern/command` | Execute command on matching lines |
| `:v/pattern/d` | Delete all lines NOT matching pattern (inverted global) |

## Headless test runner

The shipped binary supports a deterministic headless mode for E2E tests:

- `--headless --script {path}` runs an event script without terminal UI.
- The script MAY be either:
  - a JSON array of keys, where each item is a `Key` object with `code` and `mods`
  - a JSON array of steps, where each item is a tagged object with `kind`

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
