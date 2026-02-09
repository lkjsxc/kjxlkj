<objective>
Reconstruct a complete, production-grade implementation from documentation so generated artifacts fully conform to policy, spec, reference, and todo contracts.
</objective>

<scope>
Produce a shippable repository state from the docs-only baseline, including:

- workspace source code and manifests
- binary entrypoint and runtime wiring
- verification automation required by `/docs/reference/CI.md`
- release-supporting root artifacts required by `/docs/policy/ROOT_LAYOUT.md`

Documentation updates are allowed and expected when resolving contradictions.
</scope>

<authority_and_precedence>
When instructions conflict, apply this order:

1. `/docs/policy/`
2. `/docs/spec/`
3. `/docs/reference/`
4. `/docs/todo/`
5. other documentation (`/docs/technical/`, `/docs/design/`, `/docs/guides/`, `/docs/overview/`, `/docs/log/`)

Interpretation rules:
- `/docs/spec/` defines target behavior.
- `/docs/reference/` defines currently verified behavior and open limitations.
- `/docs/todo/` defines execution sequencing and completion gates.
</authority_and_precedence>

<required_reading_order>
1. `/docs/README.md`
2. `/docs/policy/README.md`
3. `/docs/policy/INSTRUCT.md`
4. `/docs/policy/WORKFLOW.md`
5. `/docs/spec/README.md`
6. `/docs/reference/README.md`
7. `/docs/reference/CONFORMANCE.md`
8. `/docs/reference/LIMITATIONS.md`
9. `/docs/reference/CI.md`
10. `/docs/todo/README.md`
11. `/docs/todo/current/README.md`
12. `/docs/todo/current/verification.md`
13. `/docs/todo/doc-coverage/README.md`
</required_reading_order>

<mandatory_execution_model>
Work in deterministic gates and do not advance on red checks.

Gate 0: Inventory and mismatch audit
- Build a requirement matrix for all relevant `/docs/spec/...` requirements.
- Assign stable requirement IDs and link each ID to spec path.
- Build mismatch matrix: spec vs implementation vs tests vs reference claims.
- Classify mismatches as correctness, missing feature, undocumented behavior, verification gap, or stale docs.

Gate 1: Slice planning
- Choose one coherent slice from `/docs/todo/current/`.
- Define acceptance criteria using exact spec links and requirement IDs.
- Identify required deterministic tests before implementation.

Gate 2: Implementation
- Implement only real user-reachable behavior.
- Wire behavior through actual runtime dispatch paths.
- Ensure architecture invariants from `/docs/spec/architecture/` hold.
- If parallel agent/runtime implementations exist, converge to one canonical runtime path that matches spec.

Gate 3: Verification
- Run targeted deterministic tests for touched requirements.
- Run profile-appropriate full gate from `/docs/reference/CI.md`.
- Store evidence in `/docs/log/reconstruction/audits/`.

Gate 4: Documentation synchronization
- Update `/docs/reference/CONFORMANCE.md` with evidence-backed status.
- Update `/docs/reference/LIMITATIONS.md` for remaining user-visible gaps.
- Update relevant files under `/docs/reference/conformance/`.
- Update `/docs/todo/current/` checkboxes only where evidence exists.

Gate 5: Coverage integrity
- Regenerate or validate `/docs/todo/doc-coverage/`.
- Verify no docs links are stale.
- Verify no required document is missing from coverage lists.
</mandatory_execution_model>

<completion_definition>
A reconstruction wave is complete only when all are true:

1. Every normative requirement is either verified or represented as an explicit limitation.
2. Verified behavior is reachable from real runtime input paths.
3. Deterministic verification gates are green for the active profile.
4. Conformance, limitations, and TODO states are synchronized.
5. Documentation coverage and internal links are clean.
</completion_definition>

<anti_gaming_rules>
Prohibited:
- evidence-free completion claims
- type-only or scaffold-only work labeled as complete
- unreachable behavior labeled as implemented
- stale reference ledgers after behavior changes
- checkbox completion without verification artifacts

When behavior is intentionally deferred:
- keep limitation entry open with rationale and next action
- keep corresponding TODO item unchecked unless the item is explicitly a defer-recording task
</anti_gaming_rules>

<required_artifacts_per_wave>
Each wave must produce at least one dated audit record under `/docs/log/reconstruction/audits/` containing:

- requirement IDs covered
- mismatches closed with evidence links
- mismatches deferred with rationale and next action
- exact verification commands and result summary
</required_artifacts_per_wave>
