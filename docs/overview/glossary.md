# Glossary

Back: [/docs/overview/README.md](/docs/overview/README.md)

Shared terms used across policy/spec/reference/todo.

| Term | Meaning |
|---|---|
| Workspace | Top-level collaboration boundary in one deployment tenant. |
| Project | Workspace-scoped grouping for notes and workflows. |
| Workspace member | User assigned a role in a workspace. |
| Note stream | Logical note identity with ordered events and current version. |
| Note projection | Materialized current view of a note for API reads. |
| Note event | Immutable append-only record of a note mutation. |
| Patch operation | Retain/insert/delete operation in patch stream protocol. |
| Backlink | Reverse link index from target note to referring notes. |
| Saved view | Persisted query/filter/sort definition for workspace navigation. |
| Dashboard widget | Optional configurable panel shown in workspace dashboards. |
| Automation rule | Trigger-condition-action definition for deterministic background work. |
| Automation run | Auditable execution record for one rule trigger event. |
| Librarian agent | Automation action that restructures notes/docs with LLM-guided operations. |
| Attribute-less XML-like protocol | Tag-only envelope format without XML attributes (`xml_attrless`). |
| Provider adapter | OpenAI-compatible integration layer for OpenRouter and LM Studio. |
| Typed metadata | Keyed structured data attached to a note. |
| Single-tenant multi-user auth | One deployment tenant with multiple authenticated users/roles. |
| First-run registration | Setup endpoint active only before first owner account exists. |
| Single-service container | One compose service process space running app and SQLite-backed runtime data. |
| Drift matrix | Requirement-level mismatch tracking ledger. |
