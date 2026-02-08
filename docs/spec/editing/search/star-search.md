# Star Search

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

Search for the word under the cursor using `*` and `#`.

## Overview

Star search searches for the word under the cursor without requiring the user to type the search pattern manually. The word boundaries are automatically added.

## Commands

| Key | Action |
|---|---|
| `*` | Search forward for the word under cursor (whole word match) |
| `#` | Search backward for the word under cursor (whole word match) |
| `g*` | Search forward for the word under cursor (partial match, no word boundaries) |
| `g#` | Search backward for the word under cursor (partial match, no word boundaries) |

## Word extraction

The word under the cursor is determined by the `iskeyword` option. The cursor must be on a keyword character for `*` and `#` to work. If the cursor is not on a keyword character, the search is not performed and a bell is emitted.

## Pattern construction

| Command | Generated pattern |
|---|---|
| `*` | `\<word\>` (whole word forward) |
| `#` | `\<word\>` (whole word backward) |
| `g*` | `word` (partial match forward) |
| `g#` | `word` (partial match backward) |

The generated pattern is set as the current search register (`/`) and search highlighting is updated.

## Behavior

After the pattern is set, the cursor jumps to the next (or previous) occurrence. If no occurrence is found, a "Pattern not found" message is displayed.

Star search wraps around the buffer (subject to `wrapscan` setting). When wrapping occurs, a "search hit BOTTOM, continuing at TOP" message is shown.

## Count

A count prefix is supported: `3*` searches forward and jumps to the 3rd occurrence of the word.

## Interaction with search

Star search sets the search register, so subsequent `n` and `N` repeat the star search pattern. The search history is updated.

## Visual mode

In Visual mode, `*` and `#` search for the selected text (not the word under cursor). The selection is escaped for use as a literal search pattern.

## CJK words

For CJK text, word boundaries may not apply in the same way. `*` on a CJK character searches for that character. `iskeyword` determines what constitutes a word.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `wrapscan` | `true` | Wrap search past end of buffer |
| `ignorecase` | `false` | Case-insensitive search |
| `smartcase` | `true` | Override ignorecase when pattern has uppercase |

## Related

- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- Search highlight: [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)
- Search commands: [/docs/spec/editing/search/search-commands.md](/docs/spec/editing/search/search-commands.md)
