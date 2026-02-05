# Wave: Docs and TODO Repair (Iteration 36)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Make the documentation and TODO workflow reliable by:

- removing policy violations
- making derived-artifact status explicit (so docs do not claim missing files exist)
- ensuring the TODO system is actionable and checkmark-verifiable

Contract reference: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)

## Checklist

### A. Enforce documentation policy

- [ ] Remove all non-Mermaid fenced blocks under `/docs/`.
- [ ] Ensure documentation structure constraints are satisfied (≤200 lines per file; ≤12 direct children per directory; every directory has exactly one `README.md`).
- [ ] Ensure internal documentation links do not use `../`.

### B. Reconcile derived automation artifacts (CI/Docker/toolchain)

- [ ] Clarify that CI, Docker support, and toolchain pinning are derived artifacts that may be absent in a docs-only baseline.
- [ ] Reduce the repository to a docs-only baseline by deleting derived implementation artifacts (`src/`, `Cargo.toml`, `Cargo.lock`) before a full reconstruction run.
- [ ] Ensure all guides/reference docs describe derived artifacts conditionally (no unconditional "this repo includes X" unless the artifact is guaranteed present).

### C. Fix conformance docs vs implementation surface

- [ ] Remove non-compliant fences from conformance docs and keep headless script schema accurate.
- [ ] Audit conformance statements that materially affect user expectations (modes, newline handling, long-line behavior, performance posture) and record any known drift in limitations.

### D. Redesign the current TODO iteration for real execution

- [ ] Add a normative completion gate command to the TODO index.
- [ ] Convert non-checklist “Tasks” sections in the current iteration into checkboxes so completion is machine-checkable.
- [ ] Uncheck any items that are not demonstrably complete (avoid pre-checked “future” items).
- [ ] Add leaf TODOs for the reported high-priority gaps:
  - leader key and feature-chord reachability: [/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md](/docs/todo/current/wave-implementation/ux/keybindings/leader/README.md)
  - interactive PTY E2E harness + regressions: [/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md](/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md)
  - append (`a`) at end-of-line off-by-one: [/docs/todo/current/wave-implementation/editing/cursor/append-eol/README.md](/docs/todo/current/wave-implementation/editing/cursor/append-eol/README.md)
  - default soft-wrap behavior: [/docs/todo/current/wave-implementation/ui/viewport/wrap/README.md](/docs/todo/current/wave-implementation/ui/viewport/wrap/README.md)
  - long-line rendering stability: [/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md](/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md)
  - interactive Insert-mode newline reliability: [/docs/todo/current/wave-implementation/modes/insert/newline/README.md](/docs/todo/current/wave-implementation/modes/insert/newline/README.md)
  - CPU/latency regression harness: [/docs/todo/current/wave-implementation/technical/latency/regression/README.md](/docs/todo/current/wave-implementation/technical/latency/regression/README.md)
  - large-file open/scroll responsiveness: [/docs/todo/current/wave-implementation/technical/memory/README.md](/docs/todo/current/wave-implementation/technical/memory/README.md)
  - syntax language detection (C/C++ etc): [/docs/todo/current/wave-implementation/features/syntax/language-detection/README.md](/docs/todo/current/wave-implementation/features/syntax/language-detection/README.md)
  - file explorer MVP: [/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md](/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md)

### E. Record work products

- [ ] Add a structured proposals/log area under `/docs/log/` and record the above gaps with links to defining specs, conformance claims, and test requirements.
