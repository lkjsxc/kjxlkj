# Drift Closure Wave Log

Back: [/docs/log/README.md](/docs/log/README.md)

## Wave Summary

Date: 2026-02-10 (continuation)
Tests before: 224 → Tests after: 230
Drift items before: 4 open → After: 0 open

## Changes Made

### R-WIN-01: Geometric Directional Window Focus

- **Problem**: `Ctrl-w h/j/k/l` used simplified cycling (`WindowNext`/`WindowPrev`)
  instead of geometric directional navigation.
- **Solution**: Added `FocusDir` enum and `find_focus()` algorithm in
  `focus.rs` that computes virtual rectangles from the `LayoutNode` tree
  and finds the nearest window in the requested direction using overlap
  and distance checks.
- **Files changed**: `focus.rs` (new), `action.rs`, `dispatch.rs`,
  `tab_page.rs`, `editor_actions.rs`, `drift_tests.rs`
- **Evidence**: `directional_focus_vertical_split`, `directional_focus_horizontal_split`

### R-I18N-01: IME Insert Mode Dispatch Integration

- **Problem**: `ImeComposition` and `route_ime_key` existed in `kjxlkj-input`
  but were not wired into the actual key dispatch path in `EditorState`.
- **Solution**: Added `ime: ImeComposition` field to `EditorState` and
  inserted IME routing into `process_key` before normal mode dispatch.
  When in Insert mode and composing, keys are routed through `route_ime_key`
  first. `Commit` inserts text, `Cancelled` discards, `Consumed` suppresses
  normal dispatch (including leader key).
- **Files changed**: `editor.rs`, `editor_actions.rs`, `Cargo.toml`, `drift_tests.rs`
- **Evidence**: `ime_commit_inserts_text`, `ime_cancel_inserts_nothing`,
  `ime_space_consumed_during_composition`

### R-WRAP-01: Wrap Rendering Integration Verification

- **Problem**: Drift matrix claimed "integration with rendering pending" but
  `paint_window.rs` already delegates to `render_buffer_wrapped` when
  `win.wrap == true`.
- **Solution**: Added integration test `rwrap01_wrap_integration` that
  creates a `WindowSnapshot` with `wrap=true`, a buffer rope, calls
  `render_window`, and verifies the resulting grid contains wrapped content.
  Updated drift matrix to reflect actual state.
- **Files changed**: `render_tests.rs`, `DRIFT_MATRIX.md`
- **Evidence**: `rwrap01_wrap_integration`

### R-TODO-01: Evidence Gate Enforcement

- **Problem**: TODO verification gates needed to be verified against
  current implementation state.
- **Solution**: Checked all non-negotiable rules, completion definition
  items, reading discipline items, and archive policy items. Updated
  CONFORMANCE, LIMITATIONS, and DRIFT_MATRIX in the same change.
- **Evidence**: All `current/README.md` and `verification.md` gates
  checked; drift matrix fully closed.

## Cleanup

- Removed dead `editor_actions.rs` at crate root (186 lines, not registered
  in `lib.rs`, missing gap-closure features).
- Moved directional focus tests from `wiring_tests.rs` to `drift_tests.rs`
  to keep `wiring_tests.rs` under 200 lines.

## File Size Constraint Compliance

All source files under 200 lines verified.
