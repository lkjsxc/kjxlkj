# Wave Progress Log

Tracks completion of each wave with evidence.

## Stage 00: Foundation Ingestion

### Waves 000–007
- Status: COMPLETE
- Evidence: All boxes checked, committed

## Stage 01: Architecture Core

### Waves 008–015
- Status: COMPLETE
- Evidence: All boxes checked, committed as 043b0f78
- Multi-task runtime architecture implemented

## Stage 02: Editing and Modes

### Wave 016: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: d5dfa1c1
- Evidence: 76 tests pass, all files ≤ 199 lines
- Key deliverables:
  - Motion enum expanded to ~40 variants
  - PendingState multi-key system (count, g/z/f/t/r/m)
  - g-prefix handler (gg, g_, ge, gE)
  - z-prefix handler (zz, zt, zb)
  - Find/till/paragraph/match-paren motions
  - All 8 word motions (w/b/e/ge/W/B/E/gE)
  - Operator composition (dd/yy/cc linewise, d3w)
  - File splits: other_modes, normal_partial,
    motion_find, motion_big_word, editor_ops

### Wave 017: Requirement Extraction and Normalization
- Status: COMPLETE
- Committed: 032b0aec
- Evidence: 91 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Operator enum expanded to 10 variants (+Lowercase, Uppercase, ToggleCase)
  - RangeType (Characterwise, Linewise, Blockwise) and Inclusivity (Inclusive, Exclusive) enums
  - motion_range_type() and motion_inclusivity() classification functions (7 tests)
  - g-prefix operators: gu→Lowercase, gU→Uppercase, g~→ToggleCase, gq→Format, gJ→JoinLinesNoSpace (5 new tests)
  - D→DeleteToEnd and Y→YankCurrentLine normal mode dispatch
  - Double-operator forms for all 10 operators (guu/gUU/g~~/gqq)
  - Case transform operators on lines and ranges (editor_ext.rs, 3 tests)
  - text_range() method on Buffer for range extraction
  - New files: motion_info.rs, editor_ext.rs

