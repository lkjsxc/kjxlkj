# Reconstruction Bootstrap

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Purpose

Deterministic scaffold guide for rebuilding typed runtime artifacts from canon.

## Relevant Canonical Inputs

- [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/todo/README.md](/docs/todo/README.md)

## Baseline Assumption

Current repository state may be docs-only with no runtime artifacts.

## Phase 1: Root Scaffold

1. Create derived runtime manifests:
   - `Cargo.toml`
   - `Cargo.lock`
2. Keep root compliant with [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md).

## Phase 2: Typed Topology

1. Create backend workspace root: `src/backend/crates/`
2. Create frontend root: `src/frontend/`
3. Scaffold required backend crates from [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md).
4. Ensure frontend source uses `.ts`/`.tsx` only.

## Phase 3: Manifest Wiring

1. Configure Rust workspace members and shared dependencies.
2. Configure TypeScript strict settings (`strict`, `noImplicitAny`, `allowJs: false`).
3. Register typed API/WS contract packages if shared.

## Phase 4: Per-Wave Gates

For every wave completion attempt:

1. run `cargo build --workspace`
2. run `cargo test --workspace`
3. run wave-specific acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Phase 5: Ledger Sync

1. Update `/docs/reference` ledgers.
2. Then and only then mark TODO checkboxes.

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Evidence map: [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
