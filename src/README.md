# Implementation

The Rust implementation is a Cargo workspace rooted at `Cargo.toml`.

Crates live under `src/crates/` and are grouped by concern:

- `src/crates/app/` - shipped binary
- `src/crates/core/` - deterministic core model and editing
- `src/crates/host/` - terminal lifecycle glue
- `src/crates/input/` - input decoding and mapping
- `src/crates/render/` - snapshot rendering
- `src/crates/services/` - async services (FS, terminal, etc)

