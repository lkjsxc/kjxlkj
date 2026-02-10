<objective>
Reconstruct a complete, production-grade implementation from documentation so
generated artifacts conform to policy, spec, reference, and TODO contracts.
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
- build requirement matrix for normative spec files
- update drift matrix with mismatch class per requirement

Gate 1: slice planning
- pick one TODO phase slice
- define acceptance criteria with requirement IDs and test IDs

Gate 2: implementation
- implement only user-reachable behavior
- preserve architecture invariants and source-layout constraints

Gate 3: verification
- run targeted deterministic tests for touched requirements
- run profile-appropriate full gate from /docs/reference/CI.md

Gate 4: documentation synchronization
- update CONFORMANCE, LIMITATIONS, and DRIFT_MATRIX in same change
- update TODO checkboxes only with evidence

Gate 5: coverage integrity
- regenerate /docs/todo/doc-coverage/ direct-link lists
- verify no stale links and no missing docs
</mandatory_execution_model>

<anti_gaming_rules>
Prohibited:
- evidence-free completion claims
- type-only/stub-only completion claims
- unreachable behavior marked complete
- stale conformance or limitation ledgers
- checked TODO items without test evidence
</anti_gaming_rules>

<completion_definition>
A wave is complete only when:
1. every normative requirement is verified or explicitly limited
2. all claimed behavior is reachable from real input paths
3. deterministic verification gates are green
4. conformance/limitations/TODO are synchronized
5. doc-coverage links every documentation file directly
</completion_definition>
