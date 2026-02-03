# TODO Management

Back: [/docs/README.md](/docs/README.md)
Structured task tracking for continuous project improvement.

## Overview

This directory manages the recursive TODO workflow for kjxlkj development.

## Documents

| Document | Content |
|----------|---------|
| [current/README.md](current/README.md) | Active TODO list (current iteration) |
| [completed/README.md](completed/README.md) | Completed tasks archive |
| [plan/README.md](plan/README.md) | Detailed implementation plan (recursive) |
| [reading/README.md](reading/README.md) | Reading log (spec/policy comprehension) |
| [doc-coverage/README.md](doc-coverage/README.md) | Checklist that links every doc outside `/docs/todo/` |

## Workflow

| Phase | Description |
|-------|-------------|
| Plan | Read docs, understand requirements |
| Implement | Create source code matching specs |
| Verify | Validate against policies |
| Commit | Frequent git commits |
| Recurse | Regenerate TODO list |

## Invariants

| Rule | Requirement |
|------|-------------|
| Second-to-last task | Always: Recreate the TODO list |
| Last task | Always: Continue to next iteration |
| Continuous | Work proceeds without stopping |

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Spec: [/docs/spec/README.md](/docs/spec/README.md)
