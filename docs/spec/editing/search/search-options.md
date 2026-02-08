# Search Options

Configuration options controlling search behavior.

## Highlight Options

### hlsearch (Highlight Search)

When enabled (`hlsearch = true`), all matches of the last
search pattern are highlighted in the buffer. Highlighting
persists until `:nohlsearch` is run or the pattern changes.
Default: `true`.

### incsearch (Incremental Search)

When enabled (`incsearch = true`), the buffer scrolls to
show the first match as the pattern is being typed.
The match is highlighted in real-time. The cursor
returns to its original position if search is canceled.
Default: `true`.

## Case Options

### ignorecase

When `ignorecase = true`, search patterns are matched
case-insensitively by default. `foo` matches `Foo`, `FOO`,
`foo`. Default: `false`.

### smartcase

When `smartcase = true` (requires `ignorecase = true`):
- Pattern with only lowercase: case-insensitive
- Pattern with any uppercase: case-sensitive
- `foo` matches `Foo`; `Foo` only matches `Foo`

Default: `false`.

## Wrap Options

### wrapscan

When `wrapscan = true`, searches wrap around the end
(or beginning for `?`) of the buffer. When `false`,
search stops at the last (or first) match and displays
"search hit BOTTOM" or "search hit TOP".
Default: `true`.

## Display Options

### shortmess

The `S` flag in `shortmess` suppresses search count
messages (e.g. `[3/15]`). When `S` is absent, the
current match index and total count are displayed in
the command line during search navigation.

## Search Match Display

The current match is highlighted with `IncSearch` highlight
group. Other matches use `Search` group. These are
configurable via colorscheme TOML under `[highlights]`.

## Performance Options

### redrawtime

Maximum time in milliseconds spent highlighting search
matches before giving up. Prevents UI freezes on
pathological patterns. Default: `2000`.

### maxmempattern

Maximum memory (KB) the regex engine may use for
pattern matching. Prevents runaway memory on complex
patterns. Default: `1000`.

## Regex Engine Options

### regexpengine

| Value | Engine |
|-------|--------|
| `0` | Automatic selection (default) |
| `1` | Backtracking NFA |
| `2` | DFA (faster, fewer features) |

Automatic mode uses DFA when the pattern supports it
and falls back to NFA for lookahead/lookbehind.

## Option Combinations

### Common Configurations

| Use Case | Settings |
|----------|----------|
| Case-smart | `ignorecase = true`, `smartcase = true` |
| No wrap | `wrapscan = false` |
| Fast highlight | `redrawtime = 500` |
| Minimal messages | `shortmess` includes `S` |

## Override in Pattern

| Prefix | Effect |
|--------|--------|
| `\c` | Force case-insensitive for this pattern |
| `\C` | Force case-sensitive for this pattern |

These override `ignorecase` and `smartcase` for the
single search where they appear.

## Keybindings for Options

| Key | Action |
|-----|--------|
| `<Leader>h` | Toggle hlsearch on/off |
| `<Leader>/` | Run `:nohlsearch` (clear current highlight) |
| `<Esc><Esc>` | Clear highlight (normal mode) |

## Configuration File

All search options are set in `~/.config/kjxlkj/config.toml`
under `[editor.search]`:
- `hlsearch` (bool)
- `incsearch` (bool)
- `ignorecase` (bool)
- `smartcase` (bool)
- `wrapscan` (bool)
- `redrawtime` (integer)
- `maxmempattern` (integer)
- `regexpengine` (integer)

## API Reference

| Function | Return |
|----------|--------|
| `searchcount()` | `{ current, total, incomplete }` |
| `searchpos(pattern)` | `[line, col]` of next match |

