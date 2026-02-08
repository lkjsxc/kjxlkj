# Code Folding

Back: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

Fold sections of text to hide detail and improve navigation.

## Overview

Folds collapse regions of text so only a summary line is visible. The editor supports indent-based, Tree-sitter-based, and manual folding.

## Fold Methods

| Method | Setting | Description |
|---|---|---|
| `indent` | `foldmethod=indent` | Fold by indentation level |
| `treesitter` | `foldmethod=treesitter` | Fold by Tree-sitter nodes |
| `manual` | `foldmethod=manual` | User-defined folds with `zf` |
| `marker` | `foldmethod=marker` | Fold between `{{{` and `}}}` markers |

## Fold Commands

| Key | Command | Description |
|---|---|---|
| `zo` | `:foldopen` | Open fold under cursor |
| `zc` | `:foldclose` | Close fold under cursor |
| `za` | - | Toggle fold under cursor |
| `zR` | - | Open all folds |
| `zM` | - | Close all folds |
| `zr` | - | Reduce fold level by 1 |
| `zm` | - | Increase fold level by 1 |
| `zf{motion}` | - | Create manual fold |
| `zd` | - | Delete fold under cursor |
| `zE` | - | Delete all folds |

## Fold Display

Folded lines are replaced with a single line showing:

1. The first line of the fold
2. A fold marker (configurable)
3. The number of folded lines

| Setting | Default | Description |
|---|---|---|
| `foldcolumn` | `1` | Width of fold indicator column |
| `foldtext` | default | Function that formats the fold display line |

## Fold Level

| Setting | Default | Description |
|---|---|---|
| `foldlevel` | `99` | Folds at this level or deeper are closed |
| `foldlevelstart` | `99` | Fold level when opening a file |
| `foldminlines` | `1` | Minimum lines for a fold to be closable |

## Nested Folds

Folds can be nested. `zo` opens the outermost fold at the cursor; repeated `zo` opens inner folds. `zO` opens all nested folds at the cursor.

## Navigation

| Key | Description |
|---|---|
| `[z` | Move to start of current fold |
| `]z` | Move to end of current fold |
| `zj` | Move to next fold |
| `zk` | Move to previous fold |

## Related

- Syntax: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)
- Tree-sitter: [/docs/spec/features/syntax/treesitter.md](/docs/spec/features/syntax/treesitter.md)
