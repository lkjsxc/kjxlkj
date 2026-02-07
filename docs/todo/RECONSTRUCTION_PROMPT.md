<objective>
Reconstruct and improve implementation from `/docs/` with maximum practical correctness.
</objective>

<non_goals>
- no evidence-free checkbox completion
- no breadth-only scaffold inflation presented as completion
- no stale status claims in conformance, limitations, or release docs
</non_goals>

<authority_and_precedence>
Use this precedence when instructions conflict:

1. `/docs/policy/`
2. `/docs/spec/`
3. `/docs/reference/CONFORMANCE*.md` and `/docs/reference/LIMITATIONS.md`
4. `/docs/todo/current/`
5. all other docs

Canonical facts:
- target behavior: `/docs/spec/`
- current supported behavior: `/docs/reference/CONFORMANCE.md`
- user-visible gaps: `/docs/reference/LIMITATIONS.md`
</authority_and_precedence>

<required_start_reading_order>
1. `/docs/README.md`
2. `/docs/policy/README.md`
3. `/docs/policy/ROOT_LAYOUT.md`
4. `/docs/policy/STRUCTURE.md`
5. `/docs/policy/WORKFLOW.md`
6. `/docs/spec/README.md`
7. `/docs/spec/technical/testing.md`
8. `/docs/reference/CONFORMANCE.md`
9. `/docs/reference/LIMITATIONS.md`
10. `/docs/todo/current/README.md`
</required_start_reading_order>

<target_selection_required>
At start, select exactly one target:
- A: full `/docs/spec/`
- B: current `/docs/reference/CONFORMANCE.md`

If scope interpretation changes, update:
- `/docs/reference/CONFORMANCE.md`
- `/docs/reference/LIMITATIONS.md` when user-visible
- `/docs/log/proposals/` when tradeoffs or deferrals are introduced
</target_selection_required>

<execution_model>
Work in strict gates. Do not proceed to next gate on red checks.

Gate 0: Baseline audit
- run verification commands from `/docs/reference/CI.md`
- create mismatch matrix and classify:
  - M1 correctness
  - M2 missing feature
  - M3 undocumented behavior
  - M4 verification gap
  - M5 stale docs
- fix priority: M1 -> user-visible M2 -> touched-area M4 -> M3/M5

Gate 1: Slice definition
- choose one coherent slice from `/docs/todo/current/`
- define acceptance criteria and exact `/docs/spec/...` references
- define required tests by behavior risk:
  - unit + integration always
  - headless E2E for cross-module behavior
  - PTY E2E for interactive terminal paths

Gate 2: Implement
- wire behavior through real user-reachable paths
- keep structure policy limits and root-absolute links (`/docs/...`)
- avoid disconnected type-only work unless explicitly marked `scaffold-only`

Gate 3: Verify and synchronize docs
- run touched tests first, then full gate
- update conformance and limitations in same change
- check TODO items only after evidence is green
</execution_model>

<anti_gaming_rules>
Prohibited:
- evidence-free completion
- claiming "implemented" for unreachable behavior
- low-signal test inflation for count targets
- broad stubs presented as shipped functionality
- destructive resets used to hide regressions

Implemented means all are true:
- reachable via documented command/key workflow
- user-visible behavior matches spec expectation
- deterministic regression coverage exists
- conformance and limitations are accurate
</anti_gaming_rules>

<deletion_policy>
Default is in-place improvement.
Docs-only reset is allowed only when explicitly requested or when incremental repair is not viable.
Before destructive reset, record rationale in `/docs/log/proposals/`.
</deletion_policy>

<testing_contract>
Follow `/docs/spec/technical/testing.md` as normative:
- each bug fix adds a regression test that fails on old behavior
- prefer persisted-state assertions over fragile screen scraping
- enforce deterministic deadlines and actionable timeout diagnostics
- keep mandatory PTY boundary scenarios and multiplexer smoke coverage
- optimize for coverage quality and reproducibility, not raw test count
</testing_contract>

<todo_deferral_rules>
For intentional deferral under `/docs/todo/current/`:
- record rationale under `/docs/log/proposals/`
- add concrete next-iteration leaf task
- then mark the deferral item complete
- do not leave terminal deferred buckets without actionable carry-forward
</todo_deferral_rules>

<required_audit_artifacts>
During each run, produce or update one audit record under:
- `/docs/log/reconstruction/audits/`

Include:
- mismatch matrix
- closed mismatches with evidence
- deferred mismatches with rationale and next action
- exact verification commands and result signals
</required_audit_artifacts>

<traceability_matrix_schema>
- Requirement ID
- Canonical doc path
- Requirement statement
- Code path(s)
- Test path(s)
- Status (`aligned`, `spec-only`, `code-only`, `test-gap`, `contradiction`)
- Mismatch class (M1-M5)
- Action (`implement`, `spec-update`, `test-add`, `refactor`, `defer-with-log`)
- Verification evidence
</traceability_matrix_schema>

<completion_protocol>
When iteration is fully green:
- publish closure evidence in audit artifacts
- invoke `Ask` for next objective per `/docs/policy/WORKFLOW.md`
- if `Ask` is unavailable, emit an explicit blocking request in plain text
</completion_protocol>

<acceptance_criteria>
Accept only when all are true:
- behavior matches selected target and linked specs
- conformance/limitations match observed behavior
- required deterministic tests are green at required layers
- TODO checkboxes reflect proven completion
- terminal multiplexer contract is verified or explicitly limited with closure plan
</acceptance_criteria>
