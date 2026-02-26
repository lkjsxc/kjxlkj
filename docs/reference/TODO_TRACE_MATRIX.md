# TODO Trace Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonical traceability ledger from TODO checkboxes to authoritative docs, expected implementation artifacts, and evidence targets.

## Rules

- Every TODO checkbox MUST link to authoritative documentation.
- Every TODO wave MUST declare expected implementation artifacts.
- Every completed TODO checkbox MUST map to evidence rows in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md).

## Wave-to-Artifact Map

| Wave | Primary Docs | Expected Implementation Artifacts | Required Acceptance IDs | Evidence Status |
|---|---|---|---|---|
| `S00/W000-002` | policy + structure + root layout | governance checks, link audits, policy enforcement scripts | docs-integrity | `pending` |
| `S01/W010-012` | crates + workspace-manifest + migrations | `Cargo.toml`, crate manifests, regenerated `migrations/*.sql` | build/type gates | `pending` |
| `S02/W020-022` | notes + events + search | note services, event store, search pipeline | `API-NOTE-*`, `API-SEARCH-*` | `pending` |
| `S03/W030-032` | web-app + api/http + api/errors | app shell routing, HTTP route handlers, conflict/error wiring | communication integration pack | `pending` |
| `S04/W040-042` | automation + agent + prompt JSON | agent loop, prompt validator, KV memory store | `API-AUTO-*`, `AGENT-*` | `pending` |
| `S05/W050-052` | auth + sessions + csrf + transport | login/session middleware, CSRF enforcement, rate limiter | security integration pack | `pending` |
| `S06/W060-062` | api/http + api/errors + types | full REST contract implementation | HTTP acceptance pack | `pending` |
| `S07/W070-072` | websocket + events + errors | WS stream manager, replay logic, idempotency store | `WS-04`, `WS-05`, `WS-06` | `pending` |
| `S08/W080-082` | editor-flow + layout + web-app | frontend shell/editor, responsive behavior, autosave/conflict UX | `E2E-*` pack | `pending` |
| `S09/W090-092` | CI + performance + testing | CI profiles, integration harness, performance checks | full `T0/T1/T2` | `pending` |
| `S10/W100-102` | backlog + structure + release | hardening refactors, advanced tests, release closure | hardening/chaos pack | `pending` |

## Per-Checkbox Trace Audit (Automated)

The following coverage checks validate every checkbox line under `/docs/todo` and `/docs/todo/waves`:

```bash
grep -RIn "^- \[[ x]\]" docs/todo docs/todo/waves | wc -l
# total checkbox lines

grep -RIn "^- \[[ x]\]" docs/todo docs/todo/waves | grep -Ec "\]\("
# checkbox lines with direct doc links

grep -RIn "^- \[[ x]\]" docs/todo docs/todo/waves | grep -Ev "\]\("
# any checkbox lines lacking direct links (must be empty)
```

Latest audit snapshot:

- total checkbox lines: `874`
- linked checkbox lines: `874`
- unlinked checkbox lines: `0`

## Closure Criteria

A wave is closable only when:

1. all wave checkboxes are complete
2. mapped acceptance IDs are green in [TEST_MATRIX.md](TEST_MATRIX.md)
3. evidence rows are present in [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md)
4. drift/limitations ledgers are synchronized

## Related

- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
- Test matrix: [TEST_MATRIX.md](TEST_MATRIX.md)
- Evidence index: [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md)
