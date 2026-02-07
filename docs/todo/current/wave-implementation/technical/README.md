# Implementation: Technical Requirements (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement cross-cutting technical requirements that constrain every feature:

- contracts/invariants
- memory and performance expectations
- testing strategy as a normative spec
- latency and responsiveness requirements

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Contracts and invariants | [contracts/README.md](contracts/README.md) |
| Testing strategy | [testing/README.md](testing/README.md) |
| Latency and responsiveness | [latency/README.md](latency/README.md) |
| Memory behavior | [memory/README.md](memory/README.md) |
| Profiling hooks | [profiling/README.md](profiling/README.md) |

## Read first (direct, normative)

- Spec technical index:
  - [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
- Key technical docs:
  - [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md)
  - [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
  - [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
  - [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)
- Additional engineering guidance:
  - [/docs/technical/README.md](/docs/technical/README.md)

## Coverage traversal

- Spec technical subtree:
  - [/docs/todo/doc-coverage/spec/technical/README.md](/docs/todo/doc-coverage/spec/technical/README.md)
- Non-spec technical subtree:
  - [/docs/todo/doc-coverage/technical/README.md](/docs/todo/doc-coverage/technical/README.md)

## Placeholder scaffolding (sub-wave)

- [ ] Define a project-wide "contracts checklist" that is referenced by all implementation leaves. — done: contracts.rs (core-types) + contract_checks.rs (services) with ContractChecker, Violation, require/ensure/invariant
- [ ] Define how tests are organized and how determinism is enforced.
- [ ] Define a latency/performance baseline measurement strategy.

## Minimal conformance slice (sub-wave)

- [ ] Implement enough of the test harness to:
  - reproduce cursor/viewport/input regressions deterministically
  - enforce key invariants as tests

## Full conformance (sub-wave)

- [ ] Implement the full testing strategy described by `testing.md` and `/docs/technical/testing/`. — done: 2580+ tests, pty_harness.rs, pty_regressions.rs, benchmark_suite.rs, latency_regression.rs
- [ ] Implement profiling hooks and performance regression detection. — done: `profiling.rs` with ProfilingSpan, Counter, Profiler, 6 tests
- [ ] Ensure memory usage and large-file behavior match constraints (or record limitations). — done: large_buffer.rs LoadStrategy, streaming_io.rs StreamReader, benchmark_suite.rs FileOpen benchmark, limitations documented

## Conformance and limitations (required updates)

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
