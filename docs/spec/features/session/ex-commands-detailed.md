# Ex Commands Detailed

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Complete reference for ex commands related to session and buffer management.

## Buffer commands

| Command | Description |
|---|---|
| `:e[dit] {file}` | Open file in current window |
| `:e[dit]!` | Reload current file, discarding changes |
| `:ene[w]` | Create new empty buffer |
| `:b[uffer] {n}` | Switch to buffer number `{n}` |
| `:b[uffer] {name}` | Switch to buffer matching `{name}` |
| `:bn[ext]` | Switch to next buffer |
| `:bp[rev]` | Switch to previous buffer |
| `:bf[irst]` | Switch to first buffer |
| `:bl[ast]` | Switch to last buffer |
| `:bd[elete] [n]` | Delete buffer `{n}` (or current) |
| `:bw[ipeout] [n]` | Wipe buffer `{n}` (remove completely) |
| `:ls` / `:buffers` | List all buffers |

## File commands

| Command | Description |
|---|---|
| `:w[rite]` | Write current buffer to file |
| `:w[rite] {file}` | Write current buffer to `{file}` |
| `:w[rite]!` | Force write (override readonly) |
| `:wa[ll]` | Write all modified buffers |
| `:sav[eas] {file}` | Save to new file and switch to it |
| `:r[ead] {file}` | Read file contents below cursor |
| `:r[ead] !{cmd}` | Read command output below cursor |

## Quit commands

| Command | Description |
|---|---|
| `:q[uit]` | Close current window |
| `:q[uit]!` | Force close (discard changes) |
| `:qa[ll]` | Quit all windows |
| `:qa[ll]!` | Force quit all |
| `:wq` | Write and quit |
| `:wqa[ll]` / `:xa[ll]` | Write all and quit |
| `:x[it]` | Write if modified, then quit |
| `:cq` | Quit with error code (for git rebase abort) |

## Window commands

| Command | Description |
|---|---|
| `:sp[lit] [file]` | Horizontal split (optionally with file) |
| `:vs[plit] [file]` | Vertical split (optionally with file) |
| `:new` | New horizontal split with empty buffer |
| `:vnew` | New vertical split with empty buffer |
| `:clo[se]` | Close current window |
| `:on[ly]` | Close all windows except current |

## Tab commands

| Command | Description |
|---|---|
| `:tabe[dit] [file]` | Open file in new tab |
| `:tabn[ext]` | Switch to next tab |
| `:tabp[rev]` | Switch to previous tab |
| `:tabc[lose]` | Close current tab |
| `:tabo[nly]` | Close all tabs except current |
| `:tabm[ove] {n}` | Move current tab to position `{n}` |

## Session commands

| Command | Description |
|---|---|
| `:SessionSave [name]` | Save session to JSON |
| `:SessionLoad [name]` | Load session from JSON |
| `:SessionDelete [name]` | Delete saved session |
| `:SessionNew` | Start fresh session |
| `:mksession {file}` | Write session file to specific path |

## Option commands

| Command | Description |
|---|---|
| `:set {option}={value}` | Set option value |
| `:set {option}?` | Query option value |
| `:set {option}` | Enable boolean option |
| `:set no{option}` | Disable boolean option |
| `:set {option}!` | Toggle boolean option |
| `:setlocal {option}={value}` | Set buffer/window-local option |

## Related

- Command syntax: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
- Essential commands: [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)
- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
