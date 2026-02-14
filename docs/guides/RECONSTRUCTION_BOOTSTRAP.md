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

Current repository state may be docs-only canonical baseline with no runtime
artifacts.

## Phase 1: Root Scaffold

1. Create derived runtime manifests:
   - `Cargo.toml`
   - `Cargo.lock`
   - `package.json`
   - `tsconfig.json`
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

## Phase 4: Type and Compile Gate

Minimum deterministic checks before feature implementation:

1. backend compile: `cargo check --workspace`
2. frontend type-check: `tsc --noEmit`
3. no handwritten JavaScript runtime source (`dist/*.js` is generated output)

## Phase 5: Wave-Driven Implementation

1. Start with [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md).
2. Implement behavior in wave order only.
3. Update ledgers and TODO state in the same change.

## Related

- Quickstart: [QUICKSTART.md](QUICKSTART.md)
- Docker workflow: [DOCKER.md](DOCKER.md)
- Evidence map: [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
