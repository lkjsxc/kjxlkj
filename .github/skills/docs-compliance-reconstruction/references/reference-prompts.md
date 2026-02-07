# Reference Prompts

## Canonical Prompt Source

Use `/docs/todo/RECONSTRUCTION_PROMPT.md` as the normative one-shot prompt contract.
Do not replace it with a looser interpretation.

## Single-Prompt Template: Full Reconstruction

Use this when rebuilding or broadly reworking the repository:

```text
Use $docs-compliance-reconstruction.
Objective: Rebuild the implementation from documentation and complete all active TODO work for this iteration.
Hard rule: Follow /docs/todo/RECONSTRUCTION_PROMPT.md exactly for instructions, constraints, and acceptance criteria.
```

## Single-Prompt Template: Targeted Implementation

Use this when implementing a specific feature or fix with the same quality bar:

```text
Use $docs-compliance-reconstruction.
Objective: Implement <feature-or-fix> in full compliance with /docs/spec and /docs/policy.
Hard rule: Apply the quality, autonomy, and verification requirements from /docs/todo/RECONSTRUCTION_PROMPT.md before declaring completion.
```

## Prompt Mapping Notes

- Docs-first run contract: `/docs/overview/all-in-docs.md`
- Active TODO control surface: `/docs/todo/current/README.md`
- Verification gate: `/docs/reference/CI.md`
- Drift tracking outputs: `/docs/reference/CONFORMANCE.md` and `/docs/reference/LIMITATIONS.md`
