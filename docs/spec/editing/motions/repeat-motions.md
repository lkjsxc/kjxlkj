# Repeat Motions

Back: [docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Repeat the last search or character-find motion.

## Overview

Several commands repeat a previous motion without
re-specifying the target. They fall into two groups:
find repeats (`;` and `,`) and search repeats
(`n` and `N`).

## Find Repeat

### Commands

| Motion | Direction | Description |
|--------|-----------|-------------|
| `;` | Same as last f/t/F/T | Repeat last character find |
| `,` | Opposite of last f/t/F/T | Reverse repeat |

### Interaction with Last Find

The `;` and `,` motions remember both the character
and the direction of the last `f`, `t`, `F`, or `T`
motion. They do NOT remember search (`/`, `?`).

### Examples

Given text `abcabc` with cursor on first `a`:
- `fb` moves to first `b`
- `;` moves to second `b`
- `,` moves back to first `b`

### Line Boundary

`;` and `,` operate within the current line only.
If there are no more matches on the current line,
the cursor does not move and no error is shown.

## Search Repeat

### Commands

| Motion | Direction | Description |
|--------|-----------|-------------|
| `n` | Same as last `/` or `?` | Repeat search forward/backward |
| `N` | Opposite of last search | Reverse search direction |

### Search Direction Memory

`n` repeats in the direction of the last search:
- After `/pattern`, `n` searches forward
- After `?pattern`, `n` searches backward

`N` always reverses the direction of `n`.

### Wrapping

Unlike find repeat, search repeat wraps around
the buffer (controlled by `wrapscan` option).
When wrapping occurs, a message is shown:
"search hit BOTTOM, continuing at TOP".

## Count Support

### Find Repeat with Count

`{count};` repeats the find motion count times.
`3;` finds the third next occurrence of the
character on the current line.

### Search Repeat with Count

`{count}n` jumps to the count-th next match.

## With Operators

### Find Repeat as Motion

| Combination | Effect |
|-------------|--------|
| `d;` | Delete to next find match |
| `c;` | Change to next find match |
| `y;` | Yank to next find match |
| `d,` | Delete to previous find match |

### Search Repeat as Motion

| Combination | Effect |
|-------------|--------|
| `dn` | Delete to next search match |
| `dN` | Delete to previous search match |
| `cn` | Change to next search match |

### Motion Type Preservation

The repeat commands preserve the inclusive/exclusive
type of the original motion:
- `f` repeat (`;`) is inclusive
- `t` repeat (`;`) is inclusive (exclusive in Vim)
- `F` repeat (`,`) is exclusive
- `T` repeat (`,`) is exclusive

## Star Search Repeat

After `*` or `#` word search:
- `n` finds next occurrence of the word
- `N` finds previous occurrence

The `*` command sets the search pattern to the word
under the cursor with `\b` boundaries.

## Edge Cases

| Scenario | Behavior |
|----------|----------|
| No previous find | `;` and `,` do nothing |
| No previous search | `n` and `N` show error |
| Empty search pattern | Uses last non-empty pattern |
| Count exceeds matches | Moves to last possible match |

## Related

- Character find: [docs/spec/editing/motions/character-find.md](/docs/spec/editing/motions/character-find.md)
- Search motions: [docs/spec/editing/motions/search-motions.md](/docs/spec/editing/motions/search-motions.md)
- Motions overview: [docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
