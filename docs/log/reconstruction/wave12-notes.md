# Wave 12 Reconstruction Notes

## Features Implemented

1. **Jump list (Ctrl-O / Ctrl-I, :jumps)** — New `jumplist` and `jumplist_idx` fields in EditorState.
   Jump list pushed on search (n/N), mark jumps (`'/\``). Ctrl-O dispatched via core-mode dispatch,
   Ctrl-I likewise. `:jumps` displays entries.

2. **Alternate file (#) register** — `alternate_buffer` field tracks previous buffer ID.
   Set on `open_file()` and `next_buffer()/prev_buffer()`. `read_special_register('#')` returns
   alternate buffer path.

3. **Count-prefixed macro playback** — `PlayMacro(char, usize)` now carries count from dispatch.
   `3@a` correctly plays macro 'a' three times.

4. **:noh / :nohlsearch** — Clears `search.active` flag, suppressing hlsearch render.

5. **Nomagic regex mode** — `\M` prefix: only `^` and `$` are regex-special, everything else escaped.
   `\m` prefix explicitly selects magic mode (current default). `strip_magic_prefix()` replaces
   `strip_vmagic()`.

6. **Visual block column highlighting** — `is_in_visual_selection()` function in grid_window.rs
   checks visual selection with Block/Char/Line logic. Highlighted cells get blue background
   (RGB 100,100,255).

7. **Modeline parsing** — `parse_modeline()` called on `open_file()`. Scans first/last 5 lines
   for `vim:`, `vi:`, `ex:` patterns. Extracts options and applies via `parse_set_command()`.

8. **Session window layout** — `serialize_layout()` traverses `LayoutNode` tree writing `split`/
   `vsplit` commands. `:source` replays these, recreating the split structure.

## New Files

- `cursor_ops_lists.rs`: Changelist and jump list navigation (extracted from cursor_ops.rs).
- `ex_jump_noh.rs`: :jumps display, :noh handler, modeline parser.
- `wave12_tests.rs`: 10 tests covering all 8 requirements.

## Modified Files

- `editor.rs`: Added jumplist, jumplist_idx, alternate_buffer fields.
- `action.rs`: Added JumpOlder, JumpNewer; PlayMacro now (char, usize).
- `dispatch.rs`: Ctrl-O → JumpOlder, Ctrl-I → JumpNewer; @{c} carries count.
- `editor_actions.rs`: Wired JumpOlder, JumpNewer; PlayMacro with count.
- `ex_dispatch.rs`: Added :jumps, :noh, :nohlsearch routes.
- `editor_search_marks.rs`: search_next/prev push jumplist; mark jumps push jumplist;
  nomagic regex support via strip_magic_prefix + nomagic_to_literal.
- `editing_ops_yank.rs`: read_special_register handles '#'.
- `ex_buffer_cmds.rs`: next_buffer/prev_buffer set alternate_buffer.
- `ex_session_cmds.rs`: serialize_layout for split/vsplit persistence.
- `grid_window.rs`: Visual selection rendering with is_in_visual_selection.
- `lib.rs`: Registered new modules.

## Test Count

240 total (227 core-state + 5 core-edit + 8 core-mode).
