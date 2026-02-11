# Wave Progress Log

Tracks completion of each wave with evidence.

## Stages 00–02 (Archived)

See [wave-progress-stages-00-02.md](wave-progress-stages-00-02.md) for complete
Stage 00 (Foundation), Stage 01 (Architecture Core), and Stage 02
(Editing and Modes) progress details.

- Stage 00 (Waves 000–007): COMPLETE
- Stage 01 (Waves 008–015): COMPLETE, 043b0f78
- Stage 02 (Waves 016–023): COMPLETE, final commit 25bcc66d, 173 tests

## Stage 03: Commands and Ranges

### Wave 024: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: 67b0bb9c
- Evidence: 175 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Ctrl-a (IncrementNumber) and Ctrl-x (DecrementNumber) normal mode dispatch
  - Number increment/decrement implementation: find_number() scans forward from cursor,
    parses decimal integers (including negative), modifies value, replaces in buffer
  - :set/:se/:setlocal command parsing via parse_set_option(): boolean (set ignorecase),
    negation (set noignorecase), key=value (set tabstop=4)
  - apply_set_option() handles ignorecase/ic, smartcase/scs, hlsearch/hls options
  - Action::IncrementNumber, Action::DecrementNumber, Action::SetOption(String,String)
  - Integration tests: ctrl_a_increments_number, set_option_via_ex_command
  - Unit test: parse_set_option_forms in command_parse.rs
  - Compacted editor_edit.rs: tuple destructuring for cursor, min() for clamping

