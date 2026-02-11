# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for the current docs-only baseline.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent or unreachable |
| `M3 undocumented behavior` | behavior exists but is not specified canonically |
| `M4 verification gap` | behavior exists but deterministic evidence is insufficient |
| `M5 stale docs` | documentation claims are contradicted by stronger evidence |

## Matrix

| Req ID | Canonical Document | Requirement | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | grouped workspace and crate tree exist | topology + build gate | verified | closed | implement | 20-crate workspace, `cargo check --workspace` passes, 327 tests pass (2026-02-11) |
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches exactly as `A` | `WR-01R`, `KEYMODE-01` | partial | `M4` | test-add | T1 headless test passes; T2 PTY harness pending |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/close/rebalance is deterministic and visible | `WIN-01R`..`WIN-05R` | partial | `M4` | test-add | Ctrl-w dispatch implemented (h/j/k/l/w/p/t/b/s/v/n/c/q/o/+/-/>/</_/|/=), directional focus with geometry, boundary focus (t/b), equalize (=), resize/max as no-op placeholders, split semantics corrected; T1 unit + integration tests pass (15 wincmd unit + 26 integration tests); T2 PTY harness pending |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and leader routes are user-visible and reliable | `EXP-01R`..`EXP-06R` | partial | `M2`, `M4` | implement + test-add | explorer open/close routing (ContentKind::Explorer leaf creation, :ExplorerClose ex command), state model (ExplorerState/ExplorerNode/NodeId, expansion_set/visible_rows/flatten), navigation (ExplorerAction enum with MoveDown/MoveUp/CollapseOrParent/ExpandOrOpen/Toggle/Close), key routing (j/k/h/l/Enter/o/q intercept in Normal mode), 12 service tests + 5 editor_explorer tests + 19 integration tests; filesystem integration (actual dir listing) pending; T2 PTY E2E pending |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure must use user-like screen-state E2E assertions | all `*R` blocker rows | partial | `M2`, `M4` | implement + test-add | T1 harness exists; T2 PTY harness pending |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | source dirs stay near 12 children and files stay <=200 lines | topology checks | verified | closed | implement | all files ≤ 200 lines, multi-task runtime, topology matches spec (2026-02-11) |
| `R-OP-01` | [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md) | operator grammar, g-prefix ops, double forms, case transforms | unit tests | partial | `M4` | test-add | 11-variant Operator enum, gu/gU/g~/gq/gJ/D/Y/! dispatch, RangeType/Inclusivity classification, ForceModifier, count multiplication; T2 pending |
| `R-REG-01` | [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md) | register store with named/numbered/unnamed/small-delete/blackhole/clipboard/read-only | unit tests | partial | `M4` | test-add | RegisterStore with record_yank/record_delete, numbered rotation, A-Z append, blackhole "_ suppression, clipboard "+/"* stubs, put p/P, read-only "./"% /"#/":/"/ registers, set_readonly(), list_all(), :registers command; 14 unit tests; T2 pending |
| `R-REGEX-01` | [/docs/spec/editing/regex/magic-modes.md](/docs/spec/editing/regex/magic-modes.md) | Vim regex magic-mode translation to Rust regex | unit tests | partial | `M4` | test-add | vim_to_rust_regex with shortcut atoms, word boundaries, grouping, alternation, quantifiers, \v very-magic, \c/\C case flags, \o/\O octal, \H non-head, \= synonym; 14 unit tests; T2 pending |
| `R-CMD-01` | [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md) | Ex command parsing with abbreviation dispatch | unit tests | partial | `M4` | test-add | parse_ex_command with q/w/wq/x/e/bn/bp/bd/b/sp/vsp/clo/on/new/vnew/Explorer/terminal, ! flag, :set/:se/:setlocal option parsing; 9 unit tests; T2 pending |
| `R-TEXTOBJ-01` | [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md) | Text object selection with operator composition | unit + integration tests | partial | `M4` | test-add | iw/aw/iW/aW word objects, i(/a)/ib bracket objects (nesting-aware multiline), i{/a}/iB, i[/a], i</a>, i"/a"/i'/a'/i`/a` quote objects, ip/ap paragraph objects (contiguous non-blank lines, trailing blank inclusion), is/as sentence objects (period/exclamation/question boundary); operator-pending 'i'/'a' prefix dispatch; text_obj_range + text_object_ext in kjxlkj-core-edit; 12 unit tests + 9 integration tests; tree-sitter (ic/ac, if/af) and tag (it/at) objects deferred; T2 pending |
| `R-SEARCH-01` | [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md) | Forward/backward search, star/hash word search, hlsearch, match count | unit + integration tests | partial | `M4` | test-add | SearchState with find_next/find_prev, set_raw_pattern, hlsearch flag, clear_highlight, match_count, word_at; * and # word search with \b boundaries, g*/g# partial match, :nohlsearch/:noh; search history tracking, ignorecase/smartcase case flags; % bracket forward scan; :set/:se/:setlocal for ignorecase/smartcase/hlsearch options; text objects (iw/aw/bracket/quote/paragraph/sentence) via operator-pending dispatch; 10 search unit tests + 19 integration tests + 9 textobj integration tests; T2 pending |
| `R-VISUAL-01` | [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md) | Visual mode charwise/linewise/blockwise selection and operators | unit + integration tests | partial | `M4` | test-add | v/V/Ctrl-v entry, sub-mode switching, operators d/x/y/c/s/>/</~/u/U/J/p, anchor swap o, charwise inclusive→exclusive dispatch, linewise multi-line delete/yank, blockwise stub (delegates to charwise); 2 unit tests + 11 integration tests; T2 pending |
| `R-BUF-01` | [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md) | Buffer navigation, alternate buffer, deletion, file open | unit + integration tests | partial | `M4` | test-add | :bn/:bp/:bd/:b N, Ctrl-^, :ls/:buffers, :e file, :bfirst/:blast, alternate tracking, sorted ordering, delete fallback; 5 unit tests + 13 integration tests; T2 pending |
| `R-RACE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | Race condition and boundary safety coverage | unit tests | partial | `M4` | test-add | 10 stress tests (rapid mode/visual/command toggle 100 cycles each, insert preserves text, split-close cycle, buffer create-delete cycle, alternate stress, resize boundary/churn) + 7 boundary tests (deterministic replay, delete/motion on empty buffer, unknown ex command, sequential ex, Ctrl-6 without alternate, force quit); T2 pending |
| `R-EDIT-01` | [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md) | Auto-pairs, surround, comment toggle, clipboard sync, format-on-paste, format-on-type | n/a | spec-only | `M2` | implement | editing helper features (auto-pairs, surround, gcc/gc comment toggle, clipboard "+/"* system sync, format-on-paste, format-on-type) not yet implemented; deferred to later stages |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | Terminal emulation and PTY integration | `TERM-01R`..`TERM-04R` | partial | `M2`, `M4` | implement + test-add | TerminalState model (id/shell/title/exited/exit_code/cols/rows) implemented with new/set_exited/resize; TerminalService stub; 2 service tests + 2 integration tests; PTY spawn/read/write pending |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 4 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 13 |
| `M5 stale docs` | 0 |

## Update Rules

- after docs-only transition, mark implementation rows as `spec-only` or `unverified`
- close rows only with reproducible runtime evidence after reconstruction
- synchronize updates with `CONFORMANCE`, `LIMITATIONS`, and `/docs/todo/`

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
