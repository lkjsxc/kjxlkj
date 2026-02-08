# Statusline / Tabline

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

## User intent

At-a-glance context:

- Mode, file name, modified state
- Cursor position, selection
- Diagnostics count, git branch/status
- LSP status and background activity

## Snapshot contract

Statusline data MUST be derived from the read-only snapshot.

| Field | Source | Example display |
|---|---|---|
| Mode | Core state | `NORMAL`, `INSERT`, `VISUAL` |
| Buffer name | Buffer metadata | `src/main.rs` |
| Modified flag | Buffer metadata | `[+]` when modified |
| Read-only flag | Buffer metadata | `[RO]` when read-only |
| Cursor position | Window/cursor state | `42:15` (line:col) |
| Selection info | Window/cursor state | `3 lines` or `12 chars` in visual |
| Diagnostics count | LSP results | `E:2 W:5` |
| Git branch/status | Git service results | `main +2 ~1 -0` |
| File type | Buffer metadata | `rust`, `markdown` |
| Encoding | Buffer metadata | `utf-8`, `latin-1` |
| Line ending | Buffer metadata | `LF`, `CRLF` |
| Task activity | Service supervision counters | Spinner when active |

## Tabline (normative)

The tabline is rendered at the top of the terminal when more than one tab exists.

| Aspect | Requirement |
|---|---|
| Content | Tab number, active buffer name, modified flag per tab |
| Active tab | Highlighted with `TabLineSel` group |
| Inactive tabs | Styled with `TabLine` group |
| Background fill | Remaining space styled with `TabLineFill` group |
| Overflow | When tabs exceed terminal width, show arrows and scroll |

## Layout (normative)

The statusline is divided into left, center, and right sections:

| Section | Default content |
|---|---|
| Left | Mode indicator, buffer name, modified/readonly flags |
| Center | Empty (reserved for user customization) |
| Right | File type, encoding, line ending, cursor position, diagnostics, git |

## Async indicators

The statusline MUST be the primary place to expose:

- Pending requests (spinner or progress indicator)
- Queue saturation (warning icon when channels are near capacity)
- Service restarts (brief flash or indicator)
- LSP status (initializing, ready, error)

## Highlight groups (normative)

| Group | Purpose |
|---|---|
| `StatusLine` | Active window statusline |
| `StatusLineNC` | Inactive window statusline |
| `StatusLineTerm` | Terminal window statusline |
| `TabLine` | Inactive tab |
| `TabLineSel` | Active tab |
| `TabLineFill` | Empty tabline space |

## Acceptance criteria

- Statusline updates MUST not require blocking queries.
- When a service is overloaded, the statusline MUST surface it.
- Statusline MUST update on every render frame (snapshot-driven).
- Each window MUST have its own statusline reflecting its buffer state.
