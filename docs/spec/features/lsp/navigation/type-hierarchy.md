# Type Hierarchy

View type inheritance and implementations.

## Overview

Type hierarchy shows supertypes (parents) and
subtypes (children) of a type.

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `<leader>ts` | Type supertypes |
| `<leader>tt` | Type subtypes |

### Command


## Supertypes

### Display


### Meaning

Traits implemented, parent classes.

## Subtypes

### Display


### Meaning

Implementing types, child classes.

## Navigation

| Key | Action |
|-----|--------|
| `j` | Move down |
| `k` | Move up |
| `<CR>` | Jump to type |
| `l` | Expand |
| `h` | Collapse |

## Language Support

### Rust

- Trait implementations
- Struct relationships

### TypeScript

- Class inheritance
- Interface implementations

### Go

- Interface implementations

## Configuration


## LSP Requirements

### Server Support

| Server | Type Hierarchy |
|--------|----------------|
| rust-analyzer | ✓ |
| typescript | ✓ |
| clangd | ✓ |

## Use Cases

### Understanding

See how types relate.

### Refactoring

Find all implementations.

### Navigation

Jump between related types.

## Display Options


## Icons

| Icon | Type |
|------|------|
| ◇ | Class |
| ⬡ | Interface |
| △ | Trait |
| ▢ | Enum |
| ○ | Struct |

## Preview

### On Select

Shows type definition:


## Tips

1. Use supertypes to find interfaces
2. Use subtypes for implementations
3. Navigate complex inheritance
4. Find all implementors

## Keybindings

