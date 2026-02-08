# Conformance: Headless Testing and E2E

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Headless test runner, E2E coverage, and runtime feature conformance entries.

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

Mode strings accepted by this conformance target include `normal`, `insert`, `visual`, `visual_line`, `visual_block`, `command`, `replace`, `terminal` (case-insensitive, with a small set of aliases).

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
| append_at_eol | When cursor is on last character, `a` appends at true end-of-line (no off-by-one) |
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

These scenarios are primarily **headless** (no terminal UI). The project also requires **interactive PTY-driven E2E** tests for bugs that can hide in terminal decoding, key-chord parsing, focus/routing, or render-loop behavior. See [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).

## Command history

| Feature | Behavior |
|---|---|
| `CommandHistory.push()` | Adds entry, deduplicates consecutive identical entries |
| `CommandHistory.prev()` | Navigate to previous history entry |
| `CommandHistory.next()` | Navigate to next history entry |
| `search_prefix()` | Find previous entry matching prefix (backward search) |
| `search_prefix_forward()` | Find next entry matching prefix (forward search) |
| `search_substring()` | Find entries containing substring |

## Git status and diff

| Feature | Behavior |
|---|---|
| `FileStatus` | Unmodified, Modified, Added, Deleted, Renamed, Untracked, Ignored, Conflicted |
| `StatusEntry` | Path + status + staged flag |
| `parse_diff_hunks()` | Parses unified diff text into `DiffHunk` with lines |
| `compute_gutter_signs()` | Produces `(line, GutterSign)` from hunks |
| `parse_blame_output()` | Parses blame lines into `BlameLine` entries |

## Terminal emulator

| Feature | Behavior |
|---|---|
| `TerminalGrid` | Cell grid with cursor position and current style |
| `put_char()` | Write character at cursor, advance with wrapping |
| `clear()` / `clear_to_eol()` | Clear screen or to end of line |
| `scroll_up()` | Scroll grid up by one line |
| `parse_ansi_simple()` | Parse subset of ANSI escapes (cursor move, clear, SGR) |

## Large buffer support

| Feature | Behavior |
|---|---|
| `LoadStrategy` | Full (<=1MB), Chunked (<=100MB), Streamed (>100MB) |
| `build_line_index()` | Byte offsets of each line start |
| `compute_chunks()` | Split file into fixed-size chunks |
| `annotate_chunks_with_lines()` | Map chunks to line ranges |
| `extract_line_range()` | Extract text for line range using index |

## Range and address parsing

| Feature | Behavior |
|---|---|
| `Address` | CurrentLine (`.`), LastLine (`$`), LineNumber, Mark (`'a`), ForwardSearch (`/pat/`), BackwardSearch (`?pat?`), Offset |
| `Range` | None, Single, FromTo (`,` separated), Entire (`%`) |
| `parse_range()` | Parses range string into Range enum |
| `parse_address()` | Parses single address with offset support |
| `resolve_range()` | Resolves range to (start, end) line numbers |

## Plugin prevention

| Feature | Behavior |
|---|---|
| `audit_source()` | Scans source for forbidden patterns (dlopen, libloading, PluginManager, etc.) |
| `audit_files()` | Batch audit across multiple files |
| `check_dependencies()` | Verifies no forbidden crate dependencies |
| `verify_architecture_rule()` | Validates architecture description matches no-plugin rule |

## Related

- Commands: [/docs/reference/CONFORMANCE_COMMANDS.md](/docs/reference/CONFORMANCE_COMMANDS.md)
- Testing infrastructure: [/docs/reference/CONFORMANCE_TESTING_INFRA.md](/docs/reference/CONFORMANCE_TESTING_INFRA.md)
