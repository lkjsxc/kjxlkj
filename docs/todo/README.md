# Reconstruction TODO — Master Execution Contract

**Authority:** This is the ONLY allowed implementation order. Following these TODOs with their linked docs MUST organically produce full spec compliance.

---

## Relevant Documents (Complete Index)

### Policy Layer (Governance)
- [Policy Root](/docs/policy/README.md) — governance index
- [INSTRUCT.md](/docs/policy/INSTRUCT.md) — operating contract
- [WORKFLOW.md](/docs/policy/WORKFLOW.md) — gate sequence
- [STRUCTURE.md](/docs/policy/STRUCTURE.md) — directory/file constraints
- [ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) — allowed root paths

### Specification Layer (Target Behavior)
- [Spec Root](/docs/spec/README.md) — specification index
- [Architecture](/docs/spec/architecture/README.md) — runtime topology
  - [configuration.md](/docs/spec/architecture/configuration.md) — config loading
  - [runtime.md](/docs/spec/architecture/runtime.md) — startup sequence
  - [crates.md](/docs/spec/architecture/crates.md) — Rust decomposition
  - [source-layout.md](/docs/spec/architecture/source-layout.md) — module splits
  - [workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) — Cargo policy
  - [deployment.md](/docs/spec/architecture/deployment.md) — process hosting
  - [completion-file-map.md](/docs/spec/architecture/completion-file-map.md) — path requirements
  - [final-file-structure.md](/docs/spec/architecture/final-file-structure.md) — tree states
- [API](/docs/spec/api/README.md) — HTTP/WS contracts
  - [http.md](/docs/spec/api/http.md) — REST endpoints
  - [websocket.md](/docs/spec/api/websocket.md) — realtime protocol
  - [types.md](/docs/spec/api/types.md) — payload schemas
  - [errors.md](/docs/spec/api/errors.md) — error model
  - [openapi.md](/docs/spec/api/openapi.md) — OpenAPI governance
  - [openapi.yaml](/docs/spec/api/openapi.yaml) — canonical OpenAPI
  - [librarian-xml.md](/docs/spec/api/librarian-xml.md) — agent XML protocol
- [Domain](/docs/spec/domain/README.md) — business rules
  - [workspaces.md](/docs/spec/domain/workspaces.md) — workspace lifecycle
  - [projects.md](/docs/spec/domain/projects.md) — project scoping
  - [permissions.md](/docs/spec/domain/permissions.md) — RBAC model
  - [notes.md](/docs/spec/domain/notes.md) — note lifecycle (ID + title separation)
  - [note-types.md](/docs/spec/domain/note-types.md) — kind taxonomy
  - [events.md](/docs/spec/domain/events.md) — event sourcing
  - [metadata.md](/docs/spec/domain/metadata.md) — typed metadata
  - [automation.md](/docs/spec/domain/automation.md) — kjxlkj-agent runs
  - [attachments.md](/docs/spec/domain/attachments.md) — file handling
  - [search.md](/docs/spec/domain/search.md) — hybrid search (REDESIGNED)
  - [export.md](/docs/spec/domain/export.md) — backup/export
- [Security](/docs/spec/security/README.md) — auth/session/transport
  - [auth.md](/docs/spec/security/auth.md) — first-run + login
  - [sessions.md](/docs/spec/security/sessions.md) — cookie lifecycle
  - [csrf.md](/docs/spec/security/csrf.md) — CSRF enforcement
  - [transport.md](/docs/spec/security/transport.md) — TLS/proxy
- [Technical](/docs/spec/technical/README.md) — cross-cutting
  - [testing.md](/docs/spec/technical/testing.md) — verification tiers + acceptance IDs
  - [performance.md](/docs/spec/technical/performance.md) — latency targets
  - [migrations.md](/docs/spec/technical/migrations.md) — schema discipline
  - [operations.md](/docs/spec/technical/operations.md) — health/backup/observability
  - [librarian-agent.md](/docs/spec/technical/librarian-agent.md) — kjxlkj-agent loop
  - [agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) — prompt schema
  - [type-safety.md](/docs/spec/technical/type-safety.md) — Rust/TS gates
  - [librarian-prompts/](/docs/spec/technical/librarian-prompts/README.md) — prompt packs
