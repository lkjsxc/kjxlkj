# Policy

Back: [/docs/README.md](/docs/README.md)
Operating invariants and agent rules.

## “All in Docs” (normative)

This repository follows an **“All in Docs”** contract:

- Documentation under `/docs/` is **normative** and **sufficient to reconstruct the system**.
- Implementation artifacts (code, build files) are **derived outputs** and are not authoritative.
- Any divergence between docs and implementation MUST be resolved by updating docs and/or recording conformance/limitations in `/docs/reference/`.

Canonical explanation: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md).

This documentation is also optimized for machine (LLM) parsing and editing.

## Documents

| Document | Purpose |
|----------|---------|
| [INSTRUCT.md](INSTRUCT.md) | **READ FIRST** - Operating contract |
| [WORKFLOW.md](WORKFLOW.md) | Workflow rules and quality standards |
| [STRUCTURE.md](STRUCTURE.md) | Directory and file structure constraints |
| [ROOT_LAYOUT.md](ROOT_LAYOUT.md) | Root-level allowlist and layout constraints |
| [Documentation fences (Mermaid-only)](#documentation-fences-mermaid-only-normative) | Documentation content compliance rule |

## Key Invariants

1. **Single Binary Runtime** - No plugins, all features native Rust
2. **No Mouse Support** - Keyboard-only interaction
3. **Two-Phase Execution** - Plan first, implement second
4. **File Size Limits** - Source ≤200 lines, Docs ≤200 lines
5. **Documentation Topology** - Every directory has one README.md
6. **Directory Size Limits** - Max 12 direct children per directory

## Canonical spec set

spec documentation is canonical:

- [docs/spec/README.md](/docs/spec/README.md)

This repository includes both:

- canonical specification under `/docs/` (normative)
- a Rust implementation under `/src/` that is expected to track `/docs/`

## Concurrency baseline
The canonical architecture is **Tokio async-first**:

- IO and long-running compute run in supervised services.
- The editor core remains a single-writer task.
- Rendering consumes snapshots only.

## Quick Reference

| Rule | Description |
|------|-------------|
| Two-Phase | Plan & docs first, code second |
| Response Format | Intent, Plan, Impact, Tests, Commits, Risks |
| Commits | `type(area): summary` format |
| Line Limit | 200 lines per file |
| Directory Rule | Every dir has exactly one README.md |

See individual documents for complete details.

## Documentation fences (Mermaid-only) (normative)

This repository treats documentation under [docs/](/docs/README.md) as **normative, code-free specification**.

- Documentation under [docs/](/docs/README.md) **MUST NOT** contain any fenced code blocks.
- The **only** exception is a Mermaid diagram fenced block (a fenced block whose info string is exactly `mermaid`).
- This prohibition is unconditional: it applies to all non-Mermaid fences, including command lines, config, pseudo-code, data samples, and ASCII art.

Rationale: embedded snippets become stale and re-introduce undocumented coupling between spec and implementation.

#### Allowed alternatives

Documentation under [docs/](/docs/README.md) **MUST** express behavior using one or more of:

- Prose with precise MUST/SHOULD language
- Tables (types, fields, invariants, inputs/outputs, state transitions)
- Inline code spans (for identifiers, short tokens, and literals)
- Links to canonical spec documents, or to source-of-truth files in an implementation repository (when present)
- Mermaid diagrams

#### Migration guidance (replace existing fences)

When converting existing documentation:

- ASCII diagrams **SHOULD** become Mermaid diagrams.
- Code sketches **SHOULD** become tables:
  - entity/type name
  - fields and types
  - invariants and constraints
  - state transitions (as rows)
- Command-line examples and config excerpts **SHOULD** become:
  - prose describing intent and constraints
  - inline code spans for key tokens
  - links to the canonical file or module that defines the behavior
- Usage examples **SHOULD** become acceptance criteria in Given/When/Then form.

#### PR compliance checklist (review gate)

For any PR that changes files under docs/:

- [x] All fenced blocks are Mermaid-only (no non-Mermaid fences; no unlabeled fences; no tilde fences).
- [x] New behavior is specified as MUST/SHOULD statements, tables, or acceptance criteria.
- [x] Implementation-relevant detail is linked to a canonical spec document or an implementation source-of-truth (no embedded duplication).
- [x] The doc remains ≤ 200 lines unless an explicit exception is recorded in policy.

### "Remove all source code" interpretation (this documentation phase)

For this documentation-reform task, **remove all source code means**:

- Remove embedded source-code snippets from documentation.
- Do not add embedded code excerpts as fenced blocks under `/docs/`.

If a future task introduces an implementation repository, record its scope and verification rules as a separate policy decision.
