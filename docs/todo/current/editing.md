# TODO: Editing Primitives

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Cursor model

- [x] Grapheme-based cursor position `(line, grapheme_offset)` per cursor spec
- [x] Bidirectional grapheme-to-display-column mapping
- [ ] CJK wide char: cursor always on grapheme boundary, never half-cell
- [ ] CJK motion atomicity: `l`/`h` skip entire width-2 grapheme
- [ ] Cursor rendering: block cursor spans full display width of grapheme
- [ ] Append-at-EOL (`a`) and `Esc` return never leaves floating cursor (REG-01)

## Motions

- [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- [x] Character motions: `h`, `l`, `space`, `backspace`
- [x] Line motions: `j`, `k`, `+`, `-` per [/docs/spec/editing/motions/line-motions.md](/docs/spec/editing/motions/line-motions.md)
- [x] Word motions: `w`, `W`, `b`, `B`, `e`, `E` per [/docs/spec/editing/motions/word-WORD.md](/docs/spec/editing/motions/word-WORD.md)
- [x] Line-position motions: `0`, `^`, `$`, `g_`
- [x] File motions: `gg`, `G`, `{count}G`
- [x] Search motions: `/`, `?`, `n`, `N`, `*`, `#` per [/docs/spec/editing/motions/search-motions.md](/docs/spec/editing/motions/search-motions.md)
- [x] Character find: `f`, `F`, `t`, `T`, `;`, `,` per [/docs/spec/editing/motions/character-find.md](/docs/spec/editing/motions/character-find.md)
- [x] Sentence/paragraph: `(`, `)`, `{`, `}` per [/docs/spec/editing/motions/sentence-paragraph.md](/docs/spec/editing/motions/sentence-paragraph.md)
- [x] Scroll motions per [/docs/spec/editing/motions/scroll-motions.md](/docs/spec/editing/motions/scroll-motions.md)
- [x] Window motions per [/docs/spec/editing/motions/window-motions.md](/docs/spec/editing/motions/window-motions.md)
- [ ] Repeat motions per [/docs/spec/editing/motions/repeat-motions.md](/docs/spec/editing/motions/repeat-motions.md)
- [ ] Jump motions per [/docs/spec/editing/motions/jumps/README.md](/docs/spec/editing/motions/jumps/README.md)
- [x] Motion grammar per [/docs/spec/editing/motions/motion-grammar.md](/docs/spec/editing/motions/motion-grammar.md)

## Operators

- [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- [x] Delete (`d`), change (`c`), yank (`y`)
- [x] Operator grammar per [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- [x] Inclusive/exclusive per [/docs/spec/editing/operators/exclusive-inclusive.md](/docs/spec/editing/operators/exclusive-inclusive.md)
- [x] Linewise/characterwise per [/docs/spec/editing/operators/linewise-characterwise.md](/docs/spec/editing/operators/linewise-characterwise.md)
- [x] Double operators (`dd`, `cc`, `yy`) per [/docs/spec/editing/operators/double-operators.md](/docs/spec/editing/operators/double-operators.md)
- [x] Count with operators per [/docs/spec/editing/operators/count-with-operators.md](/docs/spec/editing/operators/count-with-operators.md)
- [ ] Forced motion types per [/docs/spec/editing/operators/forced-motion-types.md](/docs/spec/editing/operators/forced-motion-types.md)
- [ ] Operator modifiers per [/docs/spec/editing/operators/operator-modifiers.md](/docs/spec/editing/operators/operator-modifiers.md)
- [x] Operator-pending mode per [/docs/spec/editing/operators/operator-pending.md](/docs/spec/editing/operators/operator-pending.md)
- [ ] `g` operators per [/docs/spec/editing/operators/g-operators.md](/docs/spec/editing/operators/g-operators.md)
- [ ] Advanced operators per [/docs/spec/editing/operators/advanced.md](/docs/spec/editing/operators/advanced.md)

## Text objects

- [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- [x] Inner/around text objects (`iw`, `aw`, `is`, `as`, `ip`, `ap`)
- [x] Bracket text objects per [/docs/spec/editing/text-objects/bracket-text-objects.md](/docs/spec/editing/text-objects/bracket-text-objects.md)
- [x] Quote text objects per [/docs/spec/editing/text-objects/quote-text-objects.md](/docs/spec/editing/text-objects/quote-text-objects.md)
- [ ] Tag text objects per [/docs/spec/editing/text-objects/tag-text-objects.md](/docs/spec/editing/text-objects/tag-text-objects.md)
- [ ] Argument text objects per [/docs/spec/editing/text-objects/argument-text-objects.md](/docs/spec/editing/text-objects/argument-text-objects.md)
- [ ] Function/class text objects per [/docs/spec/editing/text-objects/function-text-objects.md](/docs/spec/editing/text-objects/function-text-objects.md)
- [ ] Treesitter text objects per [/docs/spec/editing/text-objects/treesitter-text-objects.md](/docs/spec/editing/text-objects/treesitter-text-objects.md)

## Registers

- [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- [x] Named registers (`"a`-`"z`) per [/docs/spec/editing/registers/named-registers.md](/docs/spec/editing/registers/named-registers.md)
- [x] Numbered registers (`"0`-`"9`) per [/docs/spec/editing/registers/numbered-registers.md](/docs/spec/editing/registers/numbered-registers.md)
- [x] Clipboard registers (`"+`, `"*`) per [/docs/spec/editing/registers/clipboard-registers.md](/docs/spec/editing/registers/clipboard-registers.md)
- [x] Black hole register (`"_`) per [/docs/spec/editing/registers/blackhole-register.md](/docs/spec/editing/registers/blackhole-register.md)
- [x] Read-only registers per [/docs/spec/editing/registers/readonly-registers.md](/docs/spec/editing/registers/readonly-registers.md)
- [ ] Expression register per [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
- [ ] Special registers per [/docs/spec/editing/registers/special-registers.md](/docs/spec/editing/registers/special-registers.md)
- [ ] Register commands per [/docs/spec/editing/registers/register-commands.md](/docs/spec/editing/registers/register-commands.md)

## Macros

- [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- [x] Record (`q{reg}`), playback (`@{reg}`), replay last (`@@`)
- [ ] Recursive macros per [/docs/spec/editing/macros/recursive-macros.md](/docs/spec/editing/macros/recursive-macros.md)
- [ ] Register-based macros per [/docs/spec/editing/macros/register-macros.md](/docs/spec/editing/macros/register-macros.md)
- [ ] Advanced macros per [/docs/spec/editing/macros/macros-advanced.md](/docs/spec/editing/macros/macros-advanced.md)

## Marks

- [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- [x] Local marks (`a`-`z`), global marks (`A`-`Z`)
- [x] Jump to mark (`` ` ``), jump to mark line (`'`)
- [x] Automatic marks per [/docs/spec/editing/marks/automatic-marks.md](/docs/spec/editing/marks/automatic-marks.md)
- [ ] Special marks per [/docs/spec/editing/marks/special-marks.md](/docs/spec/editing/marks/special-marks.md)
- [x] Jump list per [/docs/spec/editing/marks/jumplist.md](/docs/spec/editing/marks/jumplist.md)
- [x] Change list per [/docs/spec/editing/marks/changelist.md](/docs/spec/editing/marks/changelist.md)
- [ ] Mark persistence per [/docs/spec/editing/marks/mark-persistence.md](/docs/spec/editing/marks/mark-persistence.md)

## Search and regex

- [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)
- [x] Forward search (`/`), backward search (`?`)
- [x] Search highlight per [/docs/spec/editing/search/search-highlight.md](/docs/spec/editing/search/search-highlight.md)
- [x] Star search per [/docs/spec/editing/search/star-search.md](/docs/spec/editing/search/star-search.md)
- [ ] Search history per [/docs/spec/editing/search/search-history.md](/docs/spec/editing/search/search-history.md)
- [ ] Live grep per [/docs/spec/editing/search/live-grep.md](/docs/spec/editing/search/live-grep.md)
- [ ] Regex engine per [/docs/spec/editing/regex/regex.md](/docs/spec/editing/regex/regex.md)
- [ ] Magic modes per [/docs/spec/editing/regex/magic-modes.md](/docs/spec/editing/regex/magic-modes.md)

## Text manipulation

- [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- [x] Undo/redo per [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- [x] Join/split per [/docs/spec/editing/text-manipulation/join-split.md](/docs/spec/editing/text-manipulation/join-split.md)
- [x] Case changing per [/docs/spec/editing/text-manipulation/case-changing.md](/docs/spec/editing/text-manipulation/case-changing.md)
- [x] Increment/decrement per [/docs/spec/editing/text-manipulation/increment-decrement.md](/docs/spec/editing/text-manipulation/increment-decrement.md)
- [ ] Substitute per [/docs/spec/editing/text-manipulation/substitute.md](/docs/spec/editing/text-manipulation/substitute.md)
- [ ] Sorting/alignment per [/docs/spec/editing/text-manipulation/sorting-alignment.md](/docs/spec/editing/text-manipulation/sorting-alignment.md)
- [ ] Filtering/piping per [/docs/spec/editing/text-manipulation/filtering-piping.md](/docs/spec/editing/text-manipulation/filtering-piping.md)
- [x] Bracket matching per [/docs/spec/editing/text-manipulation/bracket-matching.md](/docs/spec/editing/text-manipulation/bracket-matching.md)
- [ ] Digraphs per [/docs/spec/editing/text-manipulation/digraphs.md](/docs/spec/editing/text-manipulation/digraphs.md)

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [x] Every motion key (h/j/k/l/w/b/e/W/B/E/0/$/_/^/g_/gg/G) resolves to a real handler that moves the cursor
- [x] Every operator (d/c/y/>/</=) resolves to a real handler that mutates the buffer
- [x] Operator + motion combinations (dw, ci(, yap, etc.) are dispatched through the operator-pending path
- [x] Count prefix accumulates digits and multiplies the subsequent motion or operator
- [x] Register prefix (`"a`) sets the target register for the next yank/delete/paste
- [x] Dot repeat (`.`) replays the last recorded change with fidelity
- [x] Undo (`u`) and redo (`Ctrl-r`) traverse the undo tree correctly
- [ ] CJK motions move by grapheme, not byte offset, and never land on half-cell positions
- [x] Search motions (`/`, `?`, `n`, `N`) update cursor and search highlight simultaneously
- [x] Character find (`f`/`t`/`F`/`T`) and repeat (`;`/`,`) work within and across lines correctly
