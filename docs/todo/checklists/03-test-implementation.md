# Checklist 03: Test Implementation

Back: [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)

## Mandatory Test Specs

- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)

## Retained High-Signal Implemented Tests

- [ ] keep `key_mode_e2e` (`WR-01R`, `KEY-TRACE-*`) and upgrade assertions to screen-state checks
- [ ] keep `window_nav_e2e` and `window_nav_more_e2e` and add per-input frame expectations
- [ ] keep `explorer_terminal_paths_e2e` and upgrade to visible panel/layout assertions
- [ ] keep `cursor_wrap_e2e` and `cursor_wrap_more_e2e` as wrap/cursor baseline suites
- [ ] keep `explorer_terminal_stress_e2e` and preserve race coverage

## New Professional-Grade E2E Tests

- [ ] `KEY-SCREEN-01`: launch editor, send `Shift+a`, dump frame after each key, assert append-at-EOL pixels/text
- [ ] `KEY-SCREEN-02`: compare physical `A` vs `Shift+a` traces and visible frame equality
- [ ] `WIN-SCREEN-01`: for each split command, assert deterministic pane geometry map after each step
- [ ] `WIN-SCREEN-02`: replay same split script twice and assert byte-identical frame timeline
- [ ] `EXP-SCREEN-01`: run `:Explorer` and `<leader>e`; assert explorer pane visibility and focus marker
- [ ] `EXP-SCREEN-02`: open targets (`Enter`, `v`, `s`) and assert active pane path and cursor location
- [ ] `MIXED-SCREEN-01`: interleave split, explorer, terminal, and resize; assert no pane disappears unexpectedly
- [ ] `BD-RACE-05`: under terminal flood plus explorer refresh, assert frame stability and focus uniqueness

## Boundary and Oracle Requirements

- [ ] every blocker E2E captures state dump after each key input
- [ ] every blocker E2E asserts expected screen rows, cursor cell, focused pane, and layout summary
- [ ] every blocker E2E stores deterministic failure artifacts (trace + frame excerpt + layout tree)

## Exit to Next Checklist

- [ ] continue to [04-verification-and-ledgers.md](04-verification-and-ledgers.md)
