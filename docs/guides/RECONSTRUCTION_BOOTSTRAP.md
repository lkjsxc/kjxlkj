# Reconstruction Bootstrap

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Purpose

Deterministic scaffold guide for rebuilding source artifacts from docs-only baseline.

## Relevant Canonical Inputs

- [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Baseline Assumption

Current repository state is docs-only. This guide defines the minimum bootstrap that
must exist before stage-wave implementation work can begin.

## Phase 1: Root Scaffold

1. Create root derived files:
   - `Cargo.toml`
   - `Cargo.lock`
   - `.dockerignore`
   - `Dockerfile`
   - `docker-compose.yml`
2. Ensure root paths stay compliant with
   [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md).
3. Keep all newly created docs and source files under 200 lines by design.

## Phase 2: Workspace Topology

1. Create workspace root: `src/crates/`.
2. Create canonical group roots:
   - `src/crates/app/`
   - `src/crates/core/`
   - `src/crates/platform/`
   - `src/crates/services/`
3. Scaffold all required crate directories from
   [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md).
4. Add crate-local `Cargo.toml` and minimal `lib.rs`/`main.rs` entry points.

## Phase 3: Manifest Wiring

1. Set workspace resolver to `2`.
2. Set workspace package edition to `2021`.
3. Register all canonical members from crate topology docs.
4. Add shared dependencies listed in
   [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md).

## Phase 4: Compile-Ready Gate

Minimum deterministic checks before feature implementation:

1. `cargo metadata` succeeds.
2. `cargo check --workspace` succeeds.
3. No source file exceeds 200 lines.
4. Directory fan-out remains around 12 direct children.

## Phase 5: Wave-Driven Implementation

1. Start with [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md).
2. Implement behavior in wave order only.
3. For each wave:
   - implement user-reachable behavior
   - run mapped tests from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
   - update ledgers and TODO state in the same change

## Completion Rule

Do not mark implementation wave tasks complete until deterministic evidence exists
and linked ledgers are synchronized.

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Docker workflow: [DOCKER.md](DOCKER.md)
- Evidence map: [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
