# Plan: Repository Scope and Artifacts

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Goal

Ensure documentation does not contradict repository contents while remaining a complete specification.

## Implementation Order

### 1. Define repository scope (docs-only vs implementation)

1. Ensure the root README states what artifacts exist in this repository.
2. Ensure policy docs consistently describe whether an in-repo implementation exists.
3. Ensure any references to `Cargo.toml` or `src/` are either:
   - updated to match repository contents, or
   - explicitly framed as the expected layout of an external implementation.

### 2. Remove or reframe file-path references

1. Replace references to nonexistent files with:
   - links to normative spec documents, or
   - explicit “implementation layout” statements.

### 3. Verification rules

1. Link validation MUST pass for all real links.
2. Example syntax that looks like links MUST be expressed using inline code spans.
3. The documentation MUST remain navigable from `/docs/README.md`.
