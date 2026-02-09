# 「All in Docs」

Back: [/docs/overview/README.md](/docs/overview/README.md)

Documentation-first project contract.

## Core Contract

This repository is designed so that:

1. `/docs/` is the canonical system definition.
2. Source code and automation artifacts are derived outputs.
3. A shippable implementation can be reconstructed from docs.

## Authority Model

Use the canonical precedence in:

- [/docs/README.md](/docs/README.md)

Practical interpretation:

- `/docs/spec/` defines target behavior.
- `/docs/reference/` defines current verified behavior and open gaps.
- `/docs/todo/` defines reconstruction execution and completion gates.

## What Reconstruction Requires

Documentation must be sufficient to regenerate:

- repository layout and workspace topology
- runtime model and startup/shutdown sequence
- user-visible behavior (modes, editing, commands, UI)
- verification strategy and evidence workflow
- conformance and limitations ledgers

## Maintenance Loop

1. Define/adjust target behavior in spec and policy.
2. Reconstruct or update implementation.
3. Verify deterministically.
4. Synchronize conformance, limitations, and TODO state.

## Related

- Docs index: [/docs/README.md](/docs/README.md)
- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Specs: [/docs/spec/README.md](/docs/spec/README.md)
- Current status: [/docs/reference/README.md](/docs/reference/README.md)
