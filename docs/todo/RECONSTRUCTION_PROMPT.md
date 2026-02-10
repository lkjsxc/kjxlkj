<objective>
Reconstruct a production-grade implementation from documentation so artifacts conform to policy, spec, reference, and TODO contracts.
</objective>

<authority_and_precedence>
When instructions conflict, apply this order:
1. /docs/policy/
2. /docs/spec/
3. /docs/reference/
4. /docs/todo/
5. other documentation
</authority_and_precedence>

<required_reading_order>
1. /docs/README.md
2. /docs/policy/README.md
3. /docs/policy/INSTRUCT.md
4. /docs/policy/WORKFLOW.md
5. /docs/spec/README.md
6. /docs/reference/README.md
7. /docs/reference/CONFORMANCE.md
8. /docs/reference/LIMITATIONS.md
9. /docs/reference/DRIFT_MATRIX.md
10. /docs/todo/README.md
11. /docs/todo/current/README.md
12. /docs/todo/current/verification.md
13. /docs/todo/doc-coverage/README.md
</required_reading_order>

<mandatory_execution_model>
Gate 0: inventory and mismatch audit
- [ ] refresh requirement matrix for high-risk domains
- [ ] refresh mismatch matrix with `M1`..`M5` classes

Gate 1: blocker-first planning
- [ ] pick one active blocker row from reference limitations
- [ ] define acceptance criteria with requirement IDs and test IDs

Gate 2: implementation
- [ ] implement only user-reachable behavior
- [ ] preserve architecture invariants and source-layout constraints

Gate 3: verification
- [ ] run targeted deterministic regression tests
- [ ] run required live PTY E2E tests (`*R`) for touched blockers
- [ ] run profile-appropriate full gate from /docs/reference/CI.md

Gate 4: documentation synchronization
- [ ] update CONFORMANCE, LIMITATIONS, and DRIFT_MATRIX in same change
- [ ] update TODO checkboxes only with linked evidence

Gate 5: coverage integrity
- [ ] regenerate /docs/todo/doc-coverage/ direct-link lists
- [ ] verify no stale links and no missing docs
</mandatory_execution_model>

<anti_gaming_rules>
Prohibited:
- [x] evidence-free completion claims
- [x] type-only/stub-only completion claims
- [x] unreachable behavior marked complete
- [x] stale conformance or limitation ledgers
- [x] checked TODO items without test evidence
- [x] closing high-severity blocker without matching live E2E proof
</anti_gaming_rules>

<source_layout_contract>
Workspace members MUST follow grouped crate paths:
- [x] /src/crates/app/
- [x] /src/crates/core/
- [x] /src/crates/platform/
- [x] /src/crates/services/

Keep each source directory around 12 direct children and each source file at or below 200 lines.
</source_layout_contract>

<completion_definition>
A wave is complete only when:
1. every high-severity limitation is closed
2. all blocker behavior is reachable from real input paths
3. deterministic and live E2E gates are green
4. conformance/limitations/drift/TODO are synchronized
5. doc-coverage links every documentation file directly
</completion_definition>
