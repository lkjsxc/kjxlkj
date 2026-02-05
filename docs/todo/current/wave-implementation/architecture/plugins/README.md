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

- [ ] Define what "plugin-like behavior" means in this codebase and prohibit it.
- [ ] Ensure all feature extension points are internal modules/services only.
- [ ] Add tests and/or build checks that prevent introducing plugin loading.

