# Fundamental Intent Catalog

This catalog is authoritative for restructuring intent IDs.

| Intent ID | Statement | Primary Verification |
| --- | --- | --- |
| <a id="fi-00-topology-contract-is-explicit"></a>FI-00 | Every docs directory has one TOC `README.md`. | Topology check (`T12-topology-check`) |
| <a id="fi-01-ordering-is-deterministic"></a>FI-01 | Phase execution order is deterministic and zero-padded (`00`..`09`). | Phase order check (`T02-phase-order`) |
| <a id="fi-02-phase-shape-is-normalized"></a>FI-02 | Every phase uses the same six-section contract. | Section-shape check (`T03-section-shape`) |
| <a id="fi-03-fundamental-intents-are-explicit"></a>FI-03 | Fundamental intents are explicit, named, and uniquely identified. | Intent catalog check (`T04-intent-catalog`) |
| <a id="fi-04-phase-to-intent-mapping-is-total"></a>FI-04 | Every phase maps to at least one fundamental intent. | Phase-intent mapping check (`T05-phase-intent-links`) |
| <a id="fi-05-testing-is-interleaved-not-terminal"></a>FI-05 | Testing is interleaved between phases, not deferred to the end. | Schedule-order check (`T06-schedule-order`) |
| <a id="fi-06-coverage-matrix-covers-all-docs-markdown"></a>FI-06 | Coverage matrix directly references every `docs/**/*.md` file at least once. | Coverage completeness check (`T08-coverage-completeness`) |
| <a id="fi-07-entrypoints-link-to-restructuring-docs"></a>FI-07 | Root and docs entrypoints both link restructuring docs. | Entrypoint link checks (`T10-root-link`, `T11-docs-link`) |
| <a id="fi-08-validation-is-explicit-and-repeatable"></a>FI-08 | Validation commands and expected outcomes are explicit and repeatable. | CLI validation checks (`T12`..`T14`) |
| <a id="fi-09-governance-docs-track-topology-changes"></a>FI-09 | Governance/layout docs track restructuring topology changes. | Layout contract check (`T15-layout-contract`) |
| <a id="fi-10-runtime-tests-compose-converge"></a>FI-10 | Runtime behavior, tests, and compose verification remain converged on replay. | Runtime/test/compose checks (`T18`..`T20`) |