- [UI](/docs/spec/ui/README.md) — web app UX
  - [web-app.md](/docs/spec/ui/web-app.md) — shell + auth + note creation
  - [editor-flow.md](/docs/spec/ui/editor-flow.md) — Obsidian-like markdown editor
  - [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) — responsive (2/3 threshold)
  - [findings-traceability.md](/docs/spec/ui/findings-traceability.md) — finding mapping
  - [reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) — UX matrix
  - [workspace-suite.md](/docs/spec/ui/workspace-suite.md) — optional modules

### Reference Layer (Verified State)
- [Reference Root](/docs/reference/README.md) — state index
- [CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — verified behavior
- [LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — open gaps
- [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) — mismatch tracking
- [CI.md](/docs/reference/CI.md) — verification profiles
- [RELEASE.md](/docs/reference/RELEASE.md) — release gate
- [EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) — proof artifacts
- [IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md) — backlog

### Overview Layer (Orientation)
- [Overview Root](/docs/overview/README.md) — orientation index
- [all-in-docs.md](/docs/overview/all-in-docs.md) — docs-first doctrine
- [principles.md](/docs/overview/principles.md) — design principles
- [glossary.md](/docs/overview/glossary.md) — shared terms

### Guides Layer (Operator Playbooks)
- [Guides Root](/docs/guides/README.md) — guides index
- [QUICKSTART.md](/docs/guides/QUICKSTART.md) — first-run workflow
- [DOCKER.md](/docs/guides/DOCKER.md) — local orchestration
- [API.md](/docs/guides/API.md) — API usage
- [LIBRARIAN.md](/docs/guides/LIBRARIAN.md) — agent operation
- [RECONSTRUCTION_BOOTSTRAP.md](/docs/guides/RECONSTRUCTION_BOOTSTRAP.md) — scaffold guide

---

## Execution Rules

1. **Execute stages ONLY in listed order** — per [WORKFLOW.md](/docs/policy/WORKFLOW.md)
2. **Before closing each wave:** run `cargo build --workspace` then `cargo test --workspace` — per [CI.md](/docs/reference/CI.md)
3. **Verify acceptance IDs** — per [testing.md](/docs/spec/technical/testing.md)
4. **Sync ledgers AFTER each stage** — update [CONFORMANCE.md](/docs/reference/CONFORMANCE.md), [LIMITATIONS.md](/docs/reference/LIMITATIONS.md), [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
5. **TODO link policy** — every checkbox MUST link to governing doc — per [STRUCTURE.md](/docs/policy/STRUCTURE.md)

---

## Ordered Stages — Complete Checklist

### Stage S00: Governance Baseline
**Objective:** Establish policy foundation and docs integrity gates.
**Wave Files:** [stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md)
- [ ] S00-W000: policy tree scaffold — [wave-000.md](/docs/todo/waves/stage-00-pivot-governance/wave-000.md)
- [ ] S00-W001: docs link integrity — [wave-001.md](/docs/todo/waves/stage-00-pivot-governance/wave-001.md)
- [ ] S00-W002: structure enforcement — [wave-002.md](/docs/todo/waves/stage-00-pivot-governance/wave-002.md)

### Stage S01: Runtime Skeleton
**Objective:** Rebuild Cargo workspace and crate topology from spec.
**Wave Files:** [stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)
- [ ] S01-W010: Cargo.toml + crate dirs — [wave-010.md](/docs/todo/waves/stage-01-spec-rebuild/wave-010.md)
- [ ] S01-W011: domain types + events — [wave-011.md](/docs/todo/waves/stage-01-spec-rebuild/wave-011.md)
- [ ] S01-W012: in-memory repos — [wave-012.md](/docs/todo/waves/stage-01-spec-rebuild/wave-012.md)

### Stage S02: Notes + Search Core
**Objective:** Implement note lifecycle and redesigned hybrid search.
**Wave Files:** [stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)
- [ ] S02-W020: note CRUD + ID/title separation — [wave-020.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-020.md)
- [ ] S02-W021: event sourcing + projections — [wave-021.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-021.md)
- [ ] S02-W022: redesigned search (vector + lexical) — [wave-022.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-022.md)

### Stage S03: Realtime Integration
**Objective:** HTTP + WebSocket protocol with replay semantics.
**Wave Files:** [stage-03-runtime-integration/README.md](/docs/todo/waves/stage-03-runtime-integration/README.md)
- [ ] S03-W030: HTTP routes + handlers — [wave-030.md](/docs/todo/waves/stage-03-runtime-integration/wave-030.md)
- [ ] S03-W031: WebSocket cursor replay — [wave-031.md](/docs/todo/waves/stage-03-runtime-integration/wave-031.md)
- [ ] S03-W032: idempotency + conflict UX — [wave-032.md](/docs/todo/waves/stage-03-runtime-integration/wave-032.md)

### Stage S04: Automation + Agent
**Objective:** kjxlkj-agent loop with JSON prompts and KV memory.
**Wave Files:** [stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)
- [ ] S04-W040: agent loop scaffold — [wave-040.md](/docs/todo/waves/stage-04-schema-and-projections/wave-040.md)
- [ ] S04-W041: JSON prompt loader — [wave-041.md](/docs/todo/waves/stage-04-schema-and-projections/wave-041.md)
- [ ] S04-W042: KV memory store — [wave-042.md](/docs/todo/waves/stage-04-schema-and-projections/wave-042.md)

### Stage S05: Security Baseline
**Objective:** Auth, sessions, CSRF enforcement.
**Wave Files:** [stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)
- [ ] S05-W050: first-run + login — [wave-050.md](/docs/todo/waves/stage-05-auth-and-security/wave-050.md)
- [ ] S05-W051: session cookies + CSRF — [wave-051.md](/docs/todo/waves/stage-05-auth-and-security/wave-051.md)
- [ ] S05-W052: rate limiting + CSP — [wave-052.md](/docs/todo/waves/stage-05-auth-and-security/wave-052.md)

### Stage S06: REST API Closure
**Objective:** Full HTTP contract with acceptance tests.
**Wave Files:** [stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)
- [ ] S06-W060: workspace/project APIs — [wave-060.md](/docs/todo/waves/stage-06-rest-api/wave-060.md)
- [ ] S06-W061: search/backlink APIs — [wave-061.md](/docs/todo/waves/stage-06-rest-api/wave-061.md)
- [ ] S06-W062: automation APIs — [wave-062.md](/docs/todo/waves/stage-06-rest-api/wave-062.md)

### Stage S07: WebSocket Sync Closure
**Objective:** Realtime sync with deterministic replay.
**Wave Files:** [stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)
- [ ] S07-W070: subscription management — [wave-070.md](/docs/todo/waves/stage-07-websocket-sync/wave-070.md)
- [ ] S07-W071: patch commit/reject — [wave-071.md](/docs/todo/waves/stage-07-websocket-sync/wave-071.md)
- [ ] S07-W072: broadcast registry — [wave-072.md](/docs/todo/waves/stage-07-websocket-sync/wave-072.md)

### Stage S08: Frontend + Static Hosting
**Objective:** Obsidian-like editor + responsive shell at root URL.
**Wave Files:** [stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)
- [ ] S08-W080: TypeScript app scaffold — [wave-080.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-080.md)
- [ ] S08-W081: Obsidian-like markdown editor — [wave-081.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-081.md)
- [ ] S08-W082: responsive shell (2/3 threshold) — [wave-082.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-082.md)

### Stage S09: CI + Performance
**Objective:** Verification profiles and latency targets.
**Wave Files:** [stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)
- [ ] S09-W090: CI profiles + file-size audit — [wave-090.md](/docs/todo/waves/stage-09-ci-performance-release/wave-090.md)
- [ ] S09-W091: DB-backed integration tests — [wave-091.md](/docs/todo/waves/stage-09-ci-performance-release/wave-091.md)
- [ ] S09-W092: performance benchmarks — [wave-092.md](/docs/todo/waves/stage-09-ci-performance-release/wave-092.md)

### Stage S10: Hardening
**Objective:** Close improvement backlog.
**Wave Files:** [stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)
- [ ] S10-W100: module splits (<200 lines) — [wave-100.md](/docs/todo/waves/stage-10-hardening-and-investigation/wave-100.md)
- [ ] S10-W101: property-based tests — [wave-101.md](/docs/todo/waves/stage-10-hardening-and-investigation/wave-101.md)
- [ ] S10-W102: Docker polish — [wave-102.md](/docs/todo/waves/stage-10-hardening-and-investigation/wave-102.md)

---

## Exit Criteria — ALL Must Be True

- [ ] No high-severity rows in [LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] No open M1/M2 in [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] [RELEASE.md](/docs/reference/RELEASE.md) gate is green
- [ ] All acceptance IDs pass — [testing.md](/docs/spec/technical/testing.md)
- [ ] All source files ≤200 lines — [STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [ ] `tmp/`, `log/`, `docs/logs/` do NOT exist — [ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
