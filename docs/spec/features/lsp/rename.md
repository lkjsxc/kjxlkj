# Rename Refactoring

Rename symbols across the codebase.

## Overview

LSP-powered rename refactoring updates all references
to a symbol throughout the project.

## Usage

### Keybinding

| Key | Action |
|-----|--------|
| `<leader>rn` | Rename symbol |

### Command


## Interactive Rename

### Workflow

1. Place cursor on symbol
2. Press `<leader>rn`
3. Type new name
4. Press `<Enter>` to apply

### Display


## Preview

### Before Apply


### Preview Display


## Configuration


## Prepare Rename

### Validation

LSP validates rename before starting:
- Symbol is renameable
- Valid identifier
- No conflicts

### Error


## Scope

### Project-Wide

Renames across all project files.

### File-Specific

Some renames may be file-local.

## Undo

### Single Undo

All changes undone with single `u`.

### Per-File Undo

Each file can be undone independently.

## LSP Requirements

### Server Support

| Server | Rename |
|--------|--------|
| rust-analyzer | ✓ |
| typescript | ✓ |
| gopls | ✓ |
| clangd | ✓ |

## Special Cases

### File Rename

Some servers support renaming files:


### Module Rename

Automatically updates imports.

## Conflicts

### Detection


### Resolution

Choose different name.

## Tips

1. Preview changes before applying
2. Check all files affected
3. Use undo if needed
4. Commit before large renames

## Keybindings


## Commands

