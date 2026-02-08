# Advanced Folds

Nested folds, fold methods, and custom folding.

## Fold Levels

Folds can nest up to 20 levels deep. Each line has a fold level (0 = not folded, 1+ = fold depth). Higher levels nest inside lower levels.

## Fold Methods

| Method | Source | Description |
|---|---|---|
| `manual` | User-created | Explicitly set with `zf` |
| `indent` | Indentation | Lines at same indent level fold together |
| `expr` | Expression | Custom function returns fold level per line |
| `syntax` | Syntax regions | Folds follow syntax highlighting regions |
| `marker` | Fold markers | `{{{` / `}}}` comment markers |
| `diff` | Diff mode | Unchanged text is folded |

Set via `foldmethod` option. Default: `manual`.

## Expression Folding

The `foldexpr` setting is evaluated for each line. It returns the fold level:

| Return Value | Meaning |
|---|---|
| `0` | Not folded |
| `1`, `2`, ... | Fold at this level |
| `>1` | Start a fold at level 1 |
| `<1` | End a fold at level 1 |
| `=` | Same level as previous line |
| `-1` | Undefined (use neighbor levels) |

Tree-sitter fold expressions use AST node types to determine fold boundaries automatically.

## Marker Folds

Default markers: `{{{` (open) and `}}}` (close). Numbered markers (`{{{1`, `}}}1`) set explicit levels. Customizable via `foldmarker` option.

## Fold Text

`foldtext` controls the display of closed folds. Available tokens:

| Token | Value |
|---|---|
| `{lines}` | Number of folded lines |
| `{level}` | Fold nesting level |
| `{first_line}` | Content of first folded line |
| `{percentage}` | Percentage of total file lines |

## Fold Commands

| Key | Action |
|---|---|
| `zo` | Open fold under cursor |
| `zc` | Close fold under cursor |
| `za` | Toggle fold |
| `zO` | Open fold recursively |
| `zC` | Close fold recursively |
| `zA` | Toggle fold recursively |
| `zR` | Open all folds |
| `zM` | Close all folds |
| `zr` | Reduce folding by one level |
| `zm` | Increase folding by one level |
| `zf{motion}` | Create manual fold |
| `zd` | Delete fold under cursor |
| `zE` | Eliminate all folds |
| `[z` | Move to start of current fold |
| `]z` | Move to end of current fold |
| `zj` | Move to next fold |
| `zk` | Move to previous fold |

## Fold Options

| Option | Default | Description |
|---|---|---|
| `foldlevel` | `0` | Folds with level > this are closed |
| `foldlevelstart` | `-1` | Initial foldlevel (-1 = use foldlevel) |
| `foldminlines` | `1` | Minimum lines to show a fold |
| `foldnestmax` | `20` | Maximum nesting depth |
| `foldcolumn` | `0` | Width of fold indicator column |

## Related

- Basic folds: [/docs/spec/features/syntax/folds-advanced.md](/docs/spec/features/syntax/folds-advanced.md)
