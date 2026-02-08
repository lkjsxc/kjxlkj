# Command-Line Editing

Keys for editing in command-line mode (`:`, `/`, `?` prompts).

## Cursor Movement

### Basic

| Key          | Action             |
|--------------|-------------------|
| `<Left>`     | Move left one char |
| `<Right>`    | Move right one char|
| `<Home>`     | Start of line      |
| `<End>`      | End of line        |

### Word Movement

| Key          | Action             |
|--------------|-------------------|
| `<S-Left>`   | Move word left     |
| `<S-Right>`  | Move word right    |
| `<C-Left>`   | Move word left     |
| `<C-Right>`  | Move word right    |

### Alternative Keys

| Key      | Action       |
|----------|-------------|
| `<C-b>`  | Beginning   |
| `<C-e>`  | End         |

## Text Deletion

### Character

| Key      | Action               |
|----------|---------------------|
| `<BS>`   | Delete char left    |
| `<Del>`  | Delete char right   |
| `<C-h>`  | Delete char left    |

### Word

| Key      | Action               |
|----------|---------------------|
| `<C-w>`  | Delete word left    |

### Line

| Key      | Action               |
|----------|---------------------|
| `<C-u>`  | Delete to start     |
| `<C-k>`  | Delete to end       |

## Text Insertion

### Register Insert

| Key         | Action              |
|-------------|---------------------|
| `<C-r>{reg}`| Insert register     |
| `<C-r><C-r>`| Insert literally    |
| `<C-r><C-o>`| Insert literally    |
| `<C-r><C-w>`| Word under cursor   |
| `<C-r><C-a>`| WORD under cursor   |
| `<C-r><C-f>`| File under cursor   |
| `<C-r><C-p>`| Expanded file path  |
| `<C-r><C-l>`| Line under cursor   |

### Special Registers

| Sequence | Inserts |
|----------|---------|
| `<C-r>"` | Default (unnamed) register |
| `<C-r>0` | Yank register |
| `<C-r>+` | System clipboard |
| `<C-r>/` | Last search pattern |
| `<C-r>%` | Current filename |
| `<C-r>=` | Expression result (prompts for expression) |

## Completion

### Tab Completion

| Key        | Action              |
|------------|---------------------|
| `<Tab>`    | Complete forward    |
| `<S-Tab>`  | Complete backward   |
| `<C-d>`    | List completions    |
| `<C-l>`    | Complete longest    |

### Wildmenu

Navigate completion menu:

| Key | Action |
|-----|--------|
| `<Tab>` | Next match in wildmenu |
| `<S-Tab>` | Previous match in wildmenu |
| `<Left>` | Previous match (horizontal layout) |
| `<Right>` | Next match (horizontal layout) |
| `<Up>` | Go up a directory level (file completion) |
| `<Down>` | Enter directory / accept match |
| `<C-y>` | Accept currently highlighted match |
| `<C-e>` | Dismiss wildmenu, restore original text |

## History Navigation

### Basic

| Key      | Action              |
|----------|---------------------|
| `<Up>`   | Previous history    |
| `<Down>` | Next history        |
| `<C-p>`  | Previous history    |
| `<C-n>`  | Next history        |

### Filtered

Typing prefix then arrows filters to matching history.

## Special Keys

### Execute/Cancel

| Key      | Action         |
|----------|---------------|
| `<CR>`   | Execute        |
| `<Esc>`  | Cancel         |
| `<C-c>`  | Cancel         |
| `<C-[>`  | Cancel (alt)   |

### Mode Switch

| Key      | Action               |
|----------|---------------------|
| `<C-o>`  | Execute & return    |
| `<C-g>`  | Normal mode in cmd  |

## File/Path Expansion

### Tokens

| Token    | Expansion          |
|----------|-------------------|
| `%`      | Current file       |
| `#`      | Alternate file     |
| `#n`     | Buffer n           |
| `##`     | All buffers        |

### Modifiers

| Modifier | Effect             |
|----------|-------------------|
| `:p`     | Full path          |
| `:h`     | Head (directory)   |
| `:t`     | Tail (filename)    |
| `:r`     | Root (no extension)|
| `:e`     | Extension only     |
| `:s`     | Substitute         |

### Example

`:e %:h/other.rs` -- edit `other.rs` in same directory as current file.
`:w #` -- save to the alternate file path.

## Expression Evaluation

### Expression Register

`<C-r>=` opens an expression prompt (`=`). Type a Lua expression and press
`<CR>` to insert the result as text into the command line.

### Examples

- `<C-r>=2+2<CR>` -- inserts `4`
- `<C-r>=expand("%:t")<CR>` -- inserts current filename (tail)
- `<C-r>=line(".")<CR>` -- inserts current line number

## Literal Insert

### Special Characters

| Key         | Action              |
|-------------|---------------------|
| `<C-v>`     | Insert next literally|
| `<C-q>`     | Insert next literally|
| `<C-v>xxx`  | Insert decimal      |
| `<C-v>xNN`  | Insert hex          |
| `<C-k>{ab}` | Insert digraph      |

## Paste Operations

### Register Paste

`<C-r>{reg}` pastes register contents into the command line at cursor position:
- `<C-r>"` -- paste default register (last delete/yank)
- `<C-r>a` -- paste named register `a`; `<C-r>+` -- paste system clipboard
- `<C-r><C-w>` -- paste word under cursor (useful for search-and-replace)
- `<C-r><C-r>{reg}` -- paste literally (no special character interpretation)
