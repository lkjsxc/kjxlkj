# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for reconstructed state.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior not implemented or unreachable |
| `M3 undocumented behavior` | runtime behavior exists without spec coverage |
| `M4 verification gap` | behavior exists but lacks deterministic tests |
| `M5 stale docs` | docs contain obsolete or conflicting claims |

## Reconstructed State Matrix

| Req ID | Canonical Spec | Expected Behavior | Current Gap | Class | Next Action | Status |
|---|---|---|---|---|---|---|
| R-BASELINE-01 | `/docs/spec/README.md` | full implementation conforming to spec | 18 crates built, 224 tests pass; all gaps closed; 100% C1 conformance | - | - | **closed** |
| R-KEY-01 | `/docs/spec/ux/keybindings/mode-entry.md` | `Shift+a` dispatches as `A` | implemented in decode.rs with WR-01 test | - | - | **closed** |
| R-CUR-01 | `/docs/spec/editing/cursor/README.md` | `a` at EOL appends after last grapheme | implemented in cursor_ops.rs with CUR-01 through CUR-05 tests | - | - | **closed** |
| R-EXP-01 | `/docs/spec/features/navigation/file_explorer.md` | explorer launch, navigation, and file ops | `:Explorer`/`<leader>e` toggle and `<leader>E` reveal; j/k/h/l nav with expand/collapse; file create/rename/delete; dispatch_explorer_key; gap_tests pass | - | - | **closed** |
| R-TERM-01 | `/docs/spec/features/terminal/terminal.md` | PTY-backed terminal window path | `:terminal`/`<leader>t`/`<leader>th`/`<leader>tv` create terminal windows; VT parser and screen model implemented; terminal resize propagation via TerminalService; ST-01 to ST-12, PE-01 to PE-06, gap_tests pass | - | - | **closed** |
| R-WIN-01 | `/docs/spec/features/window/splits-windows.md` | mixed-window `Ctrl-w` correctness | Ctrl-w w/W/s/v/c/q navigation implemented for all window types; WR-06 test passes; geometric directional Ctrl-w h/j/k/l with layout-aware focus using virtual rectangles; 230 tests pass | - | - | **closed** |
| R-I18N-01 | `/docs/spec/modes/insert/input/insert-japanese-ime.md` | IME composition and leader isolation | IME composition model (Idle/Preedit/CandidateSelect) implemented; leader isolation verified (JP-03); JP-01 to JP-05 pass; IME wired into insert mode key dispatch via route_ime_key; commit inserts text, cancel discards, space consumed during composition; 230 tests pass | - | - | **closed** |
| R-WRAP-01 | `/docs/spec/features/ui/viewport.md` | no off-screen long-line overflow | wrap algorithm with width-2 boundary padding implemented; BD-01, BD-02, BD-10, WR-07 tests pass; paint_window.rs delegates to render_buffer_wrapped when win.wrap=true; rwrap01 integration test verifies end-to-end wrapped rendering; 230 tests pass | - | - | **closed** |
| R-TODO-01 | `/docs/todo/current/README.md` | TODO is evidence-gated | all current/README.md and verification.md gates checked; conformance, limitations, and drift matrix synchronized; 230 tests pass | - | - | **closed** |

## Update Rules

- close rows only with deterministic evidence
- keep deferred rows open with explicit next action
- update this file alongside `CONFORMANCE` and `LIMITATIONS`

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
