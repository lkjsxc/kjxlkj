# Runtime Test Wave Evidence

This document records closure evidence for todo `final-docs-evidence-sync` after docs-first hardening, runtime delivery, docker acceptance, and final quality gates.

## Docs-First Hardening Summary

- Docs-first sequencing remained authoritative through the [Change Policy Contract](../repository/governance/change-policy.md) and [Root Layout Contract](../repository/structure/root-layout.md).
- Runtime behavior contracts were hardened and cross-linked before and during implementation:
  - [Setup-First Contract](../vision/setup-first.md)
  - [Route Topology](../architecture/runtime/route-topology.md)
  - [UI Interaction Runtime Contract](../architecture/runtime/ui-interaction-contract.md)
  - [Server-Rendered Page Contracts](../product/flows/page-contracts.md)
  - [Admin HTMX Fragment Contracts](../product/flows/admin-htmx-contracts.md)
  - [Admin JavaScript UX Contract](../product/flows/admin-js-ux-contract.md)
  - [Admin Conflict Warning Contract](../product/flows/admin-conflict-warning.md)
- Pre-implementation docs gate (`docs-preimplementation-gate`) was recorded as **PASS** with:
  - `cargo run --bin kjxlkj -- docs validate-topology` → pass (`directories_checked=21`, `violations=0`)
  - `cargo run --bin kjxlkj -- quality check-lines` → pass (`docs_files_checked=74`, `violations=0`)
  - changed-doc local link/anchor sanity → pass (`errors=0`)

## Runtime Delivery Evidence (Full Page + HTMX + JavaScript + Conflict)

| Capability area | Delivered runtime files | Verification tests | Result |
| --- | --- | --- | --- |
| Full-page routing + setup/auth guards | [src/web/router.rs](../../src/web/router.rs), [src/web/handlers/admin_page.rs](../../src/web/handlers/admin_page.rs) | [tests/full_page_rendering.rs](../../tests/full_page_rendering.rs), [tests/public_private_visibility.rs](../../tests/public_private_visibility.rs), [tests/setup_flow.rs](../../tests/setup_flow.rs), [tests/setup_flow/get_routes.rs](../../tests/setup_flow/get_routes.rs), [tests/setup_flow/post_setup.rs](../../tests/setup_flow/post_setup.rs), [tests/setup_flow/setup_lock.rs](../../tests/setup_flow/setup_lock.rs) | **PASS** |
| HTMX admin open/preview/save/create/rename/delete/toggle flows | [src/web/router.rs](../../src/web/router.rs), [src/web/handlers/admin.rs](../../src/web/handlers/admin.rs), [src/web/handlers/admin_fragments.rs](../../src/web/handlers/admin_fragments.rs) | [tests/admin_htmx_open_preview_create.rs](../../tests/admin_htmx_open_preview_create.rs), [tests/admin_htmx_save_mutations.rs](../../tests/admin_htmx_save_mutations.rs), [tests/admin_htmx_runtime.rs](../../tests/admin_htmx_runtime.rs), [tests/admin_htmx_guards.rs](../../tests/admin_htmx_guards.rs) | **PASS** |
| JavaScript autosave/unsaved-guard/shortcut runtime | [src/web/static/admin_runtime_core.js](../../src/web/static/admin_runtime_core.js), [src/web/static/admin_runtime_autosave.js](../../src/web/static/admin_runtime_autosave.js), [src/web/static/admin_runtime_shortcuts.js](../../src/web/static/admin_runtime_shortcuts.js), [src/web/handlers/admin_page.rs](../../src/web/handlers/admin_page.rs) | [tests/admin_js_runtime.rs](../../tests/admin_js_runtime.rs), [tests/admin_js_contracts.rs](../../tests/admin_js_contracts.rs) | **PASS** |
| Last-write-wins conflict signaling + telemetry | [src/web/stores/mod.rs](../../src/web/stores/mod.rs), [src/web/handlers/admin_fragments.rs](../../src/web/handlers/admin_fragments.rs) | [tests/admin_conflict_warning.rs](../../tests/admin_conflict_warning.rs), [tests/admin_htmx_runtime.rs](../../tests/admin_htmx_runtime.rs), [tests/admin_htmx_save_mutations.rs](../../tests/admin_htmx_save_mutations.rs) | **PASS** |

