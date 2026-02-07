# Reconstruction Prompt Hardening Audit (2026-02-07)

Back: [/docs/log/reconstruction/audits/README.md](/docs/log/reconstruction/audits/README.md)

## Scope

Harden reconstruction governance to prevent false completion and stale status drift.

Primary files:

- `/docs/todo/RECONSTRUCTION_PROMPT.md`
- `/docs/reference/README.md`
- `/docs/reference/CONFORMANCE.md`
- `/docs/reference/LIMITATIONS.md`
- `/docs/reference/COMPARISON.md`
- `/docs/reference/PLUGIN_MAPPING.md`
- `/docs/reference/RELEASE.md`

## Mismatch matrix (closed in this wave)

| Requirement ID | Canonical doc path | Requirement statement | Code path(s) | Test path(s) | Observed status | Mismatch class | Action | Verification evidence |
|---|---|---|---|---|---|---|---|---|
| RP-01 | `/docs/todo/RECONSTRUCTION_PROMPT.md` | Prompt must enforce behavior-first implementation, not checkbox completion | N/A | N/A | contradiction | M5 stale docs | spec-update | prompt sections `anti_gaming_rules`, `execution_model`, `acceptance_criteria` |
| RP-02 | `/docs/todo/RECONSTRUCTION_PROMPT.md` | Prompt must require deterministic evidence artifacts per run | N/A | N/A | spec-only | M3 undocumented behavior | spec-update | new `required_audit_artifacts` section |
| REF-01 | `/docs/reference/README.md` | Reference authority order must be explicit | N/A | N/A | contradiction | M5 stale docs | spec-update | new `Authority model (normative)` section |
| REF-02 | `/docs/reference/CONFORMANCE.md` | Conformance claims must be evidence-gated and use strict status vocabulary | N/A | N/A | spec-only | M3 undocumented behavior | spec-update | new `Status vocabulary` + `Claim admission gate` |
| REF-03 | `/docs/reference/LIMITATIONS.md` | Limitation entries should carry expected/observed/evidence/next action | N/A | N/A | spec-only | M3 undocumented behavior | spec-update | new `Entry discipline (normative)` section |
| REF-04 | `/docs/reference/COMPARISON.md` | Non-authoritative docs must not hardcode current implementation claims | N/A | N/A | contradiction | M5 stale docs | spec-update | replaced current-vs-target hard claims with status-source links |
| REF-05 | `/docs/reference/PLUGIN_MAPPING.md` | Plugin mapping must source current status from conformance/limitations | N/A | N/A | contradiction | M5 stale docs | spec-update | replaced `Current status` values with `Current status source` |
| REF-06 | `/docs/reference/RELEASE.md` | Release checklist must enforce conformance evidence gate | N/A | N/A | test-gap | M4 verification gap | spec-update | new release checklist step requiring evidence-gated conformance |

## Closed mismatches with proof

- RP-01 closed by explicit prohibitions on evidence-free completion and unreachable-code completion claims.
- RP-02 closed by mandatory audit artifact requirement in reconstruction prompt.
- REF-01 closed by normative authority hierarchy in reference index.
- REF-02 closed by formal conformance status vocabulary and claim admission gate.
- REF-03 closed by limitations entry schema and conformance-linking rule.
- REF-04 and REF-05 closed by removing static status claims from non-authoritative docs.
- REF-06 closed by release gate requiring conformance-evidence consistency checks.

## Deferred mismatches

- LIM-NORM-01: legacy entries in `/docs/reference/LIMITATIONS.md` remain heterogeneous and are not fully normalized to the new schema.
  - Rationale: large ledger; safe incremental migration preferred.
  - Next action: normalize high-priority UX defect entries first in next docs-verification wave.

## Verification evidence

- Check: `python .github/scripts/check_docs_policy.py`
- Result: pass
- Proof: `Documentation policy check passed.`

- Check: `wc -l docs/todo/RECONSTRUCTION_PROMPT.md`
- Result: pass
- Proof: `157 docs/todo/RECONSTRUCTION_PROMPT.md`

- Check: `LC_ALL=C grep -n '[^ -~]' docs/todo/RECONSTRUCTION_PROMPT.md docs/reference/README.md docs/reference/CONFORMANCE.md docs/reference/RELEASE.md docs/reference/COMPARISON.md docs/reference/PLUGIN_MAPPING.md || true`
- Result: pass
- Proof: no output
