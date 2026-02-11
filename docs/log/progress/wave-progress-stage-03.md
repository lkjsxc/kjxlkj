# Wave Progress — Stage 03: Commands and Ranges (Archived)

Back: [wave-progress.md](wave-progress.md)

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
  - Boundary safety tests: 7 tests in editor_boundary_tests.rs (83 lines)
  - 17 additional tests covering rapid mode toggle, visual toggle, command mode,
    insert preserves text, split-close cycle, buffer create-delete, alternate
    stress, resize boundary/churn, deterministic replay, empty buffer safety,
    unknown ex command, force quit flag
  - lib.rs expanded to 47 lines (+editor_race_tests, +editor_boundary_tests modules)
  - Tier-C docs read: config/filetype, ftconfig, hooks-events, implementation,
    keybinding_hints, mouse-support, which-key

### Wave 031: Ledger Synchronization and Stage Exit
- Status: COMPLETE
- Committed: fdab3a88
- Evidence: 252 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Stage-03 exit integration tests: 12 tests split across 2 files
  - editor_stage03_tests.rs (134 lines): 7 tests — ex command pipeline, insert
    persistence, d$, search, set option, star search, yy register
  - editor_stage03_edit_tests.rs (108 lines): 5 tests — diw, Ctrl-a, bfirst/blast,
    visual yank, bracket match (%)
  - DRIFT_MATRIX: R-EDIT-01 editing helpers (M2, spec-only)
  - lib.rs expanded to 51 lines (+2 test modules)
  - Ledger sync: CONFORMANCE (240→252), LIMITATIONS, DRIFT_MATRIX updated
  - Stage 03 closure: all 8 waves (024-031) complete
