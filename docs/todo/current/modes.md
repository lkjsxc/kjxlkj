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

- [x] Default mode on startup
- [x] Motion dispatch (h/j/k/l, w/b/e, 0/$, gg/G, etc.)
- [x] Operator-pending mode entry (d, c, y, etc.)
- [x] Count prefix handling
- [x] Dot-repeat (`.`) for last change

## Insert mode

- [x] Entry commands: `i`, `I`, `a`, `A`, `o`, `O`
- [x] Character insertion at cursor
- [x] Backspace/delete within insert
- [x] `Esc` returns to Normal mode
- [x] Cursor clamping on exit (never past last char in Normal)
- [x] Insert-mode navigation (arrow keys)
- [x] Insert-normal mode (`Ctrl-o`)
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

- [x] Character-wise visual (`v`)
- [x] Line-wise visual (`V`)
- [x] Block-wise visual (`Ctrl-v`)
- [ ] Selection highlighting
- [x] Operators on visual selection (d, c, y, etc.)
- [ ] Per [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)

## Command-line mode

- [x] Entry via `:`, `/`, `?`
- [x] Command parsing and execution
- [x] Command-line editing (cursor movement, delete)
- [x] Command history (`Up`/`Down`)
- [ ] Tab completion

## Replace mode

- [x] Entry via `R`
- [x] Single-char replace via `r`
- [ ] Virtual replace per [/docs/spec/modes/replace/virtual-replace.md](/docs/spec/modes/replace/virtual-replace.md)
- [ ] Overstrike per [/docs/spec/modes/replace/overstrike.md](/docs/spec/modes/replace/overstrike.md)

## Mode transitions

- [x] All transitions per [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)
- [ ] CJK cursor clamping on mode change (no half-cell positions)

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [x] `i`/`I`/`a`/`A`/`o`/`O` each enter Insert mode with correct cursor placement
- [x] `Esc` from Insert mode transitions to Normal and clamps cursor to end-exclusive range
- [x] `v`/`V`/`Ctrl-v` each enter the correct Visual sub-mode with anchor set
- [x] `:` enters Command mode with empty command buffer and cursor at position 0
- [x] `R` enters Replace mode; typed characters overwrite at cursor position
- [x] `r{char}` replaces single character without entering Replace mode
- [x] Mode-dependent keybinding dispatch: same key produces different actions in different modes
- [ ] Rapid mode switching (`i Esc i Esc` x100) causes no memory leak or cursor drift
- [x] Mode indicator in statusline updates on every transition
