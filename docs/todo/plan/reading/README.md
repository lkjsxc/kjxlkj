# Plan: Reading and Reconciliation

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order (Recursive)

### 1. Policy first

1. Read `/docs/policy/INSTRUCT.md`
2. Read `/docs/policy/STRUCTURE.md`
3. Read `/docs/policy/WORKFLOW.md`
4. Read `/docs/policy/README.md`

### 2. Index documents

1. Read `/docs/README.md`
2. Read `/docs/spec/README.md`
3. Read `/docs/overview/README.md`

### 3. High-risk behavioral specs

1. Cursor semantics: `/docs/spec/editing/cursor/README.md`
2. Windows/viewport: `/docs/spec/editor/windows.md`
3. UI viewport/scrolling: `/docs/spec/features/ui/viewport.md`
4. Cursor appearance: `/docs/spec/features/ui/cursor-customization.md`
5. Runtime ordering: `/docs/spec/architecture/runtime.md`
6. Testing contract: `/docs/spec/technical/testing.md`

### 4. Reconcile contradictions and gaps

For each document read:

1. Identify contradictions with policy or other specs
2. Record the contradiction as a TODO leaf
3. Propose a single canonical rule
4. Update cross-references so readers discover the canonical rule first
