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
| [CONFORMANCE_MODES_KEYS.md](CONFORMANCE_MODES_KEYS.md) | Modes and keybindings (ledger) | Authoritative |
| [CONFORMANCE_EDITING.md](CONFORMANCE_EDITING.md) | Editing semantics (ledger) | Authoritative |
| [CONFORMANCE_COMMANDS_TESTING.md](CONFORMANCE_COMMANDS_TESTING.md) | Ex commands and headless testing (ledger) | Authoritative |
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
