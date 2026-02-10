# Root Layout Policy

This document defines allowed top-level layout and implementation placement.

## Root Allowlist

The repo root SHOULD contain only:

| Path | Purpose |
|---|---|
| `README.md` | project index |
| `LICENSE` | license |
| `docs/` | canonical specification and policies |
| `src/` | Rust implementation workspace |
| `Cargo.toml` | workspace manifest |
| `Cargo.lock` | locked dependencies |
| `.gitignore` | ignore rules |
| `rust-toolchain.toml` | toolchain pinning |
| `.github/` | CI and automation |
| `Dockerfile` | container image definition |
| `.dockerignore` | docker context filtering |

Additional root entries require explicit rationale in `/docs/log/proposals/`.

## Tracked vs Untracked

This allowlist governs tracked entries.

Derived artifacts (for example `target/`) MUST NOT be tracked.

## Docs-Only Baseline

A docs-only reconstruction baseline may intentionally contain only:

- `docs/`
- `README.md`
- `LICENSE`
- minimal dotfiles required for repository hygiene

Derived automation artifacts may be temporarily absent and regenerated later.

## Implementation Layout

| Path | Requirement |
|---|---|
| `src/crates/` | workspace crates live here |
| `src/crates/app/kjxlkj/` | shipped binary crate path |
| `src/crates/core/` | core domain crates |
| `src/crates/platform/` | platform/runtime crates |
| `src/crates/services/` | service integration crates |

## Related

- Structure constraints: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Workspace members: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
