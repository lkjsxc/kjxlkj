# Wave 30 — Reconstruction Log

## Features Implemented

### F1: Regex \zs/\ze match bounds (REQ-REGEXZS-01)
- Added `\zs`/`\ze` handling in `translate_vim_to_rust()` in `regex_translate.rs`
- Records positions, post-processes pattern: `(?:prefix)(match)(?:suffix)`
- Capture group 1 contains the actual match content
- Added `has_match_bounds` field to `TranslateResult`

### F2: :echomsg/:echohl (REQ-ECHOMSG-01)
- Added `:echomsg` command dispatch (info notification)
- Added `:echohl` command (sets/clears `echohl` option for highlight group)
- Both in `ex_dispatch.rs`

### F3: Visual block insert column tracking (REQ-VBLOCKCO-01)
- Fixed `editor_modes.rs`: block insert now uses tracked column from `block_insert_pending`
- Changed from `CursorPosition::new(sl, 0)` to `CursorPosition::new(sl, col)`

### F4: Completion source priority ordering (REQ-COMPLPRI-01)
- Added priority-based completion in `cmdline_completion.rs`
- Priority levels: 0=exact prefix builtin, 1=exact prefix user, 2=fuzzy builtin (score-based), 3=fuzzy user
- Sort by priority then alphabetically, dedup by name
- Added `fuzzy_filter_scored` returning `(score, name)` pairs

### F5: Expression register = inline evaluation (REQ-EXPRREG-01)
- `SelectRegister('=')` opens `=` prompt cmdline from normal mode
- `=` prompt execution stores result in `RegisterName::Expression`
- Sets `pending_register = Some('=')` for subsequent put
- `read_special_register('=')` returns expression register content
- `expr_from_insert` flag tracks insert-mode origin (preserves wave14 behavior)

### F6: g? ROT13 with motion count (REQ-ROT13-01)
- Added `'?'` → `Rot13` in `dispatch_g` in `dispatch.rs`
- Added `Rot13` handler in `apply_linewise_op` in `op_pending.rs`
- Added `ToggleCase` handler alongside in `apply_linewise_op`
- Added `Rot13` doubled operator (`g??`) in `op_pending_helpers.rs`
- Removed unreachable `_ => {}` catch-all (all operators now handled)

### F7: Spell checking stub (REQ-SPELL-01)
- Created `spell.rs` with `SpellChecker` struct
- Fields: enabled, good_words, bad_words, lang
- Methods: is_good, is_bad, add_good, add_bad, undo_good
- EditorState methods: toggle_spell, spell_add_good (zg), spell_add_bad (zw)
- Added `spell` field to `EditorState`

### F8: Local vimrc (.exrc) loading (REQ-EXRC-01)
- Added `load_local_exrc()` to `config_loader.rs`
- Checks `exrc` option, then tries `.exrc`, `.vimrc`, `.nvimrc` in cwd
- Calls `handle_source` to execute the found file

## Files Modified
| File | Before | After | Change |
|---|---|---|---|
| regex_translate.rs | 167 | 193 | \zs/\ze match bounds + has_match_bounds field |
| ex_dispatch.rs | 192 | 199 | echomsg/echohl, expression prompt fix |
| editor_modes.rs | 199 | 199 | block insert column tracking fix |
| cmdline_completion.rs | 192 | 194 | priority ordering, fuzzy_filter_scored |
| editor_actions.rs | 196 | 196 | SelectRegister('=') opens prompt |
| editing_ops_yank.rs | 187 | 187 | read_special_register '=' case |
| insert_register.rs | ~40 | ~41 | expr_from_insert flag |
| dispatch.rs | 200 | 195 | g? → Rot13, compressed dispatch_g |
| op_pending.rs | 191 | 192 | Rot13/ToggleCase in linewise, removed _ => {} |
| op_pending_helpers.rs | 55 | 56 | Rot13 doubled operator |
| editor.rs | 200 | 196 | spell field, expr_from_insert, compressed structs |
| config_loader.rs | 189 | 198 | load_local_exrc, compressed auto-restore |
| lib.rs | 172 | 174 | mod spell, mod wave30_tests |

## Files Created
| File | Lines | Purpose |
|---|---|---|
| spell.rs | ~95 | Spell checker stub with word lists |
| wave30_tests.rs | ~130 | 10 tests for all wave30 features |

## Test Results
- Total tests: 393 (5 + 380 + 8)
- All passing, zero clippy warnings
- All source files ≤ 200 lines
