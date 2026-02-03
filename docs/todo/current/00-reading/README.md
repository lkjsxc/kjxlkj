# Reading (Iteration 32)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Track which project documents have been read in-depth and which still require deep rereads.

## Requirements

- The reading log MUST list at least 50 documents read in-depth.
- The reading log MUST prioritize policy and canonical spec documents.
- Notes MUST focus on requirements, invariants, and acceptance criteria.

## Work Items

### A. Establish the reading index

- Add a dated entry under [/docs/todo/reading/README.md](/docs/todo/reading/README.md) that lists the documents read for this iteration.
- For each document, record:
  - Why it matters
  - Any contradictions found
  - The next follow-up doc to reconcile with

### B. Re-read high-risk specs before making behavior changes

High-risk areas (read first, re-read when editing related specs):

- Cursor semantics and rendering
- Viewport / scrolling / follow behavior
- Input decoding and event ordering
- Rendering pipeline constraints and performance targets
