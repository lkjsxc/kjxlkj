# Performance Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Target Envelope

| Target | Requirement |
|---|---|
| Dataset scale | up to 100k notes (personal large vault profile) |
| API latency | P95 < 200 ms under normal load |
| WS stability | sustained patch stream without ordering loss |

## Measurement Rules

- Measure latency separately for read and write endpoints.
- Exclude cold-start migration time from request latency SLO.
- Record DB and app resource utilization during load tests.

## Critical Scenarios

| ID | Scenario |
|---|---|
| `PERF-01` | CRUD/search latency at target scale |
| `PERF-02` | sustained WS patch stream soak |

## Related

- Testing: [testing.md](testing.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
