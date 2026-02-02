# Call Hierarchy

View incoming and outgoing function calls.

## Overview

Call hierarchy shows which functions call a function
(incoming) and which functions it calls (outgoing).

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `<leader>ci` | Incoming calls |
| `<leader>co` | Outgoing calls |

### Command


## Incoming Calls

### Display


### Meaning

Who calls this function.

## Outgoing Calls

### Display


### Meaning

What functions this function calls.

## Navigation

### Tree Navigation

| Key | Action |
|-----|--------|
| `j` | Move down |
| `k` | Move up |
| `<CR>` | Jump to location |
| `l` | Expand |
| `h` | Collapse |
| `<Tab>` | Toggle expand |

### Actions

| Key | Action |
|-----|--------|
| `<CR>` | Jump |
| `o` | Jump and close |
| `v` | Open in vsplit |
| `s` | Open in split |

## Depth

### Expand Levels


### Lazy Loading

Children loaded on expand.

## Configuration


## LSP Requirements

### Server Support

| Server | Call Hierarchy |
|--------|----------------|
| rust-analyzer | ✓ |
| clangd | ✓ |
| typescript | ✓ |
| gopls | ✓ |

## Use Cases

### Understanding Code

See how functions connect.

### Refactoring

Find all callers before changing.

### Debugging

Trace call paths.

## Display Options


## Preview

### On Select

Shows code context:


## Tips

1. Use incoming to find callers
2. Use outgoing to understand behavior
3. Expand gradually for deep trees
4. Jump to see context

## Keybindings

