# Performance Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Target Envelope

| Target | Requirement |
|---|---|
| Dataset scale | up to 100k notes and 500 active projects |
| Active users | up to 100 concurrent authenticated sessions |
| API latency | P95 < 200 ms under normal load |
| WS stability | sustained patch/workspace stream without ordering loss |
| Librarian batch | 50-source structuring batch completes in < 90 s with configured model budget |

## Measurement Rules

- Measure latency separately for read and write endpoints.
- Exclude cold-start migration time from request latency SLO.
- Record DB and app resource utilization during load tests.
- Record stream ordering error rate and replay-gap count under WS soak.
- Record parse-failure rate and retry count for librarian protocol runs.

## Critical Scenarios

| ID | Scenario |
|---|---|
| `PERF-01` | CRUD/search latency at target scale |
| `PERF-02` | sustained WS patch/workspace stream soak |
| `PERF-03` | librarian structuring throughput and retry-rate envelope |

## Related

- Testing: [testing.md](testing.md)
- Librarian technical contract: [librarian-agent.md](librarian-agent.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
