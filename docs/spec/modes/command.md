# Command mode

Command mode is the ex-style command-line interface activated from Normal mode.
It handles ex commands (`:`) and search patterns (`/`, `?`).

## Requirements

- Parsing is core-owned and deterministic.
- Executing a command produces typed intents.
- IO-heavy commands delegate to services and surface progress/cancellation.

## Command-line state

- Entering command mode MUST present a fresh prompt with an empty input buffer.
- The command-line cursor starts at position 0.
- The previous command text MUST NOT be implicitly reused or appended-to.

## Entry points

| Key | Effect |
|-----|--------|
| `:` | Open ex command prompt |
| `/` | Open forward-search prompt |
| `?` | Open backward-search prompt |

## Command-line editing keys

| Key | Effect |
|-----|--------|
| Left / Right | Move cursor one character |
| Ctrl-b | Move cursor to beginning of line |
| Ctrl-e | Move cursor to end of line |
| Ctrl-w | Delete word backward |
| Ctrl-u | Delete from cursor to start of line |
| Ctrl-r {reg} | Insert contents of register {reg} |
| Ctrl-r Ctrl-w | Insert word under cursor from document |
| Ctrl-r Ctrl-a | Insert WORD under cursor from document |
| Backspace | Delete character before cursor |
| Delete | Delete character under cursor |

The command-line input buffer is a single-line editor with its own cursor
independent of the document cursor.

## History navigation

| Key | Effect |
|-----|--------|
| Up / Ctrl-p | Previous history entry (filtered by current prefix) |
| Down / Ctrl-n | Next history entry (filtered by current prefix) |
| Ctrl-f | Open command-line window for the current prompt type |
| `q:` | Open command-line window with ex history (from Normal mode) |
| `q/` | Open command-line window with forward-search history |
| `q?` | Open command-line window with backward-search history |

- History is maintained per prompt type: ex commands and search patterns
  each have separate history lists.
- Prefix filtering means typing `:set` then pressing Up cycles only through
  history entries that begin with `set`.

## Command-line window

The command-line window is a special buffer that displays the history list
for a given prompt type.

- It opens as a normal editing buffer; standard motions and operators apply.
- The user MAY modify any history entry in the buffer.
- Pressing Enter on a line executes that line as a command or search.
- Pressing Ctrl-c or executing `:quit` closes the window and returns to
  Normal mode without executing anything.
- The window uses the same buffer and window mechanics as regular editing.

## Completion

| Key | Effect |
|-----|--------|
| Tab | Complete forward (next candidate) |
| Shift-Tab | Complete backward (previous candidate) |
| Ctrl-d | List all matching completions without inserting |

Completion sources (in priority order where applicable):

1. Command names
2. Command options and option values
3. File paths (provided by FS service)
4. Buffer names
5. Help tags
6. User-defined functions
7. Plugin-registered sources

- The wildmenu, when enabled, renders a horizontal menu bar above the
  command line showing completion candidates.
- Completion MUST NOT mutate editor state or execute side effects.
- Completion sources MAY be asynchronous; results are versioned and
  cancellable.

## Range syntax

Ranges select line spans for ex commands such as `:substitute` and `:global`.

| Specifier | Meaning |
|-----------|---------|
| `.` | Current line |
| `$` | Last line in buffer |
| `%` | Entire file (equivalent to `1,$`) |
| `N` | Absolute line number N |
| `'a` | Line of mark a |
| `'<` / `'>` | Start / end of last visual selection |
| `/pattern/` | Next line matching pattern |
| `?pattern?` | Previous line matching pattern |
| `+N` / `-N` | Relative offset from preceding address |

- Ranges compose by comma: `.,$` means current line to last line;
  `'a,'b` means from mark a to mark b.
- An omitted range defaults to the current line for most commands.
- See [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
  for full range evaluation semantics.

## Special keys

| Key | Effect |
|-----|--------|
| Enter | Execute the command or initiate the search |
| Esc | Cancel input and return to Normal mode |
| Ctrl-c | Cancel input and return to Normal mode |

Esc and Ctrl-c MUST discard any partially typed input and restore the
previous mode without side effects.

## Search specifics

- `/pattern` searches forward from the cursor; `?pattern` searches backward.
- After a search, `n` repeats in the same direction and `N` reverses.
- The `hlsearch` option, when enabled, highlights all matches in the buffer.
- The `incsearch` option, when enabled, highlights matches incrementally
  as the user types the pattern.
- Search patterns use Vim-compatible regex (magic mode by default).
- An empty pattern repeats the most recent search.
- See [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
  for the finder/search subsystem specification.

## Substitution

The `:substitute` command performs pattern-based text replacement.

| Form | Scope |
|------|-------|
| `:s/pattern/replacement/flags` | Current line only |
| `:%s/pattern/replacement/flags` | Entire file |
| `:'<,'>s/pattern/replacement/flags` | Visual selection |

Common flags:

| Flag | Meaning |
|------|---------|
| `g` | Replace all occurrences on each line (not just the first) |
| `c` | Prompt for confirmation before each replacement |
| `i` | Case-insensitive matching |
| `n` | Count matches without replacing |

See [/docs/spec/commands/substitute/substitute-command.md](/docs/spec/commands/substitute/substitute-command.md)
for the full substitution specification.

## Invariants

1. The command-line input buffer is independent from document buffers;
   editing the command line MUST NOT modify any document.
2. History persists across sessions via the session file.
3. Command parsing is deterministic and core-owned; no plugin may override
   the parser.
4. Tab completion MUST NOT execute side effects or mutate editor state.
5. Cancelling command mode (Esc or Ctrl-c) MUST return to Normal mode
   with no observable state change.

## Related

- Command system overview: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)
- Command-line interface specs: [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)
- Command completion: [/docs/spec/commands/cmdline/completion.md](/docs/spec/commands/cmdline/completion.md)
- Range evaluation: [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
- Substitution command: [/docs/spec/commands/substitute/substitute-command.md](/docs/spec/commands/substitute/substitute-command.md)
- Search and finder UI: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
