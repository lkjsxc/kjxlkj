# Testing Guidance

Back: [/docs/technical/README.md](/docs/technical/README.md)

This directory contains implementation guidance that complements the normative testing contract.

Normative source:

- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Documents

| Document | Purpose |
|---|---|
| [coverage.md](coverage.md) | Coverage posture and target-by-layer rules |
| [load.md](load.md) | Stress/load scenarios and measurement guidance |
| [mutation.md](mutation.md) | Mutation testing strategy for test quality |
| [regression.md](regression.md) | Regression policy for correctness and performance |

## Usage

When implementing or repairing behavior:

1. Start from the normative spec requirement.
2. Select the minimum deterministic tests that prove the requirement.
3. Add PTY E2E tests for terminal-sensitive behavior.
4. Update conformance or limitations when user-visible behavior changes.
