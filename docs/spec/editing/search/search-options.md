# Search Options

Back: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)

Configuration options that control search behavior.

## Overview

Search behavior is controlled by several settings that affect case sensitivity, wrapping, highlighting, and regex interpretation.

## Case Settings

| Setting | Default | Description |
|---|---|---|
| `ignorecase` | `false` | Ignore case in search patterns |
| `smartcase` | `true` | Override ignorecase if pattern has uppercase |

## Search Display

| Setting | Default | Description |
|---|---|---|
| `hlsearch` | `true` | Highlight all matches |
| `incsearch` | `true` | Show matches incrementally while typing |

## Wrapping

| Setting | Default | Description |
|---|---|---|
| `wrapscan` | `true` | Wrap search around end/beginning of buffer |

## Magic Mode

| Setting | Default | Description |
|---|---|---|
| `magic` | `true` | Use "magic" regex syntax by default |

With `magic` on, characters like `.`, `*`, `[` are regex meta-characters. With `nomagic`, they are literal.

## In-Pattern Flags

| Flag | Description |
|---|---|
| `\c` | Force case-insensitive |
| `\C` | Force case-sensitive |
| `\v` | Very magic (all non-alphanumeric chars are special) |
| `\V` | Very nomagic (only `\` is special) |
| `\m` | Magic (default) |
| `\M` | Nomagic |

## Related

- Search commands: [/docs/spec/editing/search/search-commands.md](/docs/spec/editing/search/search-commands.md)
- Search patterns: [/docs/spec/editing/search/search-patterns.md](/docs/spec/editing/search/search-patterns.md)
- Regex: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)
