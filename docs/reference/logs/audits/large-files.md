# Source Files Over 200 Lines

Back: [/docs/reference/logs/audits/README.md](/docs/reference/logs/audits/README.md)

Audit date: `2026-02-16`.

Scope: runtime source files only (`src/**`), excluding dependency/build artifacts (`node_modules`, `dist`).

## Current Over-200 Set

| Path | Lines | Action |
|---|---:|---|
| `src/frontend/app/src/components/app-shell.ts` | 422 | split UI shell into focused components (`IMP-FE-01`, `IMP-STRUCT-01`) |
| `src/crates/http/kjxlkj-http/src/routes_note.rs` | 306 | split route handlers by note sub-domain (`IMP-STRUCT-01`) |
| `src/crates/db/kjxlkj-db/src/repo_note.rs` | 302 | split repository operations by command/query (`IMP-STRUCT-01`) |
| `src/crates/ws/kjxlkj-ws/src/session.rs` | 229 | split actor/session protocol responsibilities (`IMP-STRUCT-01`) |
| `src/crates/db/kjxlkj-db/src/repo_automation.rs` | 205 | split rule/run persistence concerns (`IMP-STRUCT-01`) |

## Policy Mapping

This audit satisfies the rule in [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) to record >200-line source exceptions and backlog refactor tasks.
