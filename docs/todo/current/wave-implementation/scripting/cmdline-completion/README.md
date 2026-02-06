# Scripting: Command-Line Completion (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement scriptable completion behaviors for the command line, as specified.

## Defining documents (direct, normative)

- Command-line completion:
  - [/docs/spec/scripting/cmdline-completion.md](/docs/spec/scripting/cmdline-completion.md)

## Checklist

- [x] Placeholder scaffolding: define completion provider interfaces. — done: `CompletionProvider`, `CompletionProviderKind`, `CompletionRegistry` in `scripting.rs`
- [x] Minimal slice: implement one deterministic completion source with tests.
  - cmdline_completion.rs: complete_command() with 55 builtins, complete_option() for :set, complete_buffer(), detect_completion_kind()
- [x] Full conformance: implement all completion behaviors described by the spec. — done: `completion_engine.rs` with CompletionSource (Command/Path/Option/Buffer/Help/ColorScheme/Custom), CompletionItem, CompletionState (next/prev/current/reset), detect_source, complete_commands, complete_paths, common_prefix

