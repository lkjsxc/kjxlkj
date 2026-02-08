# Advanced Split Handling

Complex split operations and management.

## Overview

Beyond basic `:split`/`:vsplit`, the window system supports movement, rotation, sizing, pinning, and layout algorithms.

## Split Commands (normative)

| Command | Action |
|---|---|
| `:split [file]` | Horizontal split; optionally open file |
| `:vsplit [file]` | Vertical split; optionally open file |
| `:new` | Horizontal split with empty buffer |
| `:vnew` | Vertical split with empty buffer |
| `:{N}split` | Horizontal split with height N |
| `:{N}vsplit` | Vertical split with width N |

## Split Direction Options

| Option | Default | Effect |
|---|---|---|
| `splitbelow` | false | New horizontal splits appear below |
| `splitright` | false | New vertical splits appear to the right |

## Window Movement (normative)

| Key | Action |
|---|---|
| `Ctrl-w H` | Move current window to far left (full height) |
| `Ctrl-w J` | Move current window to bottom (full width) |
| `Ctrl-w K` | Move current window to top (full width) |
| `Ctrl-w L` | Move current window to far right (full height) |
| `Ctrl-w r` | Rotate windows downward/rightward in current row/column |
| `Ctrl-w R` | Rotate windows upward/leftward |
| `Ctrl-w x` | Exchange current window with next one |

## Window Sizing (normative)

| Key / Command | Action |
|---|---|
| `Ctrl-w =` | Make all windows equal size |
| `Ctrl-w +` / `Ctrl-w -` | Increase/decrease height by 1 (or count) |
| `Ctrl-w >` / `Ctrl-w <` | Increase/decrease width by 1 (or count) |
| `Ctrl-w _` | Set height to count (or maximize) |
| `Ctrl-w \|` | Set width to count (or maximize) |
| `:resize {N}` | Set height to N |
| `:vertical resize {N}` | Set width to N |

Minimum window dimensions: height = 1 line + 1 statusline, width = 1 column.

## Window Types (normative)

| Type | Behavior |
|---|---|
| Normal | Standard editing buffer |
| Preview | Shows preview content; only one allowed; `:pclose` closes |
| QuickFix | Shows quickfix list; `:copen` / `:cclose` |
| Location list | Per-window error list; `:lopen` / `:lclose` |
| Terminal | Embedded terminal emulator; managed per terminal spec |
| Float | Floating overlay window; not part of split layout |

## Pinned Windows

A pinned window is not closed by `:only`. Implementation tracks a `pinned: bool` flag per window. `:pin` pins, `:unpin` unpins.

## Smart Splits

When `splitauto` is enabled, the editor chooses horizontal or vertical split based on the current window's aspect ratio: if width > 2 * height, split vertically; otherwise horizontally.

## Scroll Binding

`:set scrollbind` in two or more windows synchronizes their vertical scroll positions. When one scrolls, the others follow by the same line delta.

## Related

- Window spec: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Window navigation: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)