## Docker Acceptance Verification Outcomes

Docker acceptance followed [Compose Commands](../containers/compose/commands.md) and [Operations Automation Contract](../operations/automation.md).

| Acceptance check | Outcome | Evidence |
| --- | --- | --- |
| `GET /` setup-first redirect (`302` to `/setup`) | **PASS** | Functional recheck record and setup-first contract alignment |
| `GET /setup` serves full HTML setup form | **PASS** | Functional recheck record and setup-flow contract alignment |
| Valid `POST /setup` redirects to `/login` (`303`) | **PASS** | Functional recheck record and setup-flow contract alignment |
| `docker compose --profile verify run --rm verify` | **PASS** | Verify profile completion without failures |
| `cargo run --bin kjxlkj -- compose verify` | **PASS** | Wrapper summary reported `steps_passed=3` of `steps_total=3` |

## Final Quality Gate Outcomes

| Gate | Command | Outcome | Recorded output |
| --- | --- | --- | --- |
| Topology | `cargo run --bin kjxlkj -- docs validate-topology` | **PASS** | `{"command":"docs.validate-topology","directories_checked":21,"status":"pass","violations":0}` |
| Line limits | `cargo run --bin kjxlkj -- quality check-lines` | **PASS** | `{"command":"quality.check-lines","docs_files_checked":82,"source_files_checked":74,"status":"pass","violations":0}` |

Gate decision: **PASS** — docs-first hardening, runtime implementation evidence, docker acceptance, and final quality checks are synchronized.

## Key Test/File Reference Index

- Runtime contracts and governance:
  - [docs/repository/governance/change-policy.md](../repository/governance/change-policy.md)
  - [docs/repository/structure/root-layout.md](../repository/structure/root-layout.md)
  - [docs/architecture/runtime/ui-interaction-contract.md](../architecture/runtime/ui-interaction-contract.md)
- Runtime implementation anchors:
  - [src/web/router.rs](../../src/web/router.rs)
  - [src/web/handlers/admin_page.rs](../../src/web/handlers/admin_page.rs)
  - [src/web/handlers/admin_fragments.rs](../../src/web/handlers/admin_fragments.rs)
  - [src/web/stores/mod.rs](../../src/web/stores/mod.rs)
  - [src/web/static/admin_runtime_core.js](../../src/web/static/admin_runtime_core.js)
  - [src/web/static/admin_runtime_autosave.js](../../src/web/static/admin_runtime_autosave.js)
  - [src/web/static/admin_runtime_shortcuts.js](../../src/web/static/admin_runtime_shortcuts.js)
- End-to-end runtime contract tests:
  - [tests/full_page_rendering.rs](../../tests/full_page_rendering.rs)
  - [tests/public_private_visibility.rs](../../tests/public_private_visibility.rs)
  - [tests/admin_htmx_open_preview_create.rs](../../tests/admin_htmx_open_preview_create.rs)
  - [tests/admin_htmx_save_mutations.rs](../../tests/admin_htmx_save_mutations.rs)
  - [tests/admin_htmx_runtime.rs](../../tests/admin_htmx_runtime.rs)
  - [tests/admin_htmx_guards.rs](../../tests/admin_htmx_guards.rs)
  - [tests/admin_js_runtime.rs](../../tests/admin_js_runtime.rs)
  - [tests/admin_js_contracts.rs](../../tests/admin_js_contracts.rs)
  - [tests/admin_conflict_warning.rs](../../tests/admin_conflict_warning.rs)
  - [tests/setup_flow.rs](../../tests/setup_flow.rs)
