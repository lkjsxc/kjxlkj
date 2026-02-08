# Vimdiff Equivalent

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Side-by-side file comparison with merge capabilities.

## Entry (normative)

| Command | Action |
|---|---|
| `:diffsplit {file}` | Open `{file}` in a horizontal split with diff mode enabled |
| `:vert diffsplit {file}` | Open `{file}` in a vertical split with diff mode |
| `:diffthis` | Enable diff mode in the current window |
| `:diffoff` | Disable diff mode in the current window |
| `:diffoff!` | Disable diff mode in all windows |
| `:diffupdate` | Recalculate diff |

## Diff display (normative)

In diff mode, two or more windows show the same region of their respective files:

| Element | Rendering |
|---|---|
| Added lines | Highlighted with `DiffAdd` group |
| Deleted lines | Highlighted with `DiffDelete` group; filler lines shown in the other window |
| Changed lines | Highlighted with `DiffChange` group; changed text within the line uses `DiffText` |
| Unchanged regions | Folded automatically when more than `diffopt` context lines apart |

Windows in diff mode MUST scroll synchronously (`scrollbind` and `cursorbind` are set automatically).

## Navigation (normative)

| Key | Action |
|---|---|
| `]c` | Jump to the next change (diff hunk) |
| `[c` | Jump to the previous change |

## Merge commands (normative)

| Command | Action |
|---|---|
| `:diffget` / `do` | Obtain the change from the other window into the current window |
| `:diffput` / `dp` | Put the change from the current window into the other window |

With a range: `:'<,'>diffget` applies only to the selected lines.

## Related

- Git integration: [/docs/spec/features/git/git.md](/docs/spec/features/git/git.md)
- Diff mode: [/docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)
- Gitsigns: [/docs/spec/features/git/gitsigns.md](/docs/spec/features/git/gitsigns.md)
