# Wave 13 Reconstruction Notes

## Summary
- **8 features** implemented: \V very-nomagic, session buffer assignments, block insert I/A, argument text objects, fuzzy completion, expression register (Ctrl-R), cross-buffer jumps, incremental search
- **9 tests** added in wave13_tests.rs
- **6 new files**: visual_block_insert.rs, text_objects_argument.rs, insert_register.rs, incsearch.rs, wave13_tests.rs, wave13-notes.md
- **Total tests**: 249 (236 core-state + 5 core-edit + 8 core-mode)

## Files Modified
- editor.rs: Added block_insert_pending, insert_register_pending fields
- editor_modes.rs: Block insert application on Insert→Normal transition; #[rustfmt::skip] on transition_mode
- editor_mode_dispatch.rs: Insert register pending check; Ctrl-R in insert mode
- visual_ops.rs: I/A keys for block mode; #[rustfmt::skip] on visual_apply_operator and visual_key_to_motion
- text_objects.rs: Added 'a' argument text object routing
- cmdline_completion.rs: Fuzzy matching fallback with fuzzy_matches() and fuzzy_filter()
- editing_ops_yank.rs: Made read_special_register pub(crate)
- editor_search_marks.rs: \V very-nomagic in strip_magic_prefix; refactored find/rfind helpers
- ex_session_cmds.rs: Per-window buffer path in serialize_layout Leaf nodes
- cmdline_dispatch.rs: Incremental search update after each search-mode keystroke
- cursor_ops_lists.rs: Cross-buffer jump switching via switch_to_buffer_id()
- lib.rs: Registered 6 new modules
- LIMITATIONS.md: Updated 8 limitation entries

## Key Decisions
- Block insert stores pending info as (start_line, end_line, col, at_end) tuple
- Argument text objects use 'a' as obj char (dia = inner argument, daa = around argument)
- Fuzzy completion is a fallback when no prefix match found
- Ctrl-R in insert mode sets insert_register_pending flag; next char selects register
- Cross-buffer jump switches focused window's content source
- Incremental search uses plain string find (not regex) for speed
