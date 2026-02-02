# Keybindings: Advanced

Expert-level nvim keybindings for power users.

## Expression Register

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-r =` | Insert expr | Insert expression result |
| `"=` | Expr register | Use expression in yank/put |

## Advanced Motions

| Key | Action | Description |
|-----|--------|-------------|
| `[(` | Prev unmatched ( | Previous unmatched ( |
| `])` | Next unmatched ) | Next unmatched ) |
| `[{` | Prev unmatched { | Previous unmatched { |
| `]}` | Next unmatched } | Next unmatched } |
| `[m` | Prev method start | Previous method start |
| `]m` | Next method start | Next method start |
| `[M` | Prev method end | Previous method end |
| `]M` | Next method end | Next method end |
| `[[` | Prev section | Previous section/function |
| `]]` | Next section | Next section/function |
| `[]` | Prev section end | Previous section end |
| `][` | Next section end | Next section end |

## Advanced Editing

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-a` | Increment | Increment number under cursor |
| `Ctrl-x` | Decrement | Decrement number under cursor |
| `<N>Ctrl-a` | Add N | Add N to number |
| `g Ctrl-a` | Seq increment | Sequential increment (Visual) |
| `g Ctrl-x` | Seq decrement | Sequential decrement (Visual) |
| `gq<motion>` | Format | Format text over motion |
| `gw<motion>` | Format keep | Format, keep cursor |
| `gqq` | Format line | Format current line |
| `gqap` | Format paragraph | Format paragraph |

## Visual Mode Advanced

| Key | Action | Description |
|-----|--------|-------------|
| `o` | Swap ends | Swap selection ends |
| `O` | Swap corners | Swap block corners (Visual Block) |
| `$` | Extend to EOL | Extend to end of all lines |
| `I` | Insert block | Insert at block start |
| `A` | Append block | Append at block end |
| `c` | Change block | Change block selection |
| `r<char>` | Replace block | Replace all chars in block |
| `>` | Indent | Indent selection |
| `<` | Outdent | Outdent selection |
| `J` | Join | Join selected lines |
| `gJ` | Join no space | Join without spaces |
| `g Ctrl-a` | Inc sequence | Increment as sequence |

## Folding

| Key | Action | Description |
|-----|--------|-------------|
| `zo` | Open fold | Open fold under cursor |
| `zO` | Open all nested | Open all folds under cursor |
| `zc` | Close fold | Close fold under cursor |
| `zC` | Close all nested | Close all folds under cursor |
| `za` | Toggle fold | Toggle fold under cursor |
| `zA` | Toggle all nested | Toggle all folds under cursor |
| `zv` | View cursor | Open folds to view cursor line |
| `zx` | Update folds | Update folds |
| `zX` | Undo manual | Undo manually opened/closed |
| `zm` | More folds | Increase foldlevel |
| `zM` | Close all | Close all folds |
| `zr` | Reduce folds | Decrease foldlevel |
| `zR` | Open all | Open all folds |
| `zn` | Fold none | Disable folding |
| `zN` | Fold normal | Enable folding |
| `zi` | Toggle folding | Toggle folding on/off |
| `[z` | Start of fold | Move to start of open fold |
| `]z` | End of fold | Move to end of open fold |
| `zj` | Next fold | Move to next fold |
| `zk` | Prev fold | Move to previous fold |
| `zf<motion>` | Create fold | Create fold over motion |
| `zd` | Delete fold | Delete fold under cursor |
| `zD` | Delete recursive | Delete all folds under cursor |
| `zE` | Eliminate folds | Delete all folds in window |

## Spell Checking

| Key | Action | Description |
|-----|--------|-------------|
| `]s` | Next misspell | Next misspelled word |
| `[s` | Prev misspell | Previous misspelled word |
| `z=` | Suggest | Show spelling suggestions |
| `zg` | Add good | Add word to dictionary |
| `zw` | Add wrong | Mark word as wrong |
| `zug` | Undo good | Undo zg |
| `zuw` | Undo wrong | Undo zw |

## Quickfix

| Key | Action | Description |
|-----|--------|-------------|
| `:copen` | Open quickfix | Open quickfix window |
| `:cclose` | Close quickfix | Close quickfix window |
| `:cnext` | Next item | Go to next quickfix item |
| `:cprev` | Prev item | Go to previous quickfix item |
| `:cfirst` | First item | Go to first quickfix item |
| `:clast` | Last item | Go to last quickfix item |
| `]q` | Next quickfix | Go to next quickfix item |
| `[q` | Prev quickfix | Go to previous quickfix item |

## Location List

| Key | Action | Description |
|-----|--------|-------------|
| `:lopen` | Open loclist | Open location list |
| `:lclose` | Close loclist | Close location list |
| `:lnext` | Next item | Go to next location |
| `:lprev` | Prev item | Go to previous location |
| `]l` | Next location | Go to next location item |
| `[l` | Prev location | Go to previous location item |

