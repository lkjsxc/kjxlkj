# Statusline / Tabline

## User intent

At-a-glance context:

- Mode, file name, modified state
- Cursor position, selection
- Diagnostics count, git branch/status
- LSP status and background activity

## Snapshot contract

Statusline data MUST be derived from the read-only snapshot.

| Field | Source |
|---|---|
| Mode | Core state |
| Buffer name + modified | Buffer metadata |
| Cursor and selection | Window/cursor state |
| Diagnostics count | LSP results merged into core view |
| Git branch/status | Git service results merged into core view |
| Task activity | Service supervision counters |

## Async indicators

The statusline MUST be the primary place to expose:

- Pending requests
- Queue saturation
- Service restarts

## Acceptance criteria

- Statusline updates MUST not require blocking queries.
- When a service is overloaded, the statusline MUST surface it.
