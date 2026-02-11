# Reconstruction Audits

Back: [/docs/log/reconstruction/README.md](/docs/log/reconstruction/README.md)

## Active Audit Records

| ID | Date | Scope | Status |
|---|---|---|---|
| `AUD-2026-02-11-BASELINE-01` | 2026-02-11 | baseline reconstruction for workspace and grouped crate topology | open |

## Evidence Snapshot

| Check | Result |
|---|---|
| `src` presence before wave | absent |
| root `Cargo.toml` presence before wave | absent |
| root `Cargo.lock` presence before wave | absent |
| `rust-toolchain.toml` presence before wave | absent |

## Improvement Ideas

| ID | Idea | Rationale | Target |
|---|---|---|---|
| `IDEA-BASELINE-LOG-01` | persist per-wave evidence rows under reconstruction audits | keeps closure evidence local to active wave | current wave |
| `IDEA-BASELINE-LOG-02` | add topology and file-size audit checklist links in each wave record | prevents missing `<=200` source file checks | phase-5 hardening |
