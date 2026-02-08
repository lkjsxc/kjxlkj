# Mark Navigation

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Patterns for navigating using marks.

## Overview

Marks provide named positions for quick navigation. This documents navigation workflows using marks.

## Basic Navigation

| Command | Jump to |
|---|---|
| `` `a `` | Exact position of mark `a` |
| `'a` | First non-blank character on mark `a`'s line |

## Motion vs Jump

`` `{mark} `` is a motion that can be combined with operators:

| Command | Effect |
|---|---|
| `` d`a `` | Delete from cursor to mark `a` |
| `` y`a `` | Yank from cursor to mark `a` |

Jumping to a mark creates a jump list entry.

## Common Workflows

| Pattern | Effect |
|---|---|
| `ma` ... `` `a `` | Set mark, work, return |
| `` `. `` | Jump to last edit position |
| `` `" `` | Jump to position when buffer was last closed |
| `` `[ `` | Jump to start of last change |

## Related

- Marks: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Mark commands: [/docs/spec/editing/marks/mark-commands.md](/docs/spec/editing/marks/mark-commands.md)
- Jump list: [/docs/spec/editing/marks/jumplist.md](/docs/spec/editing/marks/jumplist.md)
