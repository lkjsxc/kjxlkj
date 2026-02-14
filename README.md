# kjxlkj

All in Docs workspace specification.

## Current Repository State (2026-02-14)

- Documentation under `docs/` is the only canonical product value.
- Runtime source and build artifacts are intentionally absent.
- Rebuild work starts from `docs/todo/` and reconstructs disposable runtime artifacts.
- Any generated runtime implementation must satisfy typed constraints:
  - backend runtime source: Rust
  - frontend runtime source: TypeScript (`strict`)
  - handwritten JavaScript runtime source is forbidden

## Canonical Reading Order

1. `docs/policy/README.md`
2. `docs/spec/README.md`
3. `docs/reference/README.md`
4. `docs/todo/README.md`
5. `docs/guides/README.md`

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical product definition |
| `AGENTS.md` | local execution policy for coding agents |
| `GEMINI.md` | alternate agent policy mirror |
| `README.md` | repository entry point |
| `LICENSE` | license text |
| `.gitignore` | repository hygiene |
| `.github/` | optional workflow metadata |
