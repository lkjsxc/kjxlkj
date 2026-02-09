# Wave 14 Reconstruction Log

Date: 2026-02-09
Commit: (pending)

## Features Implemented

1. **REQ-WILDMENU-01**: Wildmenu display — completion candidates rendered on
   status bar row with highlight for selected item. CmdlineState extended with
   completions/completion_index fields, render_wildmenu() added to grid.rs.

2. **REQ-BLOCKCHG-01**: Block change replication — visual block `c` operator
   now sets block_insert_pending so typed text replicates to all lines on
   Insert→Normal transition.

3. **REQ-SRCHCOUNT-01**: Search match count — `[N/M]` displayed on cmdline
   row right side after search_next/search_prev. SearchState extended with
   match_count field, update_search_count() added to incsearch.rs.

4. **REQ-MACROEDIT-01**: Macro editing — sync_macro_to_register() and
   sync_register_to_macro() allow editing macros via register put/yank.

5. **REQ-GLOBALMARKS-01**: Global marks cross-buffer navigation — jump_to_mark
   and jump_to_mark_line now switch buffers for uppercase marks (A-Z).

6. **REQ-RANGEPROMPT-01**: Backwards range swap — ex_dispatch now auto-swaps
   start/end for backwards ranges instead of E493 error.

7. **REQ-CLASSTOBJ-01**: Class/function text objects — text_objects_class.rs
   provides ic/ac/if/af using brace-matching heuristic. Wired in text_objects.rs.

8. **REQ-EXPRPROMPT-01**: Expression register evaluation — Ctrl-R = in insert
   mode now evaluates last_ex_command via expr_eval and inserts result.

## New Files

- text_objects_class.rs (89 lines): Brace-matching class/function text objects
- wave14_tests.rs (115 lines): 8 tests covering all features

## Modified Files

- snapshot.rs: completions, completion_index, match_count fields
- cmdline.rs: snapshot() includes completions
- grid.rs: render_wildmenu(), search count display, Style fix
- visual_ops.rs: block change sets block_insert_pending
- incsearch.rs: update_search_count()
- editor_search_marks.rs: cross-buffer mark jumps, update_search_count calls
- macros.rs: sync_macro_to_register/sync_register_to_macro, clippy fix
- ex_dispatch.rs: backwards range swap
- text_objects.rs: 'c'|'f' routing
- insert_register.rs: expression evaluation
- cursor_ops_lists.rs: switch_to_buffer_id made pub(crate)
- lib.rs: text_objects_class, wave14_tests modules
- wave11_tests.rs: updated backwards_range_error → backwards_range_swap

## Test Results

- 257 tests total (244 core-state + 5 core-edit + 8 core-mode)
- 8 new wave14 tests, all passing
- Zero clippy warnings
- All files ≤ 200 lines
