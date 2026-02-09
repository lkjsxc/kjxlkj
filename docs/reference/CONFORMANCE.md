# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified, with evidence links.

## Current Baseline

This repository is currently maintained in a documentation-first reconstruction state.

In this state:

- target behavior is defined by `/docs/spec/`
- implementation may be partial, stale, or absent
- claims in this file MUST be evidence-backed and time-scoped

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | Reachable behavior confirmed by deterministic evidence |
| `partial` | Some behavior is verified, but user-visible gaps remain |
| `scaffold-only` | Types/modules exist but user path is not complete |
| `unverified` | No current evidence recorded |

## Verification Snapshot

| Check | Result | Evidence |
|---|---|---|
| Documentation consistency sweep | completed | current change set |
| Docs-only baseline shape | completed | current change set (workspace artifacts intentionally absent) |
| Runtime verification gate | unverified | no current evidence record |

## Claim Rules

A claim is valid only when all are true:

1. Linked spec requirement exists.
2. Runtime path is user-reachable.
3. Deterministic verification evidence is linked.
4. Remaining user-visible gaps are listed in `LIMITATIONS`.

## Domain Ledgers

- [conformance/CONFORMANCE_MODES.md](conformance/CONFORMANCE_MODES.md)
- [conformance/CONFORMANCE_KEYS_INPUT.md](conformance/CONFORMANCE_KEYS_INPUT.md)
- [conformance/CONFORMANCE_KEYS_SYSTEMS.md](conformance/CONFORMANCE_KEYS_SYSTEMS.md)
- [conformance/CONFORMANCE_KEYS_INFRA.md](conformance/CONFORMANCE_KEYS_INFRA.md)
- [conformance/CONFORMANCE_EDITING_OPERATORS.md](conformance/CONFORMANCE_EDITING_OPERATORS.md)
- [conformance/CONFORMANCE_EDITING_FEATURES.md](conformance/CONFORMANCE_EDITING_FEATURES.md)
- [conformance/CONFORMANCE_COMMANDS.md](conformance/CONFORMANCE_COMMANDS.md)
- [conformance/CONFORMANCE_COMMANDS_TYPES.md](conformance/CONFORMANCE_COMMANDS_TYPES.md)
- [conformance/CONFORMANCE_TESTING.md](conformance/CONFORMANCE_TESTING.md)
- [conformance/CONFORMANCE_TESTING_INFRA.md](conformance/CONFORMANCE_TESTING_INFRA.md)

## Related

- Open mismatches: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
