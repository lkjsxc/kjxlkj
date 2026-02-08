<objective>
Reconstruct a complete, fully-featured implementation from `/docs/` with maximum practical correctness. The implementation MUST be production-grade, not a minimal MVP.
</objective>

<non_goals>
- No evidence-free checkbox completion.
- No breadth-only scaffold inflation presented as completion.
- No stale status claims in conformance, limitations, or release docs.
- No type-only implementations that are not reachable from user input.
- No shortcutting: every feature specified in the docs MUST be wired into real user-facing paths.
</non_goals>

<authority_and_precedence>
Use this precedence when instructions conflict:

1. `/docs/policy/`
2. `/docs/spec/`
3. `/docs/reference/CONFORMANCE*.md` and `/docs/reference/LIMITATIONS.md`
4. `/docs/todo/current/`
5. All other docs

Canonical facts:
- Target behavior: `/docs/spec/`
- Current supported behavior: `/docs/reference/CONFORMANCE.md`
- User-visible gaps: `/docs/reference/LIMITATIONS.md`
- Anti-MVP measures: `/docs/log/proposals/anti-mvp-measures.md`
- Deep wiring checklist (part 1): `/docs/log/proposals/deep-wiring-checklist.md`
- Deep wiring checklist (part 2): `/docs/log/proposals/deep-wiring-checklist-2.md`
</authority_and_precedence>

<required_start_reading_order>
1. `/docs/README.md`
2. `/docs/policy/README.md`
3. `/docs/policy/ROOT_LAYOUT.md`
4. `/docs/policy/STRUCTURE.md`
5. `/docs/policy/WORKFLOW.md`
6. `/docs/spec/README.md`
7. `/docs/spec/technical/testing.md`
8. `/docs/spec/technical/testing-unit.md`
9. `/docs/spec/technical/testing-e2e.md`
10. `/docs/reference/CONFORMANCE.md`
11. `/docs/reference/LIMITATIONS.md`
12. `/docs/log/proposals/anti-mvp-measures.md`
13. `/docs/log/proposals/deep-wiring-checklist.md`
14. `/docs/log/proposals/deep-wiring-checklist-2.md`
15. `/docs/todo/README.md`
</required_start_reading_order>

<critical_specifications>
These specs define behavior that was previously implemented incorrectly. Read carefully:

- Cursor semantics with CJK: `/docs/spec/editing/cursor/README.md`
- Terminal emulator (full VT100): `/docs/spec/features/terminal/terminal.md`
- Escape parser state machine: `/docs/spec/features/terminal/escape-parser.md`
- Windows (buffer + terminal): `/docs/spec/editor/windows.md`
- Viewport wrapping with CJK: `/docs/spec/features/ui/viewport.md`
- Session JSON format: `/docs/spec/features/session/sessions.md`
- Startup sequence: `/docs/spec/architecture/startup.md`
- Render pipeline: `/docs/spec/architecture/render-pipeline.md`
- Unicode guidance: `/docs/technical/unicode.md`
- Deep wiring per-crate inventory (part 1): `/docs/log/proposals/deep-wiring-checklist.md`
- Deep wiring per-crate inventory (part 2): `/docs/log/proposals/deep-wiring-checklist-2.md`
</critical_specifications>

<execution_model>
Work in strict gates. Do not proceed to next gate on red checks.

Gate 0: Baseline audit
- Run verification commands from `/docs/reference/CI.md`.
- Create mismatch matrix and classify: M1 correctness, M2 missing feature, M3 undocumented behavior, M4 verification gap, M5 stale docs.
- Fix priority: M1 -> user-visible M2 -> touched-area M4 -> M3/M5.

Gate 1: Slice definition
- Choose one coherent slice from `/docs/todo/README.md`.
- Define acceptance criteria with exact `/docs/spec/...` references.
- Define required tests per `/docs/spec/technical/testing.md`.

Gate 2: Implement
- Wire behavior through real user-reachable paths.
- Verify each feature is callable from the binary's `main` function through real user input.
- Keep all source files under 200 lines per `/docs/policy/STRUCTURE.md`.
- Keep root-absolute links (`/docs/...`).
- Avoid disconnected type-only work unless explicitly marked `scaffold-only`.

Gate 3: Verify and synchronize docs
- Run touched tests first, then full gate.
- Update conformance and limitations in same change.
- Check TODO items only after evidence is green.
</execution_model>

<anti_gaming_rules>
Prohibited:
- Evidence-free completion.
- Claiming "implemented" for unreachable behavior.
- Low-signal test inflation for count targets.
- Broad stubs presented as shipped functionality.
- Destructive resets used to hide regressions.
- Marking a TODO complete when the feature only exists as types/structs.
- Marking a TODO complete when the feature is not wired into the main dispatch loop.

Implemented means ALL are true:
- Reachable via documented command/key workflow from the running binary.
- User-visible behavior matches spec expectation.
- Deterministic regression coverage exists.
- Conformance and limitations are accurate.
- Code volume meets the minimums in `/docs/log/proposals/anti-mvp-measures.md`.
</anti_gaming_rules>

<testing_contract>
Follow `/docs/spec/technical/testing.md` as normative:
- Per-crate unit tests per `/docs/spec/technical/testing-unit.md`.
- E2E and boundary tests per `/docs/spec/technical/testing-e2e.md`.
- Each bug fix adds a regression test that fails on old behavior.
- Prefer persisted-state assertions over fragile screen scraping.
- Enforce deterministic deadlines and actionable timeout diagnostics.
- Keep mandatory PTY boundary scenarios and multiplexer smoke coverage.
</testing_contract>

<todo_deferral_rules>
For intentional deferral:
- Record rationale under `/docs/log/proposals/`.
- Add concrete next-iteration leaf task.
- Then mark the deferral item complete.
- Do not leave terminal deferred buckets without actionable carry-forward.
</todo_deferral_rules>

<required_audit_artifacts>
During each run, produce or update one audit record under `/docs/log/reconstruction/audits/`. Include:
- Mismatch matrix.
- Closed mismatches with evidence.
- Deferred mismatches with rationale and next action.
- Exact verification commands and result signals.
</required_audit_artifacts>

<acceptance_criteria>
Accept only when ALL are true:
- Behavior matches selected target and linked specs.
- Conformance/limitations match observed behavior.
- Required deterministic tests are green at all layers.
- TODO checkboxes reflect proven completion.
- Terminal emulator spawns real PTY processes.
- Session save/load produces/reads valid JSON per schema.
- CJK cursor never occupies half-cell position.
- Long lines wrap correctly with CJK boundary padding.
- Terminal multiplexer contract is verified or explicitly limited with closure plan.
- Code volume meets minimums per `/docs/log/proposals/anti-mvp-measures.md`.
</acceptance_criteria>
