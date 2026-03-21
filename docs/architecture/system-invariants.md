# Architecture Invariants

## Core Principles

- Functional core, imperative shell.
- Async I/O at adapter boundaries.
- Deterministic outputs for automation and CLI tooling.

## Source-of-Truth Boundaries

- Markdown files are the source of truth for article content.
- PostgreSQL is the source of truth for admin and session state.

## Change Constraints

- Behavioral changes must update matching product and operations contracts.
- Structural changes must keep docs topology valid.
