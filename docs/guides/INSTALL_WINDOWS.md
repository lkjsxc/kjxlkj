# Windows Installation

Back: [/docs/guides/README.md](/docs/guides/README.md)
Ways to run kjxlkj on Windows.

For current supported behavior and known gaps, check:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

If this repository is in a docs-only baseline (no `src/` or Cargo workspace), reconstruct first:

- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Pre-built binaries

Release availability depends on the active reconstructed state and release process.

See:

- [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Build from source (recommended)

1. Install the Rust toolchain (stable).
2. Build with `cargo build`.
3. Run with `cargo run`.

If you want `kjxlkj` available globally, add the built binary directory to your `PATH`.

## Terminal recommendations

kjxlkj is a terminal UI app; terminal choice matters.

Recommended:

- Windows Terminal
- WezTerm
- Alacritty

## Troubleshooting

Common issues and solutions on Windows.

### Arrow keys or modifiers not working

Try a different terminal emulator. Some environments do not report all key events consistently.

### Colors look wrong

Ensure the terminal is configured for at least 256 colors.
