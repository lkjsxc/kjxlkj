# Repeat Motions

Repeating search and find.

## Overview

Repeat the last search or character find motion.

## Find Repeat

| Motion | Direction |
|--------|-----------|
| `;` | Same direction as last f/t/F/T |
| `,` | Opposite direction |

## With Operators

Combining repeat motions with operators enables efficient editing patterns.

## Counts

Repeat motions accept counts (e.g., `3;` repeats find three times).

## Search Repeat

| Motion | Repeats |
|--------|---------|
| `;` | Last f/t/F/T |
| `,` | Last f/t/F/T reversed |
| `n` | Last / or ? |
| `N` | Last / or ? reversed |

## Edge Cases

| Behavior | Description |
|----------|-------------|
| End of Line | If no more matches on line, motion stops |
| No Wrap | f/t don't wrap around line ends |
| Line Boundary | `;` stops at line boundary |

## Related

- Motions overview: [motions.md](motions.md)
- Character find: [character-find.md](character-find.md)
- Search motions: [search-motions.md](search-motions.md)

