# Insert Abbreviations (Completion Context)

Text expansion while typing using abbreviations.

## Overview

This document covers abbreviation-triggered completions.
For the full abbreviation specification, see the main
abbreviation document in insert mode.

## Abbreviation as Completion

### Trigger

Abbreviations act as a form of auto-completion: type
a short trigger, press a non-keyword character, and
the abbreviation expands into the full text.

### vs Completion Menu

Unlike completion menu entries, abbreviations:
- Expand automatically (no explicit accept step)
- Do not show a popup menu
- Are defined statically (not context-aware)

## Abbreviation-Based Snippets

### Simple Expansion

`:iabbrev fn function` expands `fn` to `function`.
Useful for common keywords and boilerplate.

### Multi-Line

`:iabbrev main public static void main(String[] args) {}`
Can use `<CR>` for line breaks in the expansion.

### Expression

`:iabbrev <expr> date strftime("%Y-%m-%d")` expands
`date` to the current date dynamically.

## Integration with Completion

### Priority

When both an abbreviation and a completion menu entry
match, the abbreviation takes priority since it triggers
on non-keyword character input.

### Coexistence

Abbreviations and completion can coexist. The completion
menu appears for partial matches while abbreviations
expand on trigger characters.

## Common Patterns

### Date/Time

`:iabbrev <expr> now strftime("%H:%M")`

### Signatures

`:iabbrev <buffer> sig Regards,<CR>Your Name`

### Corrections

`:iabbrev teh the`
`:iabbrev dont don't`
