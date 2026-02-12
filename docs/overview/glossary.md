# Glossary

Back: [/docs/overview/README.md](/docs/overview/README.md)

Shared terms used across policy/spec/reference/todo.

| Term | Meaning |
|---|---|
| Note stream | Logical note identity with ordered events and current version. |
| Note projection | Materialized current view of a note for API reads. |
| Note event | Immutable append-only record of a note mutation. |
| Patch operation | Retain/insert/delete operation in patch stream protocol. |
| Backlink | Reverse link index from target note to referring notes. |
| Typed metadata | Keyed structured data attached to a note. |
| Single-tenant auth | One account scope for a deployment, with authenticated sessions. |
| First-run registration | Setup endpoint active only before first account exists. |
| Single-service container | One compose service process space running app and PostgreSQL. |
| Drift matrix | Requirement-level mismatch tracking ledger. |
