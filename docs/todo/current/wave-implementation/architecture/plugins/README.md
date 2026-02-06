# Architecture: Plugins (Iteration 34)

Back: [/docs/todo/current/wave-implementation/architecture/README.md](/docs/todo/current/wave-implementation/architecture/README.md)

## Scope

Enforce the “no plugins” invariant while still supporting built-in extensibility points.

## Defining documents (direct, normative)

- Plugin architecture (constraints):
  - [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- Policy invariants:
  - [/docs/policy/README.md](/docs/policy/README.md)

## Checklist

- [x] Define what "plugin-like behavior" means in this codebase and prohibit it. — done: `plugin_prevention.rs` with FORBIDDEN_PATTERNS, audit_source(), audit_files()
- [x] Ensure all feature extension points are internal modules/services only.
- [x] Add tests and/or build checks that prevent introducing plugin loading. — done: 9 tests for clean source, dlopen detection, dependency checking, architecture rule

