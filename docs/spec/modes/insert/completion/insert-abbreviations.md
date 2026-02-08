# Insert Abbreviations Completion

Back: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)

How abbreviations interact with the completion system.

## Overview

Abbreviations and completion are independent systems that can coexist. Abbreviations trigger on non-keyword characters; completion triggers on keyword input or explicit invocation.

## Abbreviation as Completion

Abbreviations function as a simple form of text expansion. Unlike completion menu items, they expand inline without requiring selection.

| Aspect | Abbreviation | Completion |
|---|---|---|
| Trigger | Non-keyword character after keyword | `<C-x>` sequence or auto-trigger |
| UI | No menu | Menu popup |
| Selection | Automatic | User selects item |

## Abbreviation-Based Snippets

Abbreviations can serve as simple snippets:

| Abbreviation | Expansion | Use |
|---|---|---|
| `fn` | `function() {}` | Quick function template |
| `dt` | (expression for date) | Insert current date |
| `sig` | Full signature text | Email signature |

## Integration with Completion

When both systems are active:

1. The completion menu takes visual priority.
2. Abbreviation expansion occurs when the menu is dismissed or not showing.
3. If `<CR>` confirms a completion item, abbreviation expansion is suppressed.

## Related

- Abbreviations: [/docs/spec/modes/insert/insert-abbreviations.md](/docs/spec/modes/insert/insert-abbreviations.md)
- Completion: [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)
