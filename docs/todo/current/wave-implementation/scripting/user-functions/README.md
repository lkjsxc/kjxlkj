# Scripting: User Functions (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement user-defined functions where the spec requires them, including invocation contexts.

## Defining documents (direct, normative)

- User functions:
  - [/docs/spec/scripting/user-functions.md](/docs/spec/scripting/user-functions.md)

## Checklist

- [x] Placeholder scaffolding: define function registry and call boundary. — done: `UserFunction`, `UserFunctionRegistry` in `scripting.rs`
- [x] Minimal slice: implement one deterministic function call with tests. — done: `user_function_exec.rs` with `FuncContext`, `execute_function()`, `parse_let()`, `resolve_expression()`, 7 tests
- [x] Full conformance: implement all function behaviors described by the spec. — done: `user_functions.rs` with UserFunction (params/body/varargs/script-local/range/abort), FunctionRegistry (define/get/remove/list), parse_function, parse_call

