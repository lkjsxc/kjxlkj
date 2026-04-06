# Documentation Canon

`docs/` is the only active canon for intended behavior, operator workflow, and repository rules.

## Global Rules

1. Keep one canonical statement for each rule.
2. Keep every docs directory to one `README.md` plus multiple children.
3. Keep every docs file at 300 lines or fewer.
4. Keep every source file at 200 lines or fewer.
5. Record exact defaults, route shapes, commands, and UI labels when they matter.
6. Remove stale contracts instead of preserving conflicting versions.
7. Prefer short declarative bullets over narrative prose.
8. Optimize for LLM retrieval before human ornament.

## Start Paths

- First-time mixed operator + contributor: [getting-started/README.md](getting-started/README.md)
- Fastest compose boot: [getting-started/compose-quickstart.md](getting-started/compose-quickstart.md)
- First live login and note creation: [getting-started/first-session.md](getting-started/first-session.md)
- Full verification bundle: [getting-started/verification.md](getting-started/verification.md)

## Top-Level Sections

- [getting-started/README.md](getting-started/README.md): orientation, quickstart, verification, and operator handoff
- [vision/README.md](vision/README.md): product intent, constraints, and LLM-first documentation rules
- [product/README.md](product/README.md): surface, behavior, and experience contracts
- [architecture/README.md](architecture/README.md): runtime, data, and integration contracts
- [operations/README.md](operations/README.md): deployment, verification, quality, and automation runbooks
- [repository/README.md](repository/README.md): repo layout, authoring rules, and change workflow

## Recommended Reading Order

1. [getting-started/orientation.md](getting-started/orientation.md)
2. [vision/purpose.md](vision/purpose.md)
3. [repository/layout/root-layout.md](repository/layout/root-layout.md)
4. [repository/layout/src-layout.md](repository/layout/src-layout.md)
5. [architecture/runtime/stack.md](architecture/runtime/stack.md)
6. [architecture/data/id-rules.md](architecture/data/id-rules.md)
7. [architecture/data/alias-rules.md](architecture/data/alias-rules.md)
8. [product/surface/routes.md](product/surface/routes.md)
9. [product/behavior/saved-snapshots.md](product/behavior/saved-snapshots.md)
10. [operations/deployment/single-host-compose.md](operations/deployment/single-host-compose.md)
11. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
12. [operations/quality/gates.md](operations/quality/gates.md)
