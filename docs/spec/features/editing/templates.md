# File Templates

Back: [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

Automatic content insertion when creating new files.

## Overview

When a new file is created, a file template matching its type can be automatically inserted. Templates support snippet syntax (tabstops, variables).

## Template Location

Templates are stored in the configuration directory under `templates/`:

| File | Template for |
|---|---|
| `templates/rust.rs` | New `.rs` files |
| `templates/python.py` | New `.py` files |
| `templates/html.html` | New `.html` files |

## Variables

Templates support the same variables as snippets:

| Variable | Value |
|---|---|
| `$TM_FILENAME` | File name |
| `$CURRENT_YEAR` | Current year |
| `$CURRENT_DATE` | Current date (ISO 8601) |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `templates.enabled` | `false` | Enable auto-templates |
| `templates.directory` | `templates` | Template directory |

## Related

- Snippets: [/docs/spec/features/editing/snippets.md](/docs/spec/features/editing/snippets.md)