### Wave 025: Requirement Extraction and Normalization
- Status: COMPLETE
- Committed: 03f43e07
- Evidence: 189 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Text objects: iw/aw/iW/aW (word/WORD), i(/a)/ib, i{/a}/iB, i[/a], i</a>,
    i"/a", i'/a', i`/a` — 13 text object types with inner/around variants
  - text_object.rs in kjxlkj-core-edit: text_obj_range dispatcher, word_obj_range
    (big-word support), bracket_obj_range (nesting-aware, multiline, newline-trimmed),
    quote_obj_range (line-scoped quote pair search), byte_to_cursor helper (7 unit tests)
  - TextObjInner(char)/TextObjAround(char) Motion variants in action.rs
  - TextObjectInner/TextObjectAround PartialKey variants in pending.rs
  - Operator-pending text object dispatch: 'i'/'a' prefix → PartialKey → char → Motion
  - apply_operator_text_obj in editor_ops.rs: case ops, yank, delete/change with
    inclusive→exclusive range conversion (ecol+1)
  - other_modes.rs full rewrite: compacted all simple handlers, added text object
    partial resolution, double-op detection, force modifiers
  - Integration tests (editor_textobj_tests.rs): diw, daw, ciw, yiw, di(, ci{,
    operator_pending_i_a_prefix_keys (7 tests)

### Wave 026: State Model and Data Flow Design
- Status: COMPLETE
- Committed: e97095dc
- Evidence: 195 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Paragraph text objects (ip/ap): contiguous non-blank line detection with is_blank_line
    helper, around variant includes trailing blank lines
  - Sentence text objects (is/as): line-scoped sentence boundary at .!? characters,
    around variant includes trailing whitespace
  - text_object_ext.rs (126 lines): paragraph_obj_range, sentence_obj_range, 5 unit tests
  - text_object.rs updated: 'p' and 's' match arms delegating to text_object_ext
  - Integration tests: dip_deletes_inner_paragraph, dis_deletes_inner_sentence
  - Tree-sitter text objects (ic/ac, if/af) deferred (requires tree-sitter integration)
  - Tag text objects (it/at) deferred (requires HTML parser)
  - Tier-C docs read: class, function, inner, quote, tag, text_objects, treesitter

### Wave 027: Command and Route Wiring
- Status: COMPLETE
- Committed: 49d7bff3
- Evidence: 208 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Visual mode (v/V/Ctrl-v): anchor/cursor selection model in EditorState
  - Sub-mode switching: pressing different visual key switches mode, same key exits
  - Visual operators: d/x (delete), y (yank), c/s (change→insert), >/< (indent/dedent),
    ~/u/U (case), J (join), p (put)
  - Anchor swap (o): swaps visual_anchor and cursor position
  - editor_visual.rs (197 lines): apply_visual_operator dispatches by VisualKind to
    charwise/linewise/blockwise, apply_visual_char_op (inclusive→exclusive), 
    apply_visual_line_op (multi-line delete/yank), ordered() helper, 2 unit tests
  - editor_visual_tests.rs (139 lines): 11 integration tests covering entry, exit,
    delete, word delete, yank, anchor swap, line delete, sub-mode switch, same-key
    exit, change→insert, uppercase
  - VisualOperator(Operator) and VisualSwapAnchor Action variants
  - Visual anchor lifecycle in handle_key: set on visual entry, clear after action
    if was_visual (fixes ordering: apply_action before anchor clear)
  - other_modes.rs expanded: handle_visual_key with full operator/motion/sub-mode dispatch
  - Blockwise visual operations delegate to charwise (full block ops deferred)
  - editor.rs compacted to 200 lines (doc comment 3→1 line)
  - Tier-C docs read: visual/*, editor/*

### Wave 028: Boundary and Error Semantics
- Status: COMPLETE
- Committed: eabd7898
- Evidence: 213 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Buffer management: alternate buffer tracking, next/prev/delete/switch/open
  - Action variants: SwitchAlternate, ListBuffers added to action.rs
  - Ctrl-^/Ctrl-6 mapped to SwitchAlternate in normal.rs
  - :ls/:buffers command parsing in command_parse.rs
  - alternate_buffer field in EditorState for Ctrl-^ toggling
  - editor_buffer.rs (184 lines): switch_to_buffer, next/prev_buffer, delete_buffer,
    open_file (dedup-aware), switch_alternate, sorted_buffer_ids, current_buffer_id
  - All buffer actions wired in editor_action.rs dispatch
  - 5 unit tests: cycling, alternate toggle, delete fallback, last-buffer guard
  - Tier-C docs read: features/buffer/* (alternate-file, arglist, buffer-advanced,
    buffer-groups, buffer-local-options)

### Wave 029: Unit and Integration Coverage
- Status: COMPLETE
- Committed: 723f4ee4
- Evidence: 223 tests pass, all files ≤ 200 lines
- Key deliverables:
  - :bfirst/:bf and :blast/:bl commands for first/last buffer navigation
  - FirstBuffer and LastBuffer Action variants added to action.rs (200 lines)
  - first_buffer() and last_buffer() methods in editor_buffer.rs (194 lines)
  - FirstBuffer/LastBuffer dispatch wired in editor_action.rs (200 lines)
  - command_parse.rs compacted (merged 3 tests into 1): 194 lines
  - editor_buffer_tests.rs (153 lines): 13 integration tests covering buffer
    cycling, alternate toggle, deletion fallback, switch by ID, Ctrl-6 via
    handle_key, bfirst/blast navigation, open file dedup, delete-last-is-noop,
    ex command parsing for bf/bl/ls/buffers, delete updates alternate
  - Tier-C docs read: buffer-switching, bufferline, config/README, after-dir,
    audio, autocommands, command-palette

### Wave 030: Live E2E and Race Validation
- Status: COMPLETE
- Committed: 8e6d7881
- Evidence: 240 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Race condition stress tests: 10 tests in editor_race_tests.rs (140 lines)
    - rapid_mode_toggle_100_cycles: Normal→Insert→Normal 100 times
    - rapid_visual_toggle_100_cycles: Normal→Visual→Normal 100 times
    - command_mode_enter_exit_100_cycles: Normal→Command→Normal 100 times
    - insert_escape_preserves_text: typed text survives mode cycle
    - split_close_cycle_10_times: split→close→split 10 cycles
    - buffer_create_delete_cycle_20: open→delete→open 20 cycles
    - alternate_buffer_stress: 50 cycles of alternate buffer switching
    - resize_boundary_1x1: editor handles 1×1 terminal size
    - resize_boundary_large: editor handles 1000×1000 terminal size
    - resize_churn_50_cycles: rapid resize oscillation 50 cycles
  - Boundary safety tests: 7 tests in editor_boundary_tests.rs (83 lines)
    - deterministic_replay_insert_delete: identical key replay gives identical state
    - delete_on_empty_buffer_is_safe: delete char on empty buffer is no-op
    - motion_on_empty_buffer_is_safe: cursor motion on empty buffer stays at 0
    - ex_unknown_command_is_noop: unknown ex command does not crash
    - sequential_ex_commands: multiple ex commands execute in order
    - ctrl_6_without_alternate: Ctrl-6 with no alternate buffer is no-op
    - force_quit_sets_flag: :q! sets force_quit flag
  - Split editor_race_tests.rs (originally 218 lines) into race (140) + boundary (83)
  - lib.rs expanded to 47 lines (+editor_race_tests, +editor_boundary_tests modules)
  - Tier-C docs read: config/filetype, ftconfig, hooks-events, implementation,
    keybinding_hints, mouse-support, which-key
