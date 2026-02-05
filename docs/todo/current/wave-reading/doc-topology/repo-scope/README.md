# Repository Scope Consistency (Iteration 34)

Back: [/docs/todo/current/wave-reading/doc-topology/README.md](/docs/todo/current/wave-reading/doc-topology/README.md)

## Objective

Remove contradictions where documentation:

- claims derived artifacts exist unconditionally (CI/Docker/toolchain), and/or
- fails to distinguish a docs-only baseline from a shippable reconstructed repository state.

## Checklist

### A. Root-level identity

- [x] Ensure `/README.md` describes this repository’s artifacts accurately.

### B. Policy consistency

- [x] Ensure `/docs/policy/README.md` and `/docs/policy/ROOT_LAYOUT.md` distinguish “docs-only baseline” vs “shippable state” for derived artifacts.
- [x] Ensure `/docs/policy/WORKFLOW.md` describes a verification gate that can be reconstructed and run locally, and is mirrored by automated CI when present.

### C. Spec consistency

- [x] Ensure `/docs/spec/architecture/crates.md` and related architecture docs are framed as the reconstruction target topology (not a claim that every artifact is always present).
- [x] Remove or reframe references to `Cargo.toml` and `src/` where they are treated as non-deletable artifacts (they are derived outputs).

### D. Verification

- [x] All internal links resolve.
- [x] No new contradictions about repository artifacts are introduced.
