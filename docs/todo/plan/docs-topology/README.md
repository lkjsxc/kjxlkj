# Plan: Documentation Topology and Policy Compliance

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order

### 1. Eliminate policy violations

1. Remove non-Mermaid fenced blocks under `/docs/`
2. Ensure every directory has exactly one `README.md`
3. Ensure each file is ≤ 200 lines

### 2. Fix navigation and reachability

1. Ensure `/docs/README.md` links to all top-level docs directories
2. Ensure each directory `README.md` links to all children
3. Ensure all documents link back to a parent index without using `../`

### 3. Enforce directory child limits

1. Reduce any directory exceeding 12 direct children by grouping into subdirectories
2. Update all relevant `README.md` indexes accordingly

### 4. Validate links

1. Ensure every link resolves within the repository
2. Prefer `/docs/...` root paths for cross-directory links
