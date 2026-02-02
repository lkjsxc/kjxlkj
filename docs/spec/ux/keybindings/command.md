# Keybindings: Command Mode

Complete nvim-compatible Command-line mode keybindings.

## Entering Command Mode

| Key | Action | Description |
|-----|--------|-------------|
| `:` | Ex command | Enter command-line mode |
| `/` | Search forward | Enter forward search |
| `?` | Search backward | Enter backward search |
| `q:` | Command history | Open command window |
| `q/` | Search history | Open search history |

## Command Line Editing

| Key | Action | Description |
|-----|--------|-------------|
| `Backspace` | Delete char | Delete character before cursor |
| `Ctrl-h` | Delete char | Same as Backspace |
| `Ctrl-w` | Delete word | Delete word before cursor |
| `Ctrl-u` | Delete to start | Delete to start of line |
| `Ctrl-b` | Cursor start | Move to start of line |
| `Ctrl-e` | Cursor end | Move to end of line |
| `Left` | Cursor left | Move cursor left |
| `Right` | Cursor right | Move cursor right |
| `Ctrl-Left` | Word left | Move word left |
| `Ctrl-Right` | Word right | Move word right |

## Command Line History

| Key | Action | Description |
|-----|--------|-------------|
| `Up` | Previous | Previous command in history |
| `Down` | Next | Next command in history |
| `Ctrl-p` | Previous | Same as Up |
| `Ctrl-n` | Next | Same as Down |

## Completion

| Key | Action | Description |
|-----|--------|-------------|
| `Tab` | Complete | Complete command/path |
| `Ctrl-d` | List matches | Show completion list |
| `Ctrl-l` | Complete longest | Complete longest common |

## Insert Special

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-r <reg>` | Insert register | Insert register contents |
| `Ctrl-r Ctrl-w` | Word | Insert word under cursor |
| `Ctrl-r Ctrl-a` | WORD | Insert WORD under cursor |
| `Ctrl-r Ctrl-f` | File | Insert filename under cursor |
| `Ctrl-r %` | Current file | Insert current filename |

## Exiting Command Mode

| Key | Action | Description |
|-----|--------|-------------|
| `Enter` | Execute | Execute command |
| `Esc` | Cancel | Cancel and return to Normal |
| `Ctrl-c` | Cancel | Same as Esc |
| `Ctrl-[` | Cancel | Same as Esc |

## Common Ex Commands

| Command | Action |
|---------|--------|
| `:w` | Write buffer |
| `:q` | Quit window |
| `:wq` / `:x` | Write and quit |
| `:q!` | Force quit |
| `:qa` | Quit all |
| `:wa` | Write all |
| `:e <file>` | Edit file |
| `:sp` / `:split` | Horizontal split |
| `:vs` / `:vsplit` | Vertical split |
| `:bn` / `:bnext` | Next buffer |
| `:bp` / `:bprev` | Previous buffer |
| `:bd` | Delete buffer |
| `:<N>` | Go to line N |
| `:%s/old/new/g` | Substitute all |
| `:noh` | Clear highlights |
| `:set <opt>` | Set option |

## Range Specifiers

| Specifier | Meaning |
|-----------|---------|
| `.` | Current line |
| `$` | Last line |
| `%` | Entire file (1,$) |
| `'<,'>` | Visual selection |
| `N` | Line number N |
| `+N` | N lines down |
| `-N` | N lines up |
| `/pattern/` | Next match |
| `?pattern?` | Previous match |

