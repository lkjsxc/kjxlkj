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

### Waves 019–023
- Status: NOT STARTED
