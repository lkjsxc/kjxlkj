# Command-line completion
Completion is a UI layer over deterministic parsing.

## Requirements
- Completion does not mutate editor state.
- Completion sources may be async (filesystem, LSP, commands), but results are versioned and cancellable.

## Sources (normative)

| Source | Trigger context | Examples |
|---|---|---|
| Command names | After `:` with no space yet | `:wri` completes to `:write` |
| Sub-commands | After a command that takes sub-commands | `:syntax ` completes `on`, `off`, `reset` |
| Options | After `:set ` | `:set number`, `:set tabstop=` |
| Option values | After `=` in `:set opt=` | `:set filetype=` offers known filetypes |
| File paths | After commands that take file arguments | `:e src/` lists directory contents |
| Buffer names | After `:buffer ` or `:b ` | `:b main` matches buffer names |
| Help tags | After `:help ` | `:help motion` |
| Color schemes | After `:colorscheme ` | `:colorscheme dark` |
| Variables | After `:let ` or in expressions | Variable names |

## Completion keys (normative)

| Key | Action |
|---|---|
| `Tab` | Complete to next match (or first match if no completion active) |
| `Shift-Tab` | Complete to previous match |
| `Ctrl-d` | List all possible completions without completing |
| `Ctrl-l` | Complete to longest common prefix |
| `Ctrl-n` | Next match (same as Tab) |
| `Ctrl-p` | Previous match (same as Shift-Tab) |

## Wildmenu (normative)

When multiple completions match, a horizontal menu MUST be displayed in the statusline area showing available options. The currently selected option MUST be highlighted. Configuration:

| Setting | Default | Description |
|---|---|---|
| `wildmenu` | true | Show completion menu |
| `wildmode` | `full` | Completion behavior (`full`, `longest`, `list`, `lastused`) |
| `wildignorecase` | false | Case-insensitive completion matching |

## Filesystem completion detail (normative)

- Paths are resolved relative to the current working directory.
- The FS service provides directory listings asynchronously.
- Results MUST be delivered within a timeout; stale results are discarded.
- Hidden files (starting with `.`) are included only when the user has typed a `.` prefix.

## Related

- Finder command palette: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- Command parsing: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
