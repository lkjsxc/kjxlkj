# Session Management

kjxlkj supports session persistence for workflow continuity.

## Overview

Sessions save and restore:
- Open buffers
- Window layout
- Cursor positions
- Undo history (optional)
- Working directory

## Configuration


## Session Files

Default location: `~/.local/share/kjxlkj/sessions/`

### Named Sessions


### Auto Sessions

Per-directory auto sessions:


When enabled:
- Opening kjxlkj in a git repo root loads that project's session
- Session named by directory path hash

## Commands

| Command | Description |
|---------|-------------|
| `:SessionSave` | Save current session |
| `:SessionSave!` | Force overwrite |
| `:SessionLoad` | Load session (picker) |
| `:SessionDelete` | Delete a session |
| `:SessionNew` | Start fresh session |

## Keybindings


## Session File Format

Sessions stored as TOML:


## Workflow Examples

### Project-based Sessions


### Manual Session Management


## Troubleshooting

### Session not restoring

1. Check `session.auto_restore = true`
2. Verify session file exists
3. Check file permissions

### Corrupted session

Delete and recreate:
