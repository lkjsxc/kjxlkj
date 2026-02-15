# Audit Log — 2026-02-15

Back: [README.md](README.md)

## Decisions

- Using source-layout.md / final-file-structure.md 10-crate layout (not crates.md
  20-crate legacy editor layout which is out-of-scope per policy README)
- Using argon2 for password hashing (memory-hard as required by auth.md)
- Using UUID v7 for IDs as recommended by types.md
- Single Docker container with PostgreSQL + app as required by deployment.md

## Deviations

- crates.md describes a legacy terminal-editor crate decomposition that conflicts
  with source-layout.md web-server decomposition; per spec conflict rule, the
  more specific leaf (source-layout.md) overrides
