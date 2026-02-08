# Statusline DSL

Back: [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)

The domain-specific language for defining statusline content.

## Overview

The statusline DSL allows composing segments using variables, formatting directives, and conditional expressions.

## Variables

| Variable | Expands to |
|---|---|
| `%f` | File path (relative) |
| `%F` | File path (absolute) |
| `%m` | Modified flag (`[+]`) |
| `%r` | Readonly flag (`[-]`) |
| `%l` | Current line number |
| `%c` | Current column number |
| `%p` | Percentage through file |
| `%y` | File type |
| `%=` | Separator (left/right alignment) |

## Formatting

| Directive | Effect |
|---|---|
| `%{N}(...)` | Minimum width N |
| `%.{N}(...)` | Maximum width N (truncate) |
| `%-` | Left-align within width |

## Styling

Segments can reference highlight groups:

`%#HighlightGroup#text%#StatusLine#` — renders `text` using `HighlightGroup` colors, then reverts to `StatusLine`.

## Components

Custom components combine variables:

| Example | Result |
|---|---|
| `%f %m %r` | `src/main.rs [+]` |
| `%=%l:%c %p%%` | Right-aligned `42:10 50%` |

## Related

- Statusline config: [/docs/spec/features/ui/statusline/statusline-config.md](/docs/spec/features/ui/statusline/statusline-config.md)
- Highlight groups: [/docs/spec/features/syntax/highlight-groups.md](/docs/spec/features/syntax/highlight-groups.md)
