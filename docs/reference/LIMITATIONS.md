# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open gaps between target spec and current repository behavior.

## Baseline (2026-02-15)

- Full runtime reconstructed from documentation specs.
- All 10 Rust crates compile and pass unit tests.
- Remaining gaps are integration/E2E test coverage and live DB verification.

## Closed Limitations

| ID | Requirement Link | Gap | Resolution |
|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | source tree absent | **closed** — 10 crates implemented |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search not implemented | **closed** — kjxlkj-search crate with lexical+semantic pipeline |
| `LIM-NOTE-TITLE-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | datetime default missing | **closed** — create_note defaults to datetime title |
| `LIM-EDITOR-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor not implemented | **closed** — app-shell.ts with autosave/conflict/offline states |
| `LIM-LAYOUT-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | compact menu threshold not implemented | **closed** — 1280px breakpoint in app-shell.ts |
| `LIM-AGENT-JSON-01` | [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) | prompt JSON loader not implemented | **closed** — prompt.rs with SHA-256 validation |
| `LIM-AGENT-KV-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | KV memory not implemented | **closed** — kv_memory.rs + agent_kv_store table |
| `LIM-AGENT-YOLO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | YOLO mutation not implemented | **closed** — agent_loop.rs execute phase |

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | DB-backed integration tests require live PostgreSQL | `M4 verification gap` | medium | run integration suite against Docker container |
| `LIM-CI-01` | [/docs/reference/CI.md](/docs/reference/CI.md) | CI workflow file (.github/workflows) not created | `M2 missing feature` | medium | create GitHub Actions workflow |
| `LIM-E2E-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | browser E2E tests not implemented | `M4 verification gap` | medium | implement Playwright test suite |

## Closure Rules

A limitation closes only when:

1. behavior is runtime-reachable
2. deterministic tests pass
3. drift and TODO ledgers are synchronized

## Related

- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- TODO program: [/docs/todo/README.md](/docs/todo/README.md)
