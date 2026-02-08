# Essential commands
This is the target minimum Ex command surface.

For the currently supported subset (when a reconstructed implementation exists), see [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md).

## File commands (normative)

| Command | Description | Notes |
|---|---|---|
| `:e[dit] {file}` | Open file in current window | Creates buffer if not loaded |
| `:e[dit]!` | Re-read current file, discarding changes | Forces reload |
| `:w[rite]` | Write current buffer to its file | Error if no filename |
| `:w[rite] {file}` | Write current buffer to `{file}` | Does not change buffer filename |
| `:w[rite]!` | Force write (overrides read-only) | |
| `:wa[ll]` | Write all modified buffers | Skips unnamed buffers |
| `:sav[eas] {file}` | Save as new file, switch buffer to it | |
| `:q[uit]` | Close current window | Error if last window with unsaved changes |
| `:q[uit]!` | Close current window, discard changes | |
| `:wq` | Write and close | |
| `:x[it]` | Write (if modified) and close | Like `:wq` but skips write if unmodified |
| `:qa[ll]` | Close all windows and exit | Error if unsaved changes |
| `:qa[ll]!` | Close all and exit, discard changes | |
| `:cq` | Quit with error exit code | For use with git rebase, etc. |

## Buffer commands (normative)

| Command | Description |
|---|---|
| `:ls` / `:buffers` | List all buffers with status flags |
| `:b[uffer] {n}` | Switch to buffer number `{n}` |
| `:b[uffer] {name}` | Switch to buffer matching `{name}` (partial match) |
| `:bn[ext]` | Switch to next buffer |
| `:bp[revious]` | Switch to previous buffer |
| `:bf[irst]` | Switch to first buffer |
| `:bl[ast]` | Switch to last buffer |
| `:bd[elete]` | Delete (unload) current buffer |
| `:bd[elete]!` | Force delete buffer, discard changes |
| `:bw[ipeout]` | Wipe buffer completely (remove from buffer list) |

## Window commands (normative)

| Command | Description |
|---|---|
| `:sp[lit]` | Split window horizontally |
| `:sp[lit] {file}` | Split and open file |
| `:vsp[lit]` | Split window vertically |
| `:vsp[lit] {file}` | Split vertically and open file |
| `:new` | New horizontal split with empty buffer |
| `:vnew` | New vertical split with empty buffer |
| `:clo[se]` | Close current window |
| `:on[ly]` | Close all other windows |
| `:resize {n}` | Set window height to `{n}` rows |
| `:vertical resize {n}` | Set window width to `{n}` columns |

## Tab commands (normative)

| Command | Description |
|---|---|
| `:tabnew` | Open new tab with empty buffer |
| `:tabnew {file}` | Open new tab with file |
| `:tabc[lose]` | Close current tab |
| `:tabo[nly]` | Close all other tabs |
| `:tabn[ext]` | Go to next tab |
| `:tabp[revious]` | Go to previous tab |
| `gt` | Go to next tab (Normal mode) |
| `gT` | Go to previous tab (Normal mode) |

## Option commands (normative)

| Command | Description |
|---|---|
| `:set {opt}` | Set boolean option to true |
| `:set no{opt}` | Set boolean option to false |
| `:set {opt}!` | Toggle boolean option |
| `:set {opt}?` | Show current value |
| `:set {opt}={val}` | Set option to value |
| `:set {opt}+=` | Append to string option or add to number |
| `:set {opt}-=` | Remove from string option or subtract from number |
| `:setlocal {opt}` | Set option locally for current buffer/window |

## External commands (normative)

| Command | Description |
|---|---|
| `:! {cmd}` | Execute shell command via terminal service |
| `:[range]! {cmd}` | Filter range through shell command |
| `:r[ead] !{cmd}` | Read command output below current line |
| `:terminal` | Open embedded terminal |

## Search/substitute commands

See [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md) for `:s`, `:g`, `:v` and related.

## Related

- Command syntax: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
- Ranges: [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
- Conformance reference: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
