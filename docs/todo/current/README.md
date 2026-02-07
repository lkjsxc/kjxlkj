# Current TODO (Standby Transition)

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

This TODO is the active control surface for reconstruction and verification waves.

Current state: documentation has been hardened and the next implementation wave is queued in standby mode.

## Local rules (normative)

- Use checklist semantics strictly:
  - `- [x]` complete and verified
  - `- [ ]` pending or intentionally queued
- If a task is deferred, record rationale in `/docs/log/` and keep a concrete queued item.
- Keep links absolute from repository root (`/docs/...`) and do not use `../`.
- Keep completion handshake in force: when a future wave reaches full completion and verification is green, invoke `Ask` for the next objective.

## Documentation direct-link coverage

The TODO system MUST directly link all documentation files through the checklist below.

- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-00.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-00.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-01.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-01.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-02.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-02.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-03.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-03.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-04.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-04.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-05.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-05.md)
- [ ] [/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-06.md](/docs/todo/doc-coverage/all-files/kjxlkj-docs-part-06.md)

## Wave status

### Completed waves

- [ ] [/docs/todo/current/wave-placeholder/README.md](/docs/todo/current/wave-placeholder/README.md)
- [ ] [/docs/todo/current/wave-reconstruction/README.md](/docs/todo/current/wave-reconstruction/README.md)
- [ ] [/docs/todo/current/wave-reading/README.md](/docs/todo/current/wave-reading/README.md)
- [ ] [/docs/todo/current/wave-docs/README.md](/docs/todo/current/wave-docs/README.md)
- [ ] [/docs/todo/current/wave-planning/README.md](/docs/todo/current/wave-planning/README.md)
- [ ] [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)
- [ ] [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)
- [ ] [/docs/todo/current/wave-recursion/recreate-todo/README.md](/docs/todo/current/wave-recursion/recreate-todo/README.md)

### Standby wave (unchecked by design)

- [ ] [/docs/todo/current/wave-recursion/next-iteration/README.md](/docs/todo/current/wave-recursion/next-iteration/README.md)
- [ ] [/docs/todo/current/wave-verification/tests/README.md](/docs/todo/current/wave-verification/tests/README.md)
- [ ] [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md) section E follow-up is closed with explicit multi-window practical-usage PTY coverage.

## Standby objective

Implementation wave has executed unchecked boundary PTY E2E items, explicitly exercised multi-window workflows in real app paths (splits, tabs, terminal panes), and re-run full verification.
