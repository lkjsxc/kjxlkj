# TODO: Modes

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- [/docs/spec/modes/command.md](/docs/spec/modes/command.md)
- [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
- [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- [/docs/spec/modes/configuration.md](/docs/spec/modes/configuration.md)

## Normal mode

- [ ] Default mode on startup
- [ ] Motion dispatch (h/j/k/l, w/b/e, 0/$, gg/G, etc.)
- [ ] Operator-pending mode entry (d, c, y, etc.)
- [ ] Count prefix handling
- [ ] Dot-repeat (`.`) for last change

## Insert mode

- [ ] Entry commands: `i`, `I`, `a`, `A`, `o`, `O`
- [ ] Character insertion at cursor
- [ ] Backspace/delete within insert
- [ ] `Esc` returns to Normal mode
- [ ] Cursor clamping on exit (never past last char in Normal)
- [ ] Insert-mode navigation (arrow keys)
- [ ] Insert-normal mode (`Ctrl-o`)
- [ ] Auto-indentation on `Enter`
- [ ] Japanese IME composition handling per [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] Unicode input per [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- [ ] Digraph input per [/docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md)
- [ ] Literal input per [/docs/spec/modes/insert/input/insert-literal.md](/docs/spec/modes/insert/input/insert-literal.md)
- [ ] Register paste per [/docs/spec/modes/insert/input/insert-registers.md](/docs/spec/modes/insert/input/insert-registers.md)
- [ ] Special chars per [/docs/spec/modes/insert/input/insert-special-chars.md](/docs/spec/modes/insert/input/insert-special-chars.md)

## Insert mode: completion

- [/docs/spec/modes/insert/completion/README.md](/docs/spec/modes/insert/completion/README.md)
- [ ] Completion popup trigger and navigation
- [ ] Completion sources per [/docs/spec/modes/insert/completion/insert-completion-sources.md](/docs/spec/modes/insert/completion/insert-completion-sources.md)
- [ ] Abbreviation expansion per [/docs/spec/modes/insert/completion/insert-abbreviations.md](/docs/spec/modes/insert/completion/insert-abbreviations.md)
- [ ] Snippet expansion per [/docs/spec/modes/insert/completion/insert-snippets.md](/docs/spec/modes/insert/completion/insert-snippets.md)

## Insert mode: additional specs

- [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- [/docs/spec/modes/insert/insert-commands.md](/docs/spec/modes/insert/insert-commands.md)
- [/docs/spec/modes/insert/insert-indentation.md](/docs/spec/modes/insert/insert-indentation.md)
- [/docs/spec/modes/insert/insert-mappings.md](/docs/spec/modes/insert/insert-mappings.md)
- [/docs/spec/modes/insert/insert-navigation.md](/docs/spec/modes/insert/insert-navigation.md)
- [/docs/spec/modes/insert/insert-normal.md](/docs/spec/modes/insert/insert-normal.md)
- [/docs/spec/modes/insert/insert-autopairs.md](/docs/spec/modes/insert/insert-autopairs.md)

## Visual mode

- [ ] Character-wise visual (`v`)
- [ ] Line-wise visual (`V`)
- [ ] Block-wise visual (`Ctrl-v`)
- [ ] Selection highlighting
- [ ] Operators on visual selection (d, c, y, etc.)
- [ ] Per [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)

## Command-line mode

- [ ] Entry via `:`, `/`, `?`
- [ ] Command parsing and execution
- [ ] Command-line editing (cursor movement, delete)
- [ ] Command history (`Up`/`Down`)
- [ ] Tab completion

## Replace mode

- [ ] Entry via `R`
- [ ] Single-char replace via `r`
- [ ] Virtual replace per [/docs/spec/modes/replace/virtual-replace.md](/docs/spec/modes/replace/virtual-replace.md)
- [ ] Overstrike per [/docs/spec/modes/replace/overstrike.md](/docs/spec/modes/replace/overstrike.md)

## Mode transitions

- [ ] All transitions per [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- [ ] CJK cursor clamping on mode change (no half-cell positions)

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [ ] `i`/`I`/`a`/`A`/`o`/`O` each enter Insert mode with correct cursor placement
- [ ] `Esc` from Insert mode transitions to Normal and clamps cursor to end-exclusive range
- [ ] `v`/`V`/`Ctrl-v` each enter the correct Visual sub-mode with anchor set
- [ ] `:` enters Command mode with empty command buffer and cursor at position 0
- [ ] `R` enters Replace mode; typed characters overwrite at cursor position
- [ ] `r{char}` replaces single character without entering Replace mode
- [ ] Mode-dependent keybinding dispatch: same key produces different actions in different modes
- [ ] Rapid mode switching (`i Esc i Esc` x100) causes no memory leak or cursor drift
- [ ] Mode indicator in statusline updates on every transition
