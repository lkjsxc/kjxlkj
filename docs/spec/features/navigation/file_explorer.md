# File Explorer (nvim-tree Built-in)

Native file tree explorer replacing nvim-tree.lua plugin.

## User Intent

Navigate and manipulate project files without leaving the editor.

## Activation

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>e` | Toggle | Toggle file explorer |
| `<leader>E` | Reveal | Open at current file |

## Navigation Keys

| Key | Action |
|-----|--------|
| `j` / `k` | Move down/up |
| `h` | Collapse / parent |
| `l` | Expand / open |
| `Enter` | Open or toggle |
| `gg` / `G` | First / last |

## Opening Files

| Key | Action |
|-----|--------|
| `o` | Open in current window |
| `v` | Vertical split |
| `s` | Horizontal split |
| `t` | New tab |

## File Operations

| Key | Action |
|-----|--------|
| `a` | Create file |
| `A` | Create directory |
| `d` | Delete (trash) |
| `D` | Force delete |
| `r` | Rename |
| `x` / `c` / `p` | Cut / copy / paste |
| `y` / `Y` / `gy` | Copy name / path / abs path |

## View Controls

| Key | Action |
|-----|--------|
| `R` | Refresh |
| `H` | Toggle hidden |
| `I` | Toggle gitignored |
| `/` | Filter |
| `q` | Close |

## Async Model

| Work | Service | Notes |
|------|---------|-------|
| Directory listing | FS service | Incremental and cancellable |
| File operations | FS service | Atomic where possible |
| Git badges | Git service | Updated async |
| Diagnostics | LSP service | Updated on publish |

## Visual Indicators

| Indicator | Meaning |
|-----------|---------|
| `▶` / `▼` | Collapsed / expanded |
| `[M]` `[+]` `[D]` `[?]` | Git status |
| `●` / `○` | Diagnostics |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `explorer.position` | `left` | Position |
| `explorer.width` | `30` | Width |
| `explorer.icons` | `true` | Show icons |
| `explorer.show_hidden` | `false` | Show hidden |

## Acceptance Criteria

- Expanding 10k children MUST not freeze input
- Rename/move MUST update buffers and watchers
- Badges MUST degrade gracefully when offline
