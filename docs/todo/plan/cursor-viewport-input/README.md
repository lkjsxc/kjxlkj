# Plan: Cursor, Viewport, and Input

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order

### 1. Cursor movement semantics

1. Reconcile the column model (end-exclusive vs end-inclusive) across modes
2. Specify clamping and no-op rules for every motion boundary condition
3. Specify how wrapped lines and graphemes affect movement

### 2. Cursor rendering and visibility

1. Specify how the cursor is rendered in each mode
2. Specify how cursor rendering composes with highlights and overlays
3. Specify invariants: cursor must always be visible when editor focus is active

### 3. Viewport follow behavior

1. Specify vertical follow rules (scroll offsets, centering, top/bottom align)
2. Specify horizontal follow rules (wrap vs no-wrap, side scroll)
3. Specify split-local viewport invariants and cursor visibility guarantees

### 4. Input ordering and latency

1. Specify event → intent → state → snapshot → render ordering
2. Specify coalescing and backpressure rules that do not introduce key-lag
3. Specify performance targets and observability requirements

### 5. Regression acceptance criteria

1. Define Given/When/Then tests for cursor visibility
2. Define Given/When/Then tests for viewport follow and scroll
3. Define Given/When/Then tests for input latency and throughput
