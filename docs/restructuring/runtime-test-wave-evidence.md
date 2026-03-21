# Runtime Test Wave Evidence

This document records closure evidence for todo `setup-evidence-sync` after the setup UX fix and setup-flow test split.

## Runtime Contract Alignment

- Setup-first invariant reference: [Setup-First Contract](../vision/setup-first.md)
- Runtime routing reference: [Route Topology](../architecture/runtime/route-topology.md)
- User flow references:
  - [Setup Flow](../product/flows/setup-flow.md)
  - [Public Site Flow](../product/flows/public-site.md)
  - [Product Surface Map](../product/surface-map.md)

## Current Test Evidence (post-split)

| Scope | Result | Evidence |
| --- | --- | --- |
| Setup-flow integration suite | **PASS** | Entrypoint: [tests/setup_flow.rs](../../tests/setup_flow.rs). Modules: [tests/setup_flow/get_routes.rs](../../tests/setup_flow/get_routes.rs), [tests/setup_flow/post_setup.rs](../../tests/setup_flow/post_setup.rs), [tests/setup_flow/setup_lock.rs](../../tests/setup_flow/setup_lock.rs). |
| Public/private visibility regression | **PASS** | [tests/public_private_visibility.rs](../../tests/public_private_visibility.rs) confirms pre-setup redirect behavior and post-setup visibility rules. |

### Key setup-flow outcomes captured

- `GET /` before setup returns `302 Found` with `Location: /setup`.
- `GET /setup` returns `200 OK` and renders full HTML setup form.
- Valid `POST /setup` returns `303 See Other` with `Location: /login`.
- Invalid setup payloads return deterministic validation output.
- After first admin creation, setup is locked (`GET`/`POST /setup` return `404 Not Found`).

## Docker Functional Recheck Evidence

Recorded docker-functional-recheck outcomes:

- `302` `/ -> /setup` check: **PASS**
- `200` setup HTML form check: **PASS**
- `303` setup `POST` redirect to `/login` check: **PASS**
- Verify profile run (`docker compose --profile verify run --rm verify`): **PASS**
- CLI compose verify (`cargo run --bin kjxlkj -- compose verify`): **PASS**

## Documentation Constraint Check

- This document is concise and remains well under the 300-line limit.
