# Windows Installation

Back: [/docs/guides/README.md](/docs/guides/README.md)
Ways to run kjxlkj on Windows.

## Pre-built binaries

This repository does not currently publish tagged releases. Build from source.

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

### Arrow keys or modifiers not working

Try a different terminal emulator. Some environments do not report all key events consistently.

### Colors look wrong

Ensure the terminal is configured for at least 256 colors.
