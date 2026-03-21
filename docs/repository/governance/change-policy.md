# Change Policy Contract

## Required for Docs Changes

1. Update affected directory TOCs.
2. Keep links valid and relative.
3. Re-run topology and line-limit checks for touched docs.

## Required for Behavior Changes

- Update vision, product, architecture, operations, and container contracts consistently.
- Keep canonical definitions singular; replace duplicates with links.

## Contract-First Root Change Rule (Normative)

Repository root and runtime/container entry changes MUST follow docs-first sequencing.

Required sequence:

1. Define or update contracts in `docs/repository/` and `docs/containers/` first.
2. Update TOCs and cross-links so retrieval paths remain deterministic.
3. Run validation checks (`docs validate-topology`, `quality check-lines`, and touched-link sanity).
4. Apply root/runtime/container entry changes in a dedicated todo.
5. Re-run validation checks and confirm alignment with [../structure/root-layout.md](../structure/root-layout.md) and container compose contracts.

If steps 1-3 are incomplete, root/runtime change work is non-compliant and MUST stop.

## Deterministic Root-Change Readiness Checklist

Mark each item true before changing root/runtime entries:

1. Document-first authority remains explicit.
2. Persistent-runtime root classes are documented in [../structure/root-layout.md](../structure/root-layout.md).
3. Container contracts still enforce prebuilt app image flow, `./data` mount rooting, `./data/postgres`, verify opt-in, and image-baked migrations preference.
4. All links in touched docs resolve.
5. Topology check passes.
6. Line-limit check passes.
