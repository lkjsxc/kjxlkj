# Improvement Ideas

Back: [/docs/log/README.md](/docs/log/README.md)

Ideas for future enhancement discovered during the drift-closure wave.

## Architecture

- **focus.rs rectangle approach**: The directional focus algorithm
  uses normalized [0,1] virtual rectangles computed from the layout
  tree. This generalizes to arbitrary nesting depths but may benefit
  from caching if layout rebuilds become frequent.

- **ImeComposition in core-state**: Currently `kjxlkj-input` is a
  new dependency of `kjxlkj-core-state`. Consider moving `ImeComposition`
  and `ImeState` to `kjxlkj-core-types` to reduce coupling.

- **Layout tree rebuild**: `rebuild_layout` always produces a flat
  `Horizontal(...)` after window close. A split-preserving rebuild
  using the existing tree structure (removing just the closed leaf)
  would preserve user-arranged layouts.

## Testing

- **focus.rs unit tests**: Add focused unit tests for the `find_focus`
  function with complex nested layout trees (3+ levels, mixed splits).

- **IME integration with crossterm**: The current IME routing is tested
  at the model level. End-to-end testing with actual crossterm
  `ime::ImeEvent` events would require terminal test harness support.

- **Property-based wrap tests**: The wrap algorithm could benefit from
  property-based testing (e.g., quickcheck) to verify invariants:
  no row exceeds `text_cols`, every grapheme appears in exactly one row.

## Performance

- **Rope snapshot clone**: `snapshot()` clones the entire `Rope` via
  `buf.snapshot_rope()`. For large buffers, consider using `Rope::clone()`
  with copy-on-write semantics (ropey's clone is O(1)) or passing
  rope references with lifetime bounds.

- **Grid diff rendering**: The current grid diff compares cell-by-cell.
  For large terminals, a row-hash approach could skip identical rows
  before doing cell-level comparison.

## Documentation

- **Doc-coverage verification**: 446 documentation files are tracked
  in `doc-coverage-{1..5}.md`. Systematic verification of each against
  implementation state remains a future-wave task.

- **Source-to-doc traceability**: Adding `/// # Spec` doc comments
  linking each module to its normative spec file would improve
  traceability for AI-driven reconstruction.
