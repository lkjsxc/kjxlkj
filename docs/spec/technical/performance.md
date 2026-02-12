# Performance Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Target Envelope

| Target | Requirement |
|---|---|
| Dataset scale | up to 100k notes and 500 active projects |
| Active users | up to 100 concurrent authenticated sessions |
| API latency | P95 < 200 ms under normal load |
| WS stability | sustained patch/workspace stream without ordering loss |

## Measurement Rules

- Measure latency separately for read and write endpoints.
- Exclude cold-start migration time from request latency SLO.
- Record DB and app resource utilization during load tests.
- Record stream ordering error rate and replay-gap count under WS soak.

## Critical Scenarios

| ID | Scenario |
|---|---|
| `PERF-01` | CRUD/search latency at target scale |
| `PERF-02` | sustained WS patch/workspace stream soak |

## Related

- Testing: [testing.md](testing.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
