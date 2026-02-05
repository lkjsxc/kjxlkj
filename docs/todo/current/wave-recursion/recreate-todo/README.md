# Recreate the TODO List (Iteration 36)

Back: [/docs/todo/current/wave-recursion/README.md](/docs/todo/current/wave-recursion/README.md)

## Purpose

Keep the work recursive by regenerating the current TODO list with:

- Newly discovered issues
- Newly added specification gaps
- Newly identified contradictions

## Rules

- The regenerated TODO list MUST still be recursively structured.
- The regenerated TODO list MUST link every documentation file outside `/docs/todo/` via:
  - [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- The regenerated TODO list MUST keep the second-to-last item as “Recreate the TODO list”.
- The regenerated TODO list MUST create a new iteration directory under `/docs/todo/current/` only after completing the current iteration’s checklist.

## Checklist

### A. Expand and correct the TODO surface

- [ ] Add newly discovered issues as leaf TODOs (do not hide gaps in prose).
- [ ] Replace “future” pre-checked items with explicit unchecked work items.
- [ ] Ensure each leaf is small, testable, and directly linked to its defining spec documents.

### B. Verify full document reachability (required)

- [ ] Verify every documentation file is reachable via the TODO system by traversing:
  - [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- [ ] If any doc was added/moved, regenerate the doc-coverage index pages under
  `/docs/todo/doc-coverage/` and re-run reachability verification.

### C. Preserve iteration invariants

- [ ] Confirm the second-to-last wave remains:
  - [/docs/todo/current/wave-recursion/recreate-todo/README.md](/docs/todo/current/wave-recursion/recreate-todo/README.md)
- [ ] Confirm the last wave remains:
  - [/docs/todo/current/wave-recursion/next-iteration/README.md](/docs/todo/current/wave-recursion/next-iteration/README.md)
