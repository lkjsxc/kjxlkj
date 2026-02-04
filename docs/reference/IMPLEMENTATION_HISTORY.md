# Implementation History (Summary)

Back: [/docs/reference/README.md](/docs/reference/README.md)
Historical context that used to live under `/docs/log/`.

## Purpose

Repository contract reference: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

The `/docs/log/` subtree is intentionally treated as optional history. If detailed dated logs are removed, this document preserves the minimum historical context that is useful for reconstruction and maintenance.

## High-level history

| Date | Summary |
|------|---------|
| 2026-02-03 | Implementation work expanded the editor from an initial Rust workspace scaffold into a broad Vim-like editing surface (modes, motions, operators, search, macros, marks, registers, several Ex commands). |
| 2026-02-04 | Large-file responsiveness work: viewport-bounded snapshots, event-driven rendering (no idle busy-loop), streaming file open into the text model; plus a regression fix for Normal-mode `a` at end-of-line to append correctly. |
| 2026-02-04 | Full reconstruction from docs (Iteration 33): Rebuilt entire 18-crate workspace from `/docs/spec/architecture/crates.md`. Created core types, text model, undo, editing primitives, mode handling, UI snapshots, state management, input conversion, terminal rendering, host lifecycle, services, and binary entry point. All crates compile; unit tests pass. Target: current surface per CONFORMANCE.md. |
| 2026-02-04 | Project tooling baseline: introduced `rust-toolchain.toml`, GitHub Actions CI, and Docker support (`Dockerfile`, `.dockerignore`) as derived artifacts aligned with `/docs/policy/ROOT_LAYOUT.md` and `/docs/reference/CI.md`. |
| 2026-02-04 | Tooling cleanup: removed unused `Dockerfile`, CI scripts, and the docs policy checker in order to keep the repository minimal; these artifacts remain part of the reconstruction target and should be regenerated when producing a shippable repository state. |

## Provenance note

The repository started as documentation-only.

An implementation was later generated via GitHub Copilot using Claude Opus 4.5 (or equivalent coding-capable models). This history is informative, not authoritative.

## Key implementation choices (historical)

These choices were recorded during early implementation work and are reflected in the current repository:

| Topic | Choice |
|-------|--------|
| Rust edition | 2021 |
| Terminal IO | Crossterm |
| Text storage | Rope (via `ropey`) |
| Undo model | Linear undo/redo stack |
| Runtime model (target) | Tokio async-first, supervised services, single-writer core, snapshot-based rendering |

The **normative** versions of these requirements live in `/docs/spec/` and `/docs/policy/`.

## Where “what exists now” is recorded

Do not treat this history as the current feature list.

Use these ledgers instead:

- Current surface: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- User-visible gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Historical test snapshot

The early implementation sessions reported a steadily growing unit/integration test suite (on the order of ~150 tests passing by the end of 2026-02-03).

The current test suite is authoritative; treat this only as historical context.

## Known historical hygiene issues

Early sessions recorded that some source files exceeded the repo’s 200-line guideline.

If you reconstruct the implementation from docs, structure modules so that both:

- documentation files stay ≤200 lines (as per `/docs/policy/STRUCTURE.md`)
- source files stay ≤200 lines (per repo policy)

## Related

- Contract reference: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
- Policy: [/docs/policy/README.md](/docs/policy/README.md)
