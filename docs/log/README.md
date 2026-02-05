# Log

Back: [/docs/README.md](/docs/README.md)
Structured project log for implementation work, decisions, and audits.

## Purpose

Capture non-normative work products that are useful for reconstruction and iteration, without polluting the canonical spec.

This directory may be pruned in some workflows, but when it is used it SHOULD stay structured and link-driven.

## Directory map

| Path | Content |
|---|---|
| [proposals/README.md](proposals/README.md) | Improvement proposals for gaps/bugs/perf work (design + test plan). |
| [audits/README.md](audits/README.md) | Audits of doc policy compliance, TODO completeness, and drift checks. |

## What to update instead of logs

When implementation is created or changed, update:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) (what is implemented)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (user-visible gaps)
- [/docs/todo/current/README.md](/docs/todo/current/README.md) (next work)

When a proposal becomes normative, it MUST be migrated into the canonical spec under `/docs/spec/` (and corresponding TODO leaves updated).

## Related

- Docs-only reconstruction contract: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
