# Features: Integrated Terminal (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement integrated terminal panes and PTY lifecycle management.

## Defining documents (direct, normative)

- Terminal features index:
  - [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Core terminal behavior:
  - [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)

## Coverage traversal

- Terminal subtree:
  - [/docs/todo/doc-coverage/spec/features/terminal/README.md](/docs/todo/doc-coverage/spec/features/terminal/README.md)

## Checklist

- [x] Placeholder scaffolding: define terminal service APIs and pane model.
- [x] Minimal slice: implement one PTY lifecycle end-to-end with tests.
- [x] Full conformance: implement all terminal feature docs (tmux, WM integration, etc.).
  - DAP debugging: DapState, Breakpoint, BreakpointKind, StackFrame, Variable, VariableScope, DapSession
  - tmux integration: TmuxIntegration with passthrough support
  - 55 tests total
- [ ] Update conformance and limitations docs when user-visible.