### Wave 018: State Model and Data Flow Design
- Status: COMPLETE
- Committed: 1aa214b9
- Evidence: 98 tests pass, all files ≤ 200 lines
- Key deliverables:
  - RegisterStore with named (a-z), numbered (0-9), unnamed, small-delete registers
  - record_yank() writes unnamed+0; record_delete() rotates 1-9 for linewise, writes - for small; A-Z append (5 tests)
  - ForceModifier enum (Characterwise, Linewise, Blockwise) for v/V/Ctrl-v in operator-pending
  - Pre-operator count multiplication (2d3w → count 6) via save_pre_op_count()/multiplied_count()
  - Filter operator (!) added as 11th Operator variant
  - Dot-repeat recording via last_change tracking and is_text_changing() classifier
  - Register prefix ("x) via PartialKey::Register dispatch
  - New file: register.rs in core-state

### Wave 019: Command and Route Wiring
- Status: COMPLETE
- Committed: 7306f5a9
- Evidence: 125 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Vim regex compiler (regex_compile.rs): magic-mode → Rust regex translation with
    \v very-magic switch, shortcut atoms (\d,\w,\s,\a,\l,\u,\x,\h), word boundaries,
    grouping (\(...\)→(...)), alternation (\|→|), quantifiers (\+→+, \?→?),
    \{n,m} brace quantifiers (10 tests)
  - Ex command parser (command_parse.rs): abbreviation-based dispatch for
    q/w/wq/x/e/bn/bp/bd/b/sp/vsp/clo/on/new/vnew/Explorer/terminal, ! force flag (8 tests)
  - Search system (search.rs): SearchState with direction, Vim regex compilation,
    find_next() forward with wrapping, find_prev() backward with last_match_before(),
    byte/char offset helpers (5 tests)
  - Command-line input (editor_cmdline.rs): handle_command_input() for char accumulation,
    Escape/Enter/Backspace transitions, activate_cmdline() for :/?? prefixes,
    dispatch_cmdline() routing, jump_to_match() cursor movement (4 tests)
  - CommandKind::Search split into SearchForward/SearchBackward
  - regex crate added to workspace dependencies
  - New files: regex_compile.rs, command_parse.rs, search.rs, editor_cmdline.rs

### Wave 020: Boundary and Error Semantics
- Status: COMPLETE
- Committed: d265278d
- Evidence: 132 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Blackhole register ("_): record_yank/record_delete skip all writes when register is '_' (3 tests)
  - Clipboard registers ("+, "*): store locally as named registers (real clipboard deferred)
  - Regex compiler refactored: emit_escaped/emit_brace helpers, compacted to 120 lines
  - \c/\C case sensitivity flags with vim_to_rust_regex_ex() returning case_flag
  - \o/\O octal atom, \H non-head-of-word atom, \= synonym for \? (4 tests)
  - Put operations: put_after (p) and put_before (P) with linewise/characterwise handling
  - Operators wired to RegisterStore: yank calls record_yank, delete/change call record_delete
  - Cursor boundary clamping: clamp_cursor() enforces valid bounds after mutations
  - Text extraction before mutation: text_range() collects text before buffer modification
  - editor_edit.rs: get_put_entry() helper, compacted to 175→179 lines

### Wave 021: Unit and Integration Coverage
- Status: COMPLETE
- Committed: 4a33e9fe
- Evidence: 142 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Read-only registers: ". (last insert text), "% (filename), "# (alternate file),
    ": (last Ex command), "/ (last search pattern)
  - set_readonly() for system-internal writes to read-only registers
  - list_all() returns sorted register listing for :registers command
  - Insert-text session tracking: insert_text accumulator in EditorState, cleared on
    insert entry, written to ". register on insert exit
  - update_filename_register() writes "% register
  - ":" register wired in dispatch_cmdline() for Ex commands
  - "/" register wired in execute_search() for search patterns
  - :registers/:reg/:display/:di → Action::ShowRegisters command
  - InsertChar pushes to insert_text accumulator
  - register.rs compacted to 198 lines with 14 tests (6 new)
  - editor.rs compacted to 186 lines with 7 tests (2 new)
  - editor_cmdline.rs compacted to 150 lines with 6 tests (2 new)
  - command_parse.rs updated to 169 lines with 9 tests (1 new)
  - editor_action.rs updated to 138 lines (InsertChar tracking, ShowRegisters stub)
  - action.rs compacted to 197 lines (ShowRegisters variant)

### Wave 022: Live E2E and Race Validation
- Status: COMPLETE
- Committed: 3b3c77a4
- Evidence: 167 tests pass, all files ≤ 200 lines
- Key deliverables:
  - Star search (*): word under cursor forward search with \b word boundaries
  - Hash search (#): word under cursor backward search with \b word boundaries
  - :nohlsearch/:noh command for clearing search highlighting
  - hlsearch state in SearchState, automatically re-enabled on new search
  - match_count() for total match reporting
  - set_raw_pattern() for direct Rust regex pattern injection (star/hash)
  - word_at() word-under-cursor extraction in search_util.rs
  - Search integration tests: star forward, star wrap, hash backward,
    n repeats, N reverses, nohlsearch clears, new search reactivates,
    star on non-word noop, star sets "/" register, multiline search,
    multiline wrap, empty buffer, match count after star
  - search.rs split: helper functions extracted to search_util.rs
  - Boundary tests: empty buffer, non-word cursor, no-match patterns

### Waves 023
- Status: COMPLETE
- Committed: 25bcc66d
- Evidence: 173 tests pass, all files ≤ 200 lines
- Key deliverables:
  - g* partial match forward search (no word boundaries)
  - g# partial match backward search (no word boundaries)
  - Search history tracking with consecutive deduplication (capped at 100)
  - ignorecase/smartcase settings in SearchState
  - Case flag application: in-pattern \c/\C override, smartcase uppercase detection
  - % bracket matching forward scan: when cursor is not on bracket, scan forward
    on current line for first bracket character
  - bracket_pair() helper function for bracket type lookup
  - Paragraph motion compaction (for/rev loops)
  - Integration tests: g* partial match, g# partial match, % forward scan,
    search history through cmdline, ignorecase via handle_key
  - motion.rs test: % scans forward for bracket from non-bracket position

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
