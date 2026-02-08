# Expression Register (Session Context)

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

Session-specific behavior for expression register history and state.

## Overview

The expression register (`=`) allows evaluating expressions and using the result as text. This file documents session-related aspects. For core expression register behavior, see the registers specification.

## Session Persistence

| Data | Persisted | Location |
|---|---|---|
| Expression history | Yes | Session file |
| Last expression result | No | Memory only |

## History

Expression register entries are saved in the expression history, which survives session restores.

| Setting | Default | Description |
|---|---|---|
| `history.expression` | `50` | Maximum expression history entries |

## Related

- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
- Sessions: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
