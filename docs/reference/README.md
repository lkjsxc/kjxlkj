# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` records verified implementation status and release operations.

## Authority Model

Use this order when deciding what is true right now:

1. [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
2. [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
3. Deterministic automated test evidence
4. Other files in this directory

`/docs/spec/` remains the target contract for what should exist in the final product.

## Documents

| Document | Role |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | Current verified implementation surface |
| [LIMITATIONS.md](LIMITATIONS.md) | User-visible gaps and temporary exceptions |
| [CI.md](CI.md) | Verification command baseline |
| [RELEASE.md](RELEASE.md) | Release process and evidence checks |
| [CONFORMANCE_TESTING.md](CONFORMANCE_TESTING.md) | Test-surface conformance mapping |
| [CONFORMANCE_TESTING_INFRA.md](CONFORMANCE_TESTING_INFRA.md) | Integration and infra conformance mapping |
| [CONFORMANCE_MODES.md](CONFORMANCE_MODES.md) | Mode conformance sub-ledger |
| [CONFORMANCE_KEYS_INPUT.md](CONFORMANCE_KEYS_INPUT.md) | Input key conformance sub-ledger |
| [CONFORMANCE_KEYS_SYSTEMS.md](CONFORMANCE_KEYS_SYSTEMS.md) | Systems key conformance sub-ledger |
| [CONFORMANCE_KEYS_INFRA.md](CONFORMANCE_KEYS_INFRA.md) | Input infra conformance sub-ledger |
| [CONFORMANCE_EDITING_OPERATORS.md](CONFORMANCE_EDITING_OPERATORS.md) | Editing operator conformance sub-ledger |
| [CONFORMANCE_EDITING_FEATURES.md](CONFORMANCE_EDITING_FEATURES.md) | Editing feature conformance sub-ledger |
| [CONFORMANCE_COMMANDS.md](CONFORMANCE_COMMANDS.md) | Command conformance sub-ledger |
| [CONFORMANCE_COMMANDS_TYPES.md](CONFORMANCE_COMMANDS_TYPES.md) | Command/type conformance sub-ledger |
| [COMPARISON.md](COMPARISON.md) | Comparison matrix (non-authoritative) |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | Plugin migration mapping (non-authoritative) |

## Update Discipline

- Update `CONFORMANCE` and `LIMITATIONS` in the same change as behavior updates.
- Keep claims tied to existing files/tests.
- If behavior is target-only, keep it in `/docs/spec/` and mark as not yet implemented.

## Related

- Target specification: [/docs/spec/README.md](/docs/spec/README.md)
- Reconstruction TODOs: [/docs/todo/current/README.md](/docs/todo/current/README.md)
