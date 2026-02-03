# Improvement Proposals (2026-02-03)

Back: [/docs/log/2026/02/03/README.md](/docs/log/2026/02/03/README.md)

## Proposals

| Proposal | Motivation | Follow-up |
|---|---|---|
| Add a root layout allowlist policy | The policy graph references “Root Layout (Allowlist Only)” but no document defines it. | Add `docs/policy/ROOT_LAYOUT.md` and keep it updated with implementation directories. |
| Add automated checks for file size limits | Enforce “≤200 lines per file” for sources and docs. | Add a small repo-local checker and run it in CI/tests. |

