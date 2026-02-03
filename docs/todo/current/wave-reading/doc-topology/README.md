# Documentation Topology & Links (Iteration 33)

Back: [/docs/todo/current/wave-reading/README.md](/docs/todo/current/wave-reading/README.md)

## Objective

Bring documentation into full compliance with structure and link policies:

- No non-Mermaid fenced blocks under `/docs/`
- All docs reachable from [/docs/README.md](/docs/README.md)
- Directory size limits (≤ 12 direct children)
- Correct internal links without using `../`

## Tasks

### 1. Remove forbidden documentation fences

- Identify any non-Mermaid fenced blocks under `/docs/`
- Replace with prose, tables, inline code spans, or Mermaid (when diagrammatic)

### 2. Normalize cross-directory link style

- Prefer repo-root absolute paths for cross-directory links:
  - Example pattern: `/docs/spec/README.md`
- Avoid `../` in documentation links.

### 3. Enforce directory constraints

- Reduce `docs/spec/` to ≤ 12 direct children
- Reduce `docs/spec/editing/motions/` to ≤ 12 direct children

### 4. Remove contradictions about repository contents

- Ensure docs consistently reflect this repository’s scope and artifacts.
- If implementation code is not present here, docs MUST say so unambiguously.
- Use the plan: [/docs/todo/plan/repository-scope/README.md](/docs/todo/plan/repository-scope/README.md)

## Deep dives

| Area | Entry |
|------|-------|
| Repository scope consistency | [repo-scope/README.md](repo-scope/README.md) |
