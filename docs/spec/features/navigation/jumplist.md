# Jump List

Navigate through cursor position history.

## Navigation (normative)

| Key | Action |
|---|---|
| `<C-o>` | Jump to older position |
| `<C-i>` / `<Tab>` | Jump to newer position |

## What Creates Jumps (normative)

Actions that add entries to the jump list.

### Jump-creating movements

| Movement | Jump? |
|---|---|
| `G`, `gg`, `{count}G` | Yes |
| `/`, `?`, `n`, `N` | Yes |
| `%` (bracket match) | Yes |
| `(`, `)`, `{`, `}` | Yes |
| `[[`, `]]`, `[]`, `][` | Yes |
| `H`, `M`, `L` | Yes |
| `:{number}` (ex line) | Yes |
| `'mark`, `` `mark `` | Yes |
| `<C-]>` (tag jump) | Yes |

### Non-jump movements

| Movement | Jump? |
|---|---|
| `h`, `j`, `k`, `l` | No |
| `w`, `b`, `e`, `W`, `B`, `E` | No |
| `f`, `t`, `F`, `T` | No |
| `^`, `$`, `0` | No |

## Viewing Jump List

`:jumps` shows the jumplist with position index, line, column, and file/text.

## Cross-File Jumps

The jumplist spans across files. `<C-o>` may switch to a previously visited buffer. The file is reopened if it was closed.

## Jump List Size

Default capacity: 100 entries. Oldest entries are dropped when the limit is exceeded.

## Clearing

`:clearjumps` clears the entire jumplist.

## Jump List vs Change List

| Feature | Jump List | Change List |
|---|---|---|
| Tracks | Large cursor movements | Edit positions |
| Trigger | G, /, ?, n, marks, etc. | Any buffer modification |
| Navigate | `<C-o>` / `<C-i>` | `g;` / `g,` |

## Persistence

The jumplist is saved in the session file and restored on next editor start.

## Related

- Change list: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Marks: [/docs/spec/editing/marks/jump-marks.md](/docs/spec/editing/marks/jump-marks.md)
