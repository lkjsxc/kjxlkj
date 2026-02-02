# Implementation Finder

Find all implementations of a type or trait.

## Overview

Quickly navigate to implementations of interfaces,
traits, or abstract types.

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `gi` | Go to implementation |

### Command


## Display

### Multiple Implementations


### Single Implementation

Jumps directly to the implementation.

## Navigation

### From Interface/Trait


Press `gi` to see all implementations.

### From Abstract Method


Press `gi` to see concrete implementations.

## Configuration


## Actions

| Key | Action |
|-----|--------|
| `<CR>` | Jump to implementation |
| `<C-v>` | Open in vsplit |
| `<C-x>` | Open in split |
| `<C-t>` | Open in new tab |
| `<Esc>` | Cancel |

## LSP Requirements

### Server Support

| Server | Implementation |
|--------|----------------|
| rust-analyzer | ✓ |
| typescript | ✓ |
| gopls | ✓ |
| clangd | ✓ |

## Use Cases

### Interface Navigation

Find all concrete implementations.

### Polymorphism

Understand runtime behavior.

### Refactoring

See affected implementations.

## Related Commands

| Command | Description |
|---------|-------------|
| `gd` | Go to definition |
| `gD` | Go to declaration |
| `gi` | Go to implementation |
| `gr` | Go to references |

## Difference from Definition

### Definition (`gd`)

Goes to where symbol is defined.

### Implementation (`gi`)

Goes to concrete implementations.

## Preview

### Inline Preview


Shows implementation code before jumping.

## Filtering

### By Type

When multiple kinds exist:


## Keybindings


## Tips

1. Use on traits to find implementors
2. Jump from abstract to concrete
3. Use preview before jumping
4. Combine with references for full picture
