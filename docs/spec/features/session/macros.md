# Session Macros

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Macro register persistence across sessions.

## Overview

Macro registers (`a`-`z`) are saved in the session file so recorded macros survive editor restart.

## Saved Data

For each register containing a macro:

| Field | Description |
|---|---|
| `name` | Register letter |
| `content` | Keystroke sequence (stored as string) |
| `type` | Content type (char, line, block) |

## Restore Behavior

On session restore, register contents are loaded. Any existing register content is overwritten.

## Selective Save

Only non-empty named registers are saved.

## Related

- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- Sessions: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
