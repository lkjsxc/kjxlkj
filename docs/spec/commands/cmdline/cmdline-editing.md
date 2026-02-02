# Command-Line Editing

Keys for editing in command-line mode.

## Overview

Command-line mode supports
comprehensive editing keys.

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

## History Navigation

### Basic

| Key      | Action              |
|----------|---------------------|
| `<Up>`   | Previous history    |
| `<Down>` | Next history        |
| `<C-p>`  | Previous history    |
| `<C-n>`  | Next history        |

### Filtered

Typing prefix, then arrows
filters to matching history.

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


## Expression Evaluation

### Expression Register


### Examples


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

