# Scripting: Event Automation (Iteration 34)

Back: [/docs/todo/current/wave-implementation/scripting/README.md](/docs/todo/current/wave-implementation/scripting/README.md)

## Scope

Implement event automation, hooks, and event routing rules.

## Defining documents (direct, normative)

- Event automation:
  - [/docs/spec/scripting/event-automation.md](/docs/spec/scripting/event-automation.md)

## Checklist

- [x] Placeholder scaffolding: define event types and subscription model.
- [x] Minimal slice: implement one hookable event with deterministic tests.
- [x] Full conformance: implement all automation rules and event families. — done: `event_automation.rs` with AutoEvent (17 events), AutoPattern (All/Glob/FileType), AutoCmd, AutoCmdRegistry (add/matching/clear_group/remove_once_fired), fire_event

