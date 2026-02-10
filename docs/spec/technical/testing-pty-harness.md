# PTY E2E Harness Contract

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

Contract for the live PTY process harness used by `*R` tests.

## Purpose

The PTY harness runs the full kjxlkj binary inside a pseudo-terminal, sends raw key bytes, and captures rendered frames for assertion. This is the only harness level that can prove real input decoding paths.

## Architecture

```
┌───────────────────────────────────────┐
│           Test Driver (Rust)          │
│  - spawn kjxlkj in PTY via portable_pty│
│  - send raw key bytes                  │
│  - poll PTY output                     │
│  - parse frames and assert             │
└─────────────┬─────────────────────────┘
              │ PTY master fd
              ▼
┌───────────────────────────────────────┐
│         kjxlkj (child process)        │
│  - crossterm reads from PTY slave     │
│  - renders to PTY slave               │
└───────────────────────────────────────┘
```

## Crate

The harness lives in `kjxlkj-test-harness` crate under `src/crates/app/`.

## Public API

```rust
/// PTY E2E test harness
pub struct PtyHarness {
    pty: PtyPair,
    child: Child,
}

impl PtyHarness {
    /// Spawn kjxlkj in PTY with given size
    pub fn spawn(cols: u16, rows: u16) -> Result<Self>;
    
    /// Send raw key bytes (e.g., Shift+a = 'A')
    pub fn send_bytes(&mut self, bytes: &[u8]) -> Result<()>;
    
    /// Send key event (high-level)
    pub fn send_key(&mut self, key: &str) -> Result<()>;
    
    /// Wait for output containing pattern (with timeout)
    pub fn wait_for(&mut self, pattern: &str, timeout: Duration) -> Result<String>;
    
    /// Capture current frame as string grid
    pub fn capture_frame(&mut self) -> Result<Frame>;
    
    /// Exit gracefully
    pub fn quit(&mut self) -> Result<()>;
}

/// Captured terminal frame
pub struct Frame {
    pub lines: Vec<String>,
    pub cursor: (u16, u16),
}
```

## Key Encoding

The harness must correctly encode modifier keys:

| Key | Bytes |
|---|---|
| `a` | `[0x61]` |
| `A` (Shift+a) | `[0x41]` |
| `Ctrl-w` | `[0x17]` |
| `Esc` | `[0x1B]` |
| `Enter` | `[0x0D]` |
| `h` | `[0x68]` |
| Arrow keys | ANSI escape sequences |

## Timeout Policy

All harness operations use bounded timeouts with explicit error messages:

| Operation | Default Timeout | Diagnostic |
|---|---|---|
| spawn | 5s | startup failed |
| wait_for | 2s | pattern not found |
| capture_frame | 500ms | output stall |
| quit | 1s | graceful shutdown failed |

## Determinism Rules

- No blind sleep; use polling with deadlines
- Fixed PTY size for reproducibility
- Capture full diagnostics on failure:
  - current mode
  - focused window ID
  - cursor position
  - last 20 input events
  - frame excerpt

## Test Example

```rust
#[test]
fn wr_01r_shift_a_decode() {
    let mut h = PtyHarness::spawn(80, 24).unwrap();
    h.send_key("i").unwrap(); // Enter insert mode
    h.send_bytes(b"A").unwrap(); // Shift+a -> 'A'
    h.send_key("Esc").unwrap();
    
    let frame = h.capture_frame().unwrap();
    assert!(frame.lines[0].contains("A"));
    h.quit().unwrap();
}
```

## Dependencies

- `portable_pty` for cross-platform PTY
- `tokio` for async polling (optional)

## Fallback for CI

When PTY is unavailable (e.g., minimal containers), tests are marked `#[ignore]` with a note. The headless state harness provides baseline coverage.

## Related

- E2E test matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
