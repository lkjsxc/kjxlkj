# Improvement Proposals (2026-02-03)

Back: [/docs/log/2026/02/03/README.md](/docs/log/2026/02/03/README.md)

## Proposals

| Proposal | Motivation | Follow-up |
|---|---|---|
| Add a root layout allowlist policy | The policy graph references “Root Layout (Allowlist Only)” but no document defines it. | Done: `docs/policy/ROOT_LAYOUT.md` added and linked from `docs/policy/README.md`. |
| Add automated checks for file size limits | Enforce “≤200 lines per file” for sources and docs. | Add a repo-local verifier test (line limits, link rules, fence rules) and run it in CI/tests. |
| Add headless scripting for E2E tests | E2E should exercise real key-driven flows without flaky terminal scraping. | Done: `kjxlkj --headless --script` accepts a JSON key stream; `cargo test -p kjxlkj` includes a headless write/quit test. |
