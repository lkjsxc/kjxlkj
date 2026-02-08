# Mark Persistence

Saving and restoring marks across editor sessions.

## What Gets Persisted (normative)

| Mark Type | Persisted | Storage |
|---|---|---|
| Uppercase (A-Z) | Yes | Global session file |
| Numbered (0-9) | Yes | Global session file |
| Lowercase (a-z) | Optional | Per-file metadata |
| `'"` (last position) | Yes | Per-file metadata |
| Other special marks | No | Runtime only |

## Storage Location

Marks are stored in the session JSON file at `~/.local/share/kjxlkj/session.json`. Per-file marks are within the `files` array of the session.

## Save Triggers

- **On exit**: All persistent marks saved on normal editor exit
- **On buffer close**: Buffer-local position (`'"`) saved when buffer is closed
- **Periodic**: Optionally, save every `session_save_interval` seconds (default: 300)

## Load Triggers

- **On startup**: Global marks (A-Z) and file history (numbered 0-9) loaded
- **On buffer open**: File-specific marks loaded when file is reopened

## Handling Missing Files

When a mark references a file that no longer exists:

| Behavior | Description |
|---|---|
| Remove (default) | Silently remove invalid marks |
| Keep | Preserve mark; jump may fail |

## Configuration

| Option | Default | Description |
|---|---|---|
| `persist_marks` | `true` | Enable mark persistence |
| `persist_local_marks` | `false` | Also persist lowercase (a-z) marks |
| `session_save_interval` | `300` | Auto-save interval in seconds (0 = disable) |

## Commands

| Command | Description |
|---|---|
| `:marks` | List all marks |
| `:delmarks a-z` | Delete marks a through z |
| `:delmarks!` | Delete all lowercase marks for current buffer |

## Related

- Jump marks: [/docs/spec/editing/marks/jump-marks.md](/docs/spec/editing/marks/jump-marks.md)
- Session persistence: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)
