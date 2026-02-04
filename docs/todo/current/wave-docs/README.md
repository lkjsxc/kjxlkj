# Wave: Docs and TODO Repair (Iteration 34)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Make the repository’s “docs as source of truth” workflow reliable by:

- removing policy violations
- making derived-artifact status explicit (so docs do not claim missing files exist)
- ensuring the TODO system is actionable and checkmark-verifiable

## Checklist

### A. Enforce documentation policy

- [x] Remove all non-Mermaid fenced blocks under `/docs/`.
- [x] Ensure documentation structure constraints are satisfied (≤200 lines per file; ≤12 direct children per directory; every directory has exactly one `README.md`).
- [x] Ensure internal documentation links do not use `../`.

### B. Reconcile derived automation artifacts (CI/Docker/toolchain)

- [x] Clarify that CI, Docker support, and toolchain pinning are derived artifacts that may be absent in a docs-only baseline.
- [ ] Ensure all guides/reference docs describe derived artifacts conditionally (no unconditional “this repo includes X” unless the artifact is guaranteed present).

### C. Fix conformance docs vs implementation surface

- [x] Remove non-compliant fences from conformance docs and keep headless script schema accurate.
- [ ] Audit conformance statements that materially affect user expectations (modes, newline handling, long-line behavior, performance posture) and record any known drift in limitations.

### D. Redesign the current TODO iteration for real execution

- [x] Add a normative completion gate command to the TODO index.
- [ ] Convert non-checklist “Tasks” sections in the current iteration into checkboxes so completion is machine-checkable.
- [ ] Uncheck any items that are not demonstrably complete (avoid pre-checked “future” items).
- [ ] Add leaf TODOs for the reported high-priority gaps:
  - long-line rendering stability
  - interactive Insert-mode newline reliability
  - CPU/latency regressions vs baseline expectations
  - large-file open/scroll responsiveness
  - file explorer MVP (spec exists; implementation missing)

### E. Record work products

- [ ] Add a structured proposals/log area under `/docs/log/` and record the above gaps with links to defining specs, conformance claims, and test requirements.

