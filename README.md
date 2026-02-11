# kjxlkj

A Vim-compatible terminal text editor. Single native Rust binary, keyboard-driven, no external plugins.

## Documentation

All canonical system definitions live in [`docs/`](docs/README.md).

Source code and automation artifacts are derived outputs reconstructed from documentation.

## Quick Start

```sh
cargo build --release
./target/release/kjxlkj [file...]
```

## Build Requirements

- Rust stable (edition 2021)
- Tokio async runtime
- POSIX terminal (or Windows ConPTY)

## Project Structure

| Path | Purpose |
|---|---|
| `docs/` | Canonical specification and policies |
| `src/crates/` | Rust workspace crates |
| `Cargo.toml` | Workspace manifest |

## License

See [LICENSE](LICENSE).
