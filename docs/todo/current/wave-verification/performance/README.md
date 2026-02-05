# Verification: Performance (Iteration 36)

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Scope

Verify performance and latency requirements.

## Defining specs

- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

## Acceptance criteria (placeholder)

- Given a large file (100k lines), when opening, then it MUST complete within acceptable time.
- Given a typing burst, when measuring latency, then it MUST remain imperceptible.
- Given a scroll burst, when measuring latency, then the viewport MUST follow smoothly.

## Tests (placeholder)

- [ ] Test: large file open latency
- [ ] Test: typing burst latency
- [ ] Test: scroll burst latency
- [ ] Test: resize storm handling
