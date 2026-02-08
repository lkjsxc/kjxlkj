# Reference Documentation

Back: [/docs/README.md](/docs/README.md)
Reference materials for implementation status, release readiness, and scope framing.

## Authority model (normative)

Use references with this authority order:

1. `/docs/spec/` defines target behavior.
2. `/docs/reference/CONFORMANCE.md` defines what is currently implemented.
3. `/docs/reference/LIMITATIONS.md` defines user-visible gaps against target behavior.
4. All other docs in this directory are supportive and non-authoritative for runtime status.

Do not claim "implemented" from non-authoritative docs.

## Documents

| Document | Purpose | Authority |
|----------|---------|-----------|
| [CONFORMANCE.md](CONFORMANCE.md) | Conformance ledger (implementation surface / reconstruction target) | Authoritative |
| [CONFORMANCE_MODES.md](CONFORMANCE_MODES.md) | Modes, Normal-mode keys, Visual mode (ledger) | Authoritative |
| [CONFORMANCE_KEYS_INPUT.md](CONFORMANCE_KEYS_INPUT.md) | Insert/Replace modes and core systems (ledger) | Authoritative |
| [CONFORMANCE_KEYS_SYSTEMS.md](CONFORMANCE_KEYS_SYSTEMS.md) | UI, terminal, buffer, window, theme systems (ledger) | Authoritative |
| [CONFORMANCE_KEYS_INFRA.md](CONFORMANCE_KEYS_INFRA.md) | Input infrastructure and coverage (ledger) | Authoritative |
| [CONFORMANCE_EDITING_OPERATORS.md](CONFORMANCE_EDITING_OPERATORS.md) | Editing operators, motions, text objects (ledger) | Authoritative |
| [CONFORMANCE_EDITING_FEATURES.md](CONFORMANCE_EDITING_FEATURES.md) | Editing features and rendering (ledger) | Authoritative |
| [CONFORMANCE_COMMANDS.md](CONFORMANCE_COMMANDS.md) | Ex commands, ranges, core types (ledger) | Authoritative |
| [CONFORMANCE_COMMANDS_TYPES.md](CONFORMANCE_COMMANDS_TYPES.md) | Command and editor type definitions (ledger) | Authoritative |
| [CONFORMANCE_TESTING.md](CONFORMANCE_TESTING.md) | Headless test runner and E2E coverage (ledger) | Authoritative |
| [CONFORMANCE_TESTING_INFRA.md](CONFORMANCE_TESTING_INFRA.md) | Testing infrastructure and integration (ledger) | Authoritative |
| [LIMITATIONS.md](LIMITATIONS.md) | Known user-visible gaps | Authoritative |
| [CI.md](CI.md) | Continuous integration checks | Authoritative for verification gate |
| [RELEASE.md](RELEASE.md) | Release process and readiness checks | Normative process |
| [COMPARISON.md](COMPARISON.md) | Editor comparison matrix | Non-authoritative for current status |
| [PLUGIN_MAPPING.md](PLUGIN_MAPPING.md) | Plugin-to-built-in target mapping | Non-authoritative for current status |

## Update discipline

When implementation behavior changes:

1. Update conformance and limitations first.
2. Verify tests and CI commands are accurate.
3. Only then update comparison/mapping summaries.

## Related

- Guides: [docs/guides/README.md](/docs/guides/README.md)
- Specifications: [docs/spec/README.md](/docs/spec/README.md)
