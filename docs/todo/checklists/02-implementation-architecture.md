# Checklist 02: Implementation Architecture

Back: [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)

## Required Architecture Sources

- [ ] [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- [ ] [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)

## Source Tree Reconstruction Targets

- [ ] `src/crates/app/` contains only app entry crates and harness crates
- [ ] `src/crates/core/` contains core editor/state/mode/text/edit/ui crates
- [ ] `src/crates/platform/` contains host/input/render crates
- [ ] `src/crates/services/` contains service and service-adapter crates
- [ ] every source directory stays near 12 direct children
- [ ] any file trending beyond 200 lines is split before further implementation

## High-Risk Module Split Targets

- [ ] input decoding: split decode, normalization, IME gate, resolver, tracing
- [ ] window system: split tree mutation, focus policy, geometry policy, session codec
- [ ] explorer service: split state model, fs operations, refresh logic, open-target routing
- [ ] test harness: split PTY transport, script DSL, frame oracle, diagnostics formatter

## Exit to Next Checklist

- [ ] continue to [03-test-implementation.md](03-test-implementation.md)
