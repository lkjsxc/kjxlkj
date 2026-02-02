# Workspaces

Multi-folder workspace support and management.

## Overview

Workspaces allow opening multiple folders as a
single editing environment.

## Creating Workspaces

### Open Multiple Folders


### Workspace File


## Workspace File Format


## File Explorer

### Multi-Root View


## Navigation

### Keybindings

| Key | Action |
|-----|--------|
| `<leader>1-9` | Jump to folder 1-9 |
| `<leader>wp` | Workspace picker |

### Finder


## Per-Folder Settings


## LSP Integration

### Separate Instances

Each folder can have its own LSP:


## Git Integration

### Multiple Repositories

Each folder maintains separate git state:

- Independent gitsigns
- Separate branch display
- Per-repo operations

## Search

### Workspace-Wide Search


### Finder Scope


## Workspace Commands

| Command | Action |
|---------|--------|
| `:WorkspaceAdd path` | Add folder |
| `:WorkspaceRemove` | Remove current folder |
| `:WorkspaceSave` | Save workspace file |

## Recommendations

### Project Structure


### Microservices


## Workspace Session

### Auto-Save


### Restore

Workspaces restore:
- Open buffers
- Window layout
- Cursor positions
- Fold state

## Tips

1. Use workspace files for complex projects
2. Name folders for clarity
3. Set per-folder overrides
4. Use finder for quick navigation
