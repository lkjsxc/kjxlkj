# Documentation Topology & Links (Iteration 34)

Back: [/docs/todo/current/wave-reading/README.md](/docs/todo/current/wave-reading/README.md)

## Objective

Bring documentation into full compliance with structure and link policies:

- No non-Mermaid fenced blocks under `/docs/`
- All docs reachable from [/docs/README.md](/docs/README.md)
- Directory size limits (≤ 12 direct children)
- Correct internal links without using `../`

## Tasks

### 1. Remove forbidden documentation fences

- [ ] Identify any non-Mermaid fenced blocks under `/docs/`.
- [ ] Replace forbidden fences with prose, tables, inline code spans, or Mermaid (when diagrammatic).

### 2. Normalize cross-directory link style

- [ ] Prefer repo-root absolute paths for cross-directory links when it reduces ambiguity:
  - Example pattern: `/docs/spec/README.md`
- [ ] Avoid `../` in documentation links.

### 3. Enforce directory constraints

- [ ] Ensure no directory under `/docs/` exceeds 12 direct children.
- [ ] Ensure every directory under `/docs/` contains exactly one `README.md`.
- [ ] Ensure no documentation file exceeds 200 lines.

### 4. Remove contradictions about repository contents

- [ ] Ensure docs consistently reflect the repository’s scope and derived artifacts (docs-only baseline vs shippable reconstructed state).
- [ ] Ensure CI/Docker/toolchain documentation does not claim absent artifacts are present.
- [ ] Use the plan: [/docs/todo/plan/repository-scope/README.md](/docs/todo/plan/repository-scope/README.md)

## Deep dives

| Area | Entry |
|------|-------|
| Repository scope consistency | [repo-scope/README.md](repo-scope/README.md) |
