# Features: Configuration (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement configuration features (themes, keybindings, options) as specified.

## Defining documents (direct, normative)

- Config features index:
  - [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

## Coverage traversal

- Config subtree:
  - [/docs/todo/doc-coverage/spec/features/config/README.md](/docs/todo/doc-coverage/spec/features/config/README.md)

## Checklist

- [x] Placeholder scaffolding: define config file format and load/apply boundaries.
- [x] Minimal slice: implement one configuration axis end-to-end with tests.
- [x] Full conformance: implement all config documents in the subtree. — done: `config_options.rs` with OptionScope (Global/Buffer/Window), OptionValue, OptionDef, ConfigStore (define/get/set/resolve/all_names), parse_set_arg → SetAction, build_defaults (10 options)
- [x] Update conformance and limitations docs when user-visible. — done: conformance and limitations entries maintained with each batch

