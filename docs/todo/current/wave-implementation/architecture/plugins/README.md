# Architecture: Plugins (Iteration 33)

Back: [/docs/todo/current/wave-implementation/architecture/README.md](/docs/todo/current/wave-implementation/architecture/README.md)

## Scope

Enforce the “no plugins” invariant while still supporting built-in extensibility points.

## Defining documents (direct, normative)

- Plugin architecture (constraints):
  - [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- Policy invariants:
  - [/docs/policy/README.md](/docs/policy/README.md)

## Checklist

- [x] Define what “plugin-like behavior” means in this codebase and prohibit it.
- [x] Ensure all feature extension points are internal modules/services only.
- [x] Add tests and/or build checks that prevent introducing plugin loading.

