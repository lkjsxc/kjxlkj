# Session Registers

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Register persistence across sessions.

## Overview

Named registers (`a`-`z`) are saved in the session file so their contents persist across editor restarts.

## Saved Data

Per register:

| Field | Type | Description |
|---|---|---|
| `name` | string | Register letter |
| `content` | string | Text content |
| `type` | string | `char`, `line`, or `block` |

## Clipboard Integration

System clipboard registers (`+`, `*`) are NOT persisted — they reflect live system clipboard state.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `session.save_registers` | `true` | Save registers in session |

## Related

- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Sessions: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
