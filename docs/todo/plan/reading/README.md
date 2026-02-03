# Plan: Reading and Reconciliation

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order (Recursive)

### 1. Complete doc coverage (mandatory)

1. Follow: [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
2. Record completion and contradictions in: [/docs/todo/reading/README.md](/docs/todo/reading/README.md)

### 2. Policy and spec indices (quick anchor rereads)

1. `/docs/policy/README.md`
2. `/docs/spec/README.md`
3. `/docs/overview/README.md`
4. `/docs/reference/CONFORMANCE.md`

### 3. High-risk behavioral specs

1. Cursor semantics: `/docs/spec/editing/cursor/README.md`
2. Windows/viewport: `/docs/spec/editor/windows.md`
3. UI viewport/scrolling: `/docs/spec/features/ui/viewport.md`
4. Runtime ordering: `/docs/spec/architecture/runtime.md`
5. Testing contract: `/docs/spec/technical/testing.md`

### 4. Reconcile contradictions and gaps

For each document read:

1. Identify contradictions with policy or other specs
2. Record the contradiction as a TODO leaf
3. Propose a single canonical rule
4. Update cross-references so readers discover the canonical rule first
