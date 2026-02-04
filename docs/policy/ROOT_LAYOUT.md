# Root Layout Policy

This document defines the allowed top-level layout and the required implementation placement.

## Root allowlist

The repo root SHOULD contain only the following entries:

| Path | Purpose |
|---|---|
| `README.md` | Project index and pointers |
| `LICENSE` | License |
| `docs/` | Canonical specification and policies |
| `src/` | Rust implementation workspace |
| `Cargo.toml` | Workspace manifest |
| `Cargo.lock` | Locked dependencies |
| `.gitignore` | Git ignore rules |
| `rust-toolchain.toml` | Toolchain pinning for reproducible builds and CI |
| `.github/` | CI configuration (GitHub Actions) and automation (Dependabot) |
| `Dockerfile` | Docker image definition |
| `.dockerignore` | Docker build context filtering |

Additional root entries MUST be justified by a concrete need and recorded in:

- `/docs/reference/IMPLEMENTATION_HISTORY.md`

## Implementation layout

| Path | Requirement |
|---|---|
| `src/crates/` | Workspace crates MUST live here |
| `src/crates/kjxlkj/` | The single shipped binary crate MUST live here |
