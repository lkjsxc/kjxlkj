# Keybinding DSL

Domain-specific language for keybindings.

## Overview

kjxlkj uses a powerful DSL for defining keybindings
with conditions, sequences, and actions.

## Basic Syntax

A binding maps a key (or key sequence) to an action:

| Syntax | Meaning |
|--------|---------|
| `map j cursor.down` | `j` moves cursor down |
| `map <C-s> file.save` | Ctrl+s saves file |
| `map <S-Tab> indent.left` | Shift+Tab unindents |
| `map <CR> line.open_below` | Enter opens line below |

## Key Notation

### Modifiers

| Notation | Meaning |
|----------|---------|
| `<C-x>` | Ctrl + x |
| `<S-x>` | Shift + x |
| `<A-x>` | Alt + x |
| `<M-x>` | Meta + x |
| `<D-x>` | Cmd (Mac) |

### Special Keys

| Notation | Key |
|----------|-----|
| `<CR>` | Enter |
| `<Esc>` | Escape |
| `<Tab>` | Tab |
| `<BS>` | Backspace |
| `<Space>` | Space |
| `<Up>` `<Down>` `<Left>` `<Right>` | Arrow keys |
| `<F1>`-`<F12>` | Function keys |

## Sequences

### Multi-Key

Key sequences are consecutive keys. The editor waits for the full
sequence before dispatching (controlled by `timeoutlen`).

| Sequence | Meaning |
|----------|---------|
| `gc` | Toggle comment (operator) |
| `gq` | Format text (operator) |
| `gg` | Go to first line |
| `dd` | Delete line |
| `<Leader>ff` | Leader, then `f`, then `f` |

### Leader Key

`<Leader>` is a placeholder resolved at runtime. Default is `\`.

```
set leader=<Space>
map <Leader>w file.save
map <Leader>ff fuzzy.files
```

## Conditional Bindings

### Mode-Specific

Restrict a binding to a mode with mode-prefixed commands:

| Command | Mode | Example |
|---------|------|---------|
| `nmap` | Normal | `nmap j cursor.down` |
| `imap` | Insert | `imap <C-h> char.delete_left` |
| `vmap` | Visual | `vmap d selection.delete` |
| `cmap` | Command | `cmap <C-a> line.home` |
| `tmap` | Terminal | `tmap <Esc> terminal.exit` |

### Context-Based

Restrict bindings by filetype or buffer property:

```
map <Leader>r run.file { filetype = "rust" }
map <Leader>r run.file { filetype = "python" }
map <C-b> build.project { path_glob = "*.go" }
```

## Actions

### Simple Action

Bind a key to an editor command by name:

```
map <C-s> file.save
map u edit.undo
map <C-r> edit.redo
```

### With Arguments

Pass arguments in parentheses after the action:

```
map <Leader>1 buffer.goto(1)
map <C-d> scroll.lines(10)
map <Leader>e split.open("vertical")
```

### Command

Bind to an Ex command string with `:`:

```
map <Leader>s :sort<CR>
map <Leader>h :nohlsearch<CR>
```

### Lua (Future)

Planned scripting hook for complex logic:

```
-- not yet implemented; planned syntax:
map <Leader>x lua(function()
  local line = editor.current_line()
  if line:match("TODO") then editor.execute("comment.toggle") end
end)
```

## Unbinding

Remove a binding with `unmap`:

```
unmap j
nunmap gc        -- mode-specific unbind
```

## Remapping

### Recursive

`map` is recursive -- the target is re-evaluated through the map chain:

```
map j gj            -- j triggers gj
map gj scroll.down   -- gj triggers scroll.down (j -> scroll.down)
```

### Non-Recursive

`noremap` prevents recursive expansion:

```
noremap j gj     -- j sends literal gj, no further expansion
nnoremap k gk
inoremap jk <Esc>
```

## Which-Key Integration

After pressing a prefix key, a popup shows available continuations:

| Setting | Default | Meaning |
|---------|---------|---------|
| `whichkey.enable` | `true` | Show which-key popup |
| `whichkey.delay` | `300` | Milliseconds before popup |
| `whichkey.sort` | `"alpha"` | Sort order (`alpha` or `order`) |

Register descriptions: `map <Leader>ff fuzzy.files { desc = "Find files" }`

## Examples

### Complete Config

```
set leader=<Space>
set timeoutlen=400

nnoremap j gj
nnoremap k gk
nnoremap <C-d> scroll.half_down
nnoremap <C-u> scroll.half_up
nmap <Leader>w file.save
nmap <Leader>q buffer.close
nmap <Leader>ff fuzzy.files
nmap <Leader>fg fuzzy.grep
nmap <C-h> window.focus_left
inoremap jk <Esc>
vmap < indent.left
vmap > indent.right
```
