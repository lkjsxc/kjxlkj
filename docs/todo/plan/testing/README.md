# Plan: Testing and Verification

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order

### 1. Test taxonomy

1. Unit tests: pure state transitions and text operations
2. Integration tests: async message ordering and cancellation
3. Golden UI tests: snapshot-to-frame determinism for critical views
4. E2E tests: terminal-driven editor operation flows

### 2. Cursor/viewport regression suite

1. Cursor visibility tests across modes and themes
2. Viewport follow tests across scroll offsets and splits
3. Resize-storm tests for stability and responsiveness

### 3. Input latency and throughput suite

1. No “one-key lag” scenarios under rapid typing
2. Backpressure behavior that preserves correctness
3. Performance thresholds and regression gating

### 4. Terminal feature suite

1. PTY lifecycle and crash handling
2. Scrollback navigation and rendering under load
3. Input routing and mode transitions
