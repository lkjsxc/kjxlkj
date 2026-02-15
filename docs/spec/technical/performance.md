# Performance Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Target Envelope

| Target | Requirement |
|---|---|
| Dataset scale | up to 100k notes |
| Active users | up to 100 concurrent sessions |
| API latency | P95 < 200 ms under normal load |
| Search latency | P95 < 250 ms for hybrid mode at target scale |
| WS stability | sustained stream without ordering loss |
| Agent loop | loop iteration P95 < 5 s excluding provider wait |

## Critical Scenarios

| ID | Scenario |
|---|---|
| `PERF-01` | CRUD/search latency at target scale |
| `PERF-02` | sustained WS replay/patch soak |
| `PERF-03` | embedding index refresh throughput |
| `PERF-04` | `kjxlkj-agent` loop throughput with KV memory persistence |

## Related

- Search spec: [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
- Agent spec: [librarian-agent.md](librarian-agent.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
