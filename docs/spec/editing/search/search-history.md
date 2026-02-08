# Search History

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

History of search patterns used with `/` and `?`.

## Overview

Every search pattern entered with `/` or `?` is saved in the search history. The history can be navigated and re-used.

## Navigation

| Key | Action |
|---|---|
| `<Up>` / `<C-p>` | Previous search pattern |
| `<Down>` / `<C-n>` | Next search pattern |

History navigation filters entries to match any prefix already typed.

## History Size

| Setting | Default | Description |
|---|---|---|
| `history.search` | `100` | Maximum search history entries |

## Search History Window

`q/` opens the search history in a command-line window where entries can be browsed, edited, and executed with `<CR>`.

## Session Persistence

Search history is saved in the session file and survives editor restarts.

## Programmatic Access

| Command | Description |
|---|---|
| `:history /` | Display search history |

## Last Search Pattern

The last search pattern is stored in the `/` register. It can be inserted with `<C-r>/` in insert or command-line modes.

## Clear History

`:clearhistory search` clears the search history.

## Related

- Search commands: [/docs/spec/editing/search/search-commands.md](/docs/spec/editing/search/search-commands.md)
- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
- Command-line history: [/docs/spec/commands/cmdline/cmdline-history.md](/docs/spec/commands/cmdline/cmdline-history.md)
