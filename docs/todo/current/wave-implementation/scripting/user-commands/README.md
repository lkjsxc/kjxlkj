# Scripting: User Commands (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement user-defined commands and their integration with Ex parsing/execution.

## Defining documents (direct, normative)

- User commands:
  - [/docs/spec/scripting/user-commands.md](/docs/spec/scripting/user-commands.md)

## Checklist

- [x] Placeholder scaffolding: define command registration and scoping rules. — done: `UserCommand`, `NArgs`, `UserCommandRegistry` in `scripting.rs`
- [x] Minimal slice: create one user command deterministically with tests. — done: `user_command_exec.rs` with `execute_user_command()`, `dispatch_user_command()`, `validate_nargs()`, `substitute_args()`, 9 tests
- [ ] Full conformance: implement all user command features described by the spec.

