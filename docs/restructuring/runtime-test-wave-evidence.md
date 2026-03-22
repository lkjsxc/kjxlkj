# Runtime Test Wave Evidence

This record captures deterministic convergence between docs, runtime surfaces, tests, and CLI gates.

## Contract Baseline

- Docs-first governance remains authoritative:
  - [Change Policy](../repository/governance/change-policy.md)
  - [Root Layout](../repository/structure/root-layout.md)
- Runtime convergence references:
  - [Route Topology](../architecture/runtime/route-topology.md)
  - [UI Interaction Contract](../architecture/runtime/ui-interaction-contract.md)
  - [Product Surface Map](../product/surface-map.md)
  - [Admin HTMX Contracts](../product/flows/admin-htmx-contracts.md)
  - [Admin JS UX Contract](../product/flows/admin-js-ux-contract.md)

## Runtime Delivery Evidence

| Capability area | Delivered runtime files | Verification tests | Result |
| --- | --- | --- | --- |
| Setup/login/admin lifecycle | [src/web/router.rs](../../src/web/router.rs), [src/web/handlers/setup.rs](../../src/web/handlers/setup.rs), [src/web/handlers/auth.rs](../../src/web/handlers/auth.rs) | [tests/workflow.rs](../../tests/workflow.rs) | **PASS** |
| Inline direct edit on article page | [src/web/handlers/public.rs](../../src/web/handlers/public.rs), [src/web/handlers/article_edit.rs](../../src/web/handlers/article_edit.rs), [src/web/handlers/article_page.rs](../../src/web/handlers/article_page.rs) | [tests/workflow.rs](../../tests/workflow.rs) | **PASS** |
| History + restore + navigation | [src/web/stores/content/history.rs](../../src/web/stores/content/history.rs), [src/web/stores/content/base.rs](../../src/web/stores/content/base.rs), [src/web/handlers/article_history_page.rs](../../src/web/handlers/article_history_page.rs) | [tests/workflow.rs](../../tests/workflow.rs) | **PASS** |
| Private-by-default timeline metadata | [src/core/content/frontmatter.rs](../../src/core/content/frontmatter.rs), [migrations/0003_article_timeline_history.sql](../../migrations/0003_article_timeline_history.sql), [src/adapters/postgres/search_repo.rs](../../src/adapters/postgres/search_repo.rs) | [tests/workflow.rs](../../tests/workflow.rs), [src/core/content/frontmatter_tests.rs](../../src/core/content/frontmatter_tests.rs) | **PASS** |
| CLI convergence checks | [src/cli/runner.rs](../../src/cli/runner.rs), [src/cli/topology.rs](../../src/cli/topology.rs), [src/cli/line_limits/scan.rs](../../src/cli/line_limits/scan.rs), [src/cli/compose.rs](../../src/cli/compose.rs) | [src/cli/topology_tests.rs](../../src/cli/topology_tests.rs), [src/cli/line_limits_tests.rs](../../src/cli/line_limits_tests.rs), [src/cli/compose_tests.rs](../../src/cli/compose_tests.rs) | **PASS** |

## Verification Outcomes

| Gate | Command | Outcome |
| --- | --- | --- |
| Docs topology | `cargo run --bin kjxlkj -- docs validate-topology` | **PASS** |
| Restricted-language scan | `cargo run --bin kjxlkj -- docs validate-terms` | **PASS** |
| Line limits | `cargo run --bin kjxlkj -- quality check-lines` | **PASS** |
| Runtime tests | `cargo test -q` | **PASS** |
| Compose verify | `docker compose --profile verify run --rm verify` | **PASS** |
| Compose CLI wrapper | `cargo run --bin kjxlkj -- compose verify` | **PASS** |

Deterministic replay rule: run gates exactly in table order; any non-pass result blocks status completion.
