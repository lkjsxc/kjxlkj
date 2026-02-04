# Conformance: Ex Commands and Testing

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
Command-line (Ex) command subset and headless/E2E surface in the conformance ledger.

In a docs-only baseline, treat this as the intended initial reconstruction target (update it after regenerating the implementation).

## Command-line (Ex) commands (subset)

| Command | Behavior |
|---|---|
| `:q` / `:q!` | Quit (forced with `!`). Refuses if buffer is modified unless forced. |
| `:qa` / `:qa!` | Quit all (forced with `!`) |
| `:w` | Write to current buffer path (if set) |
| `:w {file}` | Write to `{file}` |
| `:wa` / `:wall` | Write all buffers |
| `:wq` / `:x` / `:exit` | Write then quit |
| `:wq {file}` | Write to `{file}` then quit |
| `:e {file}` / `:e! {file}` | Edit file (forced with `!`) |
| `:ls` / `:buffers` | List open buffers |
| `:bn` / `:bnext` | Go to next buffer |
| `:bp` / `:bprev` / `:bprevious` | Go to previous buffer |
| `:bd` / `:bdelete` | Delete current buffer |
| `:bd!` / `:bdelete!` | Force delete current buffer |
| `:sp` / `:split` | Split window horizontally (stub) |
| `:vsp` / `:vsplit` | Split window vertically (stub) |
| `:new` | Open new empty buffer in split (stub) |
| `:vnew` | Open new empty buffer in vertical split (stub) |
| `:only` | Close all other windows (stub) |
| `:set {option}` | Set editor option (number, nonumber, wrap, nowrap) |
| `:{number}` | Go to line number |
| `:! {cmd}` | Run `{cmd}` via terminal service and display first output line as status |
| `:s/pattern/replacement/` | Substitute on current line |
| `:s/pattern/replacement/g` | Substitute all occurrences on current line |
| `:g/pattern/d` | Delete all lines matching pattern |
| `:g/pattern/command` | Execute command on matching lines |
| `:v/pattern/d` | Delete all lines NOT matching pattern (inverted global) |

## Headless test runner

This conformance target includes a deterministic headless mode intended for E2E tests and CI-like environments:

- `--headless` runs without a terminal UI.
- `--script {path}` runs an event script (if provided).
- An optional positional `{file}` argument loads a file into the initial buffer before the script runs.

### Script format

The headless script file is UTF-8 JSON in one of two accepted shapes:

| Shape | Description |
|---|---|
| Steps array | A JSON array of `ScriptStep` objects. Each element is an object with a `kind` discriminator. |
| Keys array | A JSON array of `ScriptKey` objects. Each element is a key description without `kind`. |

#### `ScriptKey` schema (used by both shapes)

| Field | Type | Required | Meaning |
|---|---:|:---:|---|
| `code` | string | yes | Key identity (either a single character, or a named special key). |
| `ctrl` | boolean | no | Whether Ctrl is held. Defaults to `false`. |
| `alt` | boolean | no | Whether Alt/Meta is held. Defaults to `false`. |
| `shift` | boolean | no | Whether Shift is held. Defaults to `false`. |

Named `code` values supported by this conformance target:

| `code` | Meaning |
|---|---|
| `Escape`, `Esc` | Escape key |
| `Enter`, `Return` | Enter/Return key |
| `Backspace` | Backspace key |
| `Tab` | Tab key |
| `Left`, `Right`, `Up`, `Down` | Arrow keys |

If `code` is not a named value above, it MUST be a single-character string.

#### `ScriptStep` kinds

| `kind` | Additional fields | Effect |
|---|---|---|
| `key` | `code`, optional `ctrl`/`alt`/`shift` | Inject one key event. |
| `keys` | `keys` (string) | Inject literal characters, one per Unicode scalar value. |
| `assert_mode` | `mode` (string) | Assert the current editor mode. |
| `assert_cursor` | `line` (integer), `col` (integer) | Assert the cursor position (0-based). |
| `assert_line` | `line` (integer), `content` (string) | Assert an exact line string match (0-based). |

Mode strings accepted by this conformance target include `normal`, `insert`, `visual`, `visual_line`, `visual_block`, `command`, `replace` (case-insensitive, with a small set of aliases).

## E2E test coverage

Once an implementation claims conformance to this document, it MUST include automated tests for at least the following E2E scenarios:

| Test | Behavior verified |
|---|---|
| headless_mode_starts | Editor starts, quits on `:q` |
| insert_mode | `i` enters Insert mode, text entry works |
| cursor_movement | `h/j/k/l` move cursor correctly |
| command_mode | `:` enters Command mode |
| visual_mode | `v` enters Visual mode |
| visual_line_mode | `V` enters Visual-Line mode |
| replace_mode | `R` enters Replace mode |
| append_mode | `a` appends after cursor |
| open_line_below | `o` opens line below |
| open_line_above | `O` opens line above |
| text_insert_delete | Insert mode text entry and backspace |
| word_motions | `w/b` word movement |
| line_motions | `0/$` line start/end |
| file_motions | `gg/G` file start/end |
| typing_burst | 50 chars typed rapidly in insert mode |
| scroll_burst | 20 lines scrolled rapidly with `j` |
| mode_switch_burst | 10 rapid Normal/Insert mode switches |
| input_ordering | Verify input sequence order preserved |

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
