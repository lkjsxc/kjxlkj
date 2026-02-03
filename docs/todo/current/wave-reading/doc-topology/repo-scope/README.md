# Repository Scope Consistency (Iteration 33)

Back: [/docs/todo/current/wave-reading/doc-topology/README.md](/docs/todo/current/wave-reading/doc-topology/README.md)

## Objective

Remove contradictions where docs assume an in-repo implementation exists when it does not.

## Checklist

### A. Root-level identity

- Ensure `/README.md` describes this repository’s artifacts accurately.

### B. Policy consistency

- Ensure `/docs/policy/README.md` does not claim that `src/` exists in this repository.
- Ensure `/docs/policy/WORKFLOW.md` and `/docs/policy/INSTRUCT.md` describe verification steps that exist in this repository, while still specifying expectations for an external Rust implementation.

### C. Spec consistency

- Ensure `/docs/spec/architecture/crates.md` is framed as intended implementation topology rather than a statement about current repo contents.
- Remove or reframe references to `Cargo.toml` and `src/` where they are treated as present artifacts.

### D. Verification

- All internal links MUST resolve.
- No new contradictions about repository artifacts are introduced.
