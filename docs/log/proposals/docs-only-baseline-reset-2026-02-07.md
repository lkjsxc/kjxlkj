# Proposal: Docs-Only Baseline Reset (2026-02-07)

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Summary

Prepare the repository for a clean reconstruction iteration by resetting derived implementation artifacts to a docs-only baseline.

## Rationale

- User requested a hard reset of source artifacts after documentation hardening.
- Current docs now include stricter anti-gaming reconstruction contract and evidence gates.
- A clean baseline reduces accidental carry-over from potentially low-quality generated code.

## Defining references

- Reconstruction prompt: [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)
- Root layout policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Workflow policy: [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md)
- Conformance authority: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations authority: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Planned artifact reset

Remove derived implementation artifacts while preserving canonical docs:

- `src/`
- `Cargo.toml`
- `Cargo.lock`
- `.github/`
- `Dockerfile`
- `.dockerignore`
- `rust-toolchain.toml`
- `target/`

## TODO linkage

Next implementation should start from:

- [/docs/todo/current/wave-reconstruction/README.md](/docs/todo/current/wave-reconstruction/README.md)
- [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)

## Acceptance criteria

- Repository retains canonical docs and navigation integrity.
- Derived implementation artifacts above are removed.
- A reconstruction agent can start from docs-only baseline and follow the hardened prompt.

## Test strategy

- Before reset: run docs policy check and capture pass evidence.
- After reset: verify docs tree remains intact and root reflects docs-only baseline.

## User-visible impact

- Local build/test commands are unavailable until reconstruction regenerates workspace artifacts.
- Documentation remains the source of truth and reconstruction entrypoint.
