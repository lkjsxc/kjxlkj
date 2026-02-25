# Reconstruction TODO — Master Execution Contract

**Authority:** This is the **ONLY** allowed implementation order. Following these TODOs with their linked docs MUST organically produce full spec compliance.

**Status:** All checkboxes reset for clean rebuild from docs-only baseline.

---

## Complete Documentation Index

Every file in `/docs` is linked below. Clicking links during implementation will organically produce full spec compliance.

### Policy Layer (Governance — 5 files)
- [docs/README.md](/docs/README.md) — Documentation index
- [docs/policy/README.md](/docs/policy/README.md) — Policy index
- [docs/policy/INSTRUCT.md](/docs/policy/INSTRUCT.md) — Operating contract
- [docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md) — Gate sequence
- [docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) — Directory/file constraints (≤12 items, ≤200 lines)
- [docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) — Allowed root paths

### Overview Layer (Orientation — 4 files)
- [docs/overview/README.md](/docs/overview/README.md) — Orientation index
- [docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) — Docs-first doctrine
- [docs/overview/principles.md](/docs/overview/principles.md) — Design principles
- [docs/overview/glossary.md](/docs/overview/glossary.md) — Shared terms

### Specification Layer (Target Behavior — 50 files)

#### Spec Root
- [docs/spec/README.md](/docs/spec/README.md) — Specification index

#### Architecture (9 files)
- [docs/spec/architecture/README.md](/docs/spec/architecture/README.md) — Architecture index
- [docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md) — Config loading (`data/config.json`)
- [docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) — Startup sequence
- [docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md) — Rust decomposition (10 crates)
- [docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) — Module splits
- [docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) — Cargo policy
- [docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) — Process hosting, Docker
- [docs/spec/architecture/completion-file-map.md](/docs/spec/architecture/completion-file-map.md) — Path requirements
- [docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) — Tree states (State A/B)

#### API (7 files)
- [docs/spec/api/README.md](/docs/spec/api/README.md) — API index
- [docs/spec/api/http.md](/docs/spec/api/http.md) — REST endpoints
- [docs/spec/api/websocket.md](/docs/spec/api/websocket.md) — Realtime protocol
- [docs/spec/api/types.md](/docs/spec/api/types.md) — Payload schemas
- [docs/spec/api/errors.md](/docs/spec/api/errors.md) — Error model
- [docs/spec/api/openapi.md](/docs/spec/api/openapi.md) — OpenAPI governance
- [docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) — Agent XML protocol

#### Domain (12 files)
- [docs/spec/domain/README.md](/docs/spec/domain/README.md) — Domain index
- [docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md) — Workspace lifecycle
- [docs/spec/domain/projects.md](/docs/spec/domain/projects.md) — Project scoping
- [docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) — RBAC model
- [docs/spec/domain/notes.md](/docs/spec/domain/notes.md) — Note lifecycle (ID + title separation)
- [docs/spec/domain/note-types.md](/docs/spec/domain/note-types.md) — Kind taxonomy
- [docs/spec/domain/events.md](/docs/spec/domain/events.md) — Event sourcing
- [docs/spec/domain/metadata.md](/docs/spec/domain/metadata.md) — Typed metadata
- [docs/spec/domain/automation.md](/docs/spec/domain/automation.md) — kjxlkj-agent runs
- [docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) — File handling
- [docs/spec/domain/search.md](/docs/spec/domain/search.md) — Hybrid search (REDESIGNED, 4-stage pipeline)
- [docs/spec/domain/export.md](/docs/spec/domain/export.md) — Backup/export

#### Security (5 files)
- [docs/spec/security/README.md](/docs/spec/security/README.md) — Security index
- [docs/spec/security/auth.md](/docs/spec/security/auth.md) — First-run + login
- [docs/spec/security/sessions.md](/docs/spec/security/sessions.md) — Cookie lifecycle
- [docs/spec/security/csrf.md](/docs/spec/security/csrf.md) — CSRF enforcement
- [docs/spec/security/transport.md](/docs/spec/security/transport.md) — TLS/proxy

#### Technical (9 files)
- [docs/spec/technical/README.md](/docs/spec/technical/README.md) — Technical index
- [docs/spec/technical/testing.md](/docs/spec/technical/testing.md) — Verification tiers + acceptance IDs
- [docs/spec/technical/performance.md](/docs/spec/technical/performance.md) — Latency targets
- [docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) — Schema discipline
- [docs/spec/technical/operations.md](/docs/spec/technical/operations.md) — Health/backup/observability
- [docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) — kjxlkj-agent loop
- [docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) — Prompt schema
- [docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) — Rust/TS gates
- [docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) — Prompt packs

#### UI (8 files)
- [docs/spec/ui/README.md](/docs/spec/ui/README.md) — UI index
- [docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) — Shell + auth + note creation (ROOT URL CONTRACT)
- [docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) — Obsidian-like markdown editor
- [docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) — Responsive (2/3 threshold at 1280px)
- [docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) — Finding mapping
- [docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) — UX matrix
- [docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) — Optional modules

### Reference Layer (Verified State — 8 files)
- [docs/reference/README.md](/docs/reference/README.md) — State index
- [docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Verified behavior
- [docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — Open gaps
- [docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) — Mismatch tracking
- [docs/reference/CI.md](/docs/reference/CI.md) — Verification profiles
- [docs/reference/RELEASE.md](/docs/reference/RELEASE.md) — Release gate
- [docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) — Proof artifacts
- [docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md) — Backlog

### Guides Layer (Operator Playbooks — 6 files)
- [docs/guides/README.md](/docs/guides/README.md) — Guides index
- [docs/guides/QUICKSTART.md](/docs/guides/QUICKSTART.md) — First-run workflow
- [docs/guides/DOCKER.md](/docs/guides/DOCKER.md) — Local orchestration
- [docs/guides/API.md](/docs/guides/API.md) — API usage
- [docs/guides/LIBRARIAN.md](/docs/guides/LIBRARIAN.md) — Agent operation
- [docs/guides/RECONSTRUCTION_BOOTSTRAP.md](/docs/guides/RECONSTRUCTION_BOOTSTRAP.md) — Scaffold guide

### TODO Waves (Execution Plan — 44 files)
- [docs/todo/README.md](/docs/todo/README.md) — TODO index
- [docs/todo/waves/README.md](/docs/todo/waves/README.md) — Waves index

---

## Execution Rules

1. **Execute stages ONLY in listed order** — per [WORKFLOW.md](/docs/policy/WORKFLOW.md)
2. **Before closing each wave:** run `cargo build --workspace` then `cargo test --workspace` — per [CI.md](/docs/reference/CI.md)
3. **Verify acceptance IDs** — per [testing.md](/docs/spec/technical/testing.md)
4. **Sync ledgers AFTER each stage** — update [CONFORMANCE.md](/docs/reference/CONFORMANCE.md), [LIMITATIONS.md](/docs/reference/LIMITATIONS.md), [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
5. **TODO link policy** — every checkbox MUST link to governing doc — per [STRUCTURE.md](/docs/policy/STRUCTURE.md)
6. **File size enforcement** — all .rs/.ts files ≤200 lines — per [STRUCTURE.md](/docs/policy/STRUCTURE.md)
7. **Root URL accessibility** — app MUST be fully functional at `http://localhost:8080/` — per [web-app.md](/docs/spec/ui/web-app.md)

---

## Ordered Stages — Complete Checklist

### Stage S00: Governance Baseline
**Objective:** Establish policy foundation and docs integrity gates.

**Governing Docs:**
- [policy/README.md](/docs/policy/README.md) — Policy index
- [STRUCTURE.md](/docs/policy/STRUCTURE.md) — Directory/file constraints
- [ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) — Allowed root paths

**Wave Files:** [stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md)

- [ ] S00-W000: policy tree scaffold — [wave-000.md](/docs/todo/waves/stage-00-pivot-governance/wave-000.md)
- [ ] S00-W001: docs link integrity — [wave-001.md](/docs/todo/waves/stage-00-pivot-governance/wave-001.md)
- [ ] S00-W002: structure enforcement — [wave-002.md](/docs/todo/waves/stage-00-pivot-governance/wave-002.md)

---

### Stage S01: Runtime Skeleton
**Objective:** Rebuild Cargo workspace and crate topology from spec.

**Governing Docs:**
- [architecture/README.md](/docs/spec/architecture/README.md) — Architecture index
- [crates.md](/docs/spec/architecture/crates.md) — Rust decomposition (10 crates)
- [workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) — Cargo policy
- [final-file-structure.md](/docs/spec/architecture/final-file-structure.md) — Tree states

**Wave Files:** [stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

- [ ] S01-W010: Cargo.toml + crate dirs — [wave-010.md](/docs/todo/waves/stage-01-spec-rebuild/wave-010.md)
- [ ] S01-W011: domain types + events — [wave-011.md](/docs/todo/waves/stage-01-spec-rebuild/wave-011.md)
- [ ] S01-W012: in-memory repos — [wave-012.md](/docs/todo/waves/stage-01-spec-rebuild/wave-012.md)

---

### Stage S02: Notes + Search Core
**Objective:** Implement note lifecycle and redesigned hybrid search.

**Governing Docs:**
- [notes.md](/docs/spec/domain/notes.md) — Note lifecycle (ID + title separation)
- [search.md](/docs/spec/domain/search.md) — Hybrid search (4-stage pipeline)
- [events.md](/docs/spec/domain/events.md) — Event sourcing
- [note-types.md](/docs/spec/domain/note-types.md) — Kind taxonomy

**Wave Files:** [stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

- [ ] S02-W020: note CRUD + ID/title separation — [wave-020.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-020.md)
- [ ] S02-W021: event sourcing + projections — [wave-021.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-021.md)
- [ ] S02-W022: redesigned search (vector + lexical) — [wave-022.md](/docs/todo/waves/stage-02-workspace-bootstrap/wave-022.md)

---

### Stage S03: Realtime Integration
**Objective:** HTTP + WebSocket protocol with replay semantics.

**Governing Docs:**
- [http.md](/docs/spec/api/http.md) — REST endpoints
- [websocket.md](/docs/spec/api/websocket.md) — Realtime protocol
- [types.md](/docs/spec/api/types.md) — Payload schemas
- [errors.md](/docs/spec/api/errors.md) — Error model

**Wave Files:** [stage-03-runtime-integration/README.md](/docs/todo/waves/stage-03-runtime-integration/README.md)

- [ ] S03-W030: HTTP routes + handlers — [wave-030.md](/docs/todo/waves/stage-03-runtime-integration/wave-030.md)
- [ ] S03-W031: WebSocket cursor replay — [wave-031.md](/docs/todo/waves/stage-03-runtime-integration/wave-031.md)
- [ ] S03-W032: idempotency + conflict UX — [wave-032.md](/docs/todo/waves/stage-03-runtime-integration/wave-032.md)

---

### Stage S04: Automation + Agent
**Objective:** kjxlkj-agent loop with JSON prompts and KV memory.

**Governing Docs:**
- [librarian-agent.md](/docs/spec/technical/librarian-agent.md) — kjxlkj-agent loop
- [agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) — Prompt schema
- [automation.md](/docs/spec/domain/automation.md) — kjxlkj-agent runs
- [librarian-xml.md](/docs/spec/api/librarian-xml.md) — Agent XML protocol

**Wave Files:** [stage-04-schema-and-projections/README.md](/docs/todo/waves/stage-04-schema-and-projections/README.md)

- [ ] S04-W040: agent loop scaffold — [wave-040.md](/docs/todo/waves/stage-04-schema-and-projections/wave-040.md)
- [ ] S04-W041: JSON prompt loader — [wave-041.md](/docs/todo/waves/stage-04-schema-and-projections/wave-041.md)
- [ ] S04-W042: KV memory store — [wave-042.md](/docs/todo/waves/stage-04-schema-and-projections/wave-042.md)

---

### Stage S05: Security Baseline
**Objective:** Auth, sessions, CSRF enforcement.

**Governing Docs:**
- [auth.md](/docs/spec/security/auth.md) — First-run + login
- [sessions.md](/docs/spec/security/sessions.md) — Cookie lifecycle
- [csrf.md](/docs/spec/security/csrf.md) — CSRF enforcement
- [transport.md](/docs/spec/security/transport.md) — TLS/proxy

**Wave Files:** [stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)

- [ ] S05-W050: first-run + login — [wave-050.md](/docs/todo/waves/stage-05-auth-and-security/wave-050.md)
- [ ] S05-W051: session cookies + CSRF — [wave-051.md](/docs/todo/waves/stage-05-auth-and-security/wave-051.md)
- [ ] S05-W052: rate limiting + CSP — [wave-052.md](/docs/todo/waves/stage-05-auth-and-security/wave-052.md)

---

### Stage S06: REST API Closure
**Objective:** Full HTTP contract with acceptance tests.

**Governing Docs:**
- [http.md](/docs/spec/api/http.md) — REST endpoints
- [errors.md](/docs/spec/api/errors.md) — Error model
- [testing.md](/docs/spec/technical/testing.md) — Verification tiers + acceptance IDs

**Wave Files:** [stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

- [ ] S06-W060: workspace/project APIs — [wave-060.md](/docs/todo/waves/stage-06-rest-api/wave-060.md)
- [ ] S06-W061: search/backlink APIs — [wave-061.md](/docs/todo/waves/stage-06-rest-api/wave-061.md)
- [ ] S06-W062: automation APIs — [wave-062.md](/docs/todo/waves/stage-06-rest-api/wave-062.md)

---

### Stage S07: WebSocket Sync Closure
**Objective:** Realtime sync with deterministic replay.

**Governing Docs:**
- [websocket.md](/docs/spec/api/websocket.md) — Realtime protocol
- [events.md](/docs/spec/domain/events.md) — Event sourcing
- [editor-flow.md](/docs/spec/ui/editor-flow.md) — Obsidian-like markdown editor

**Wave Files:** [stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

- [ ] S07-W070: subscription management — [wave-070.md](/docs/todo/waves/stage-07-websocket-sync/wave-070.md)
- [ ] S07-W071: patch commit/reject — [wave-071.md](/docs/todo/waves/stage-07-websocket-sync/wave-071.md)
- [ ] S07-W072: broadcast registry — [wave-072.md](/docs/todo/waves/stage-07-websocket-sync/wave-072.md)

---

### Stage S08: Frontend + Static Hosting
**Objective:** Obsidian-like editor + responsive shell at root URL.

**Governing Docs:**
- [editor-flow.md](/docs/spec/ui/editor-flow.md) — Obsidian-like markdown editor
- [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) — Responsive (2/3 threshold at 1280px)
- [web-app.md](/docs/spec/ui/web-app.md) — Shell + auth + note creation (ROOT URL CONTRACT)
- [workspace-suite.md](/docs/spec/ui/workspace-suite.md) — Optional modules

**Wave Files:** [stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

- [ ] S08-W080: TypeScript app scaffold — [wave-080.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-080.md)
- [ ] S08-W081: Obsidian-like markdown editor — [wave-081.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-081.md)
- [ ] S08-W082: responsive shell (2/3 threshold) — [wave-082.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/wave-082.md)

---

### Stage S09: CI + Performance
**Objective:** Verification profiles and latency targets.

**Governing Docs:**
- [CI.md](/docs/reference/CI.md) — Verification profiles
- [performance.md](/docs/spec/technical/performance.md) — Latency targets
- [testing.md](/docs/spec/technical/testing.md) — Verification tiers + acceptance IDs

**Wave Files:** [stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

- [ ] S09-W090: CI profiles + file-size audit — [wave-090.md](/docs/todo/waves/stage-09-ci-performance-release/wave-090.md)
- [ ] S09-W091: DB-backed integration tests — [wave-091.md](/docs/todo/waves/stage-09-ci-performance-release/wave-091.md)
- [ ] S09-W092: performance benchmarks — [wave-092.md](/docs/todo/waves/stage-09-ci-performance-release/wave-092.md)

---

### Stage S10: Hardening
**Objective:** Close improvement backlog.

**Governing Docs:**
- [IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md) — Backlog
- [STRUCTURE.md](/docs/policy/STRUCTURE.md) — Directory/file constraints
- [RELEASE.md](/docs/reference/RELEASE.md) — Release gate

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
- [ ] All 120 documentation files linked and accessible — [docs/README.md](/docs/README.md)
- [ ] Root URL serves complete app — [web-app.md](/docs/spec/ui/web-app.md)
- [ ] Final file structure matches State B — [final-file-structure.md](/docs/spec/architecture/final-file-structure.md)

---

## File Count Summary

| Layer | File Count |
|-------|------------|
| Policy | 5 |
| Overview | 4 |
| Specification | 50 |
| Reference | 8 |
| Guides | 6 |
| TODO Waves | 46 |
| docs/README.md | 1 |
| **Total** | **120** |

---

## Quick Reference

### Key Specifications

| Spec | File | Description |
|------|------|-------------|
| Search | [search.md](/docs/spec/domain/search.md) | 4-stage pipeline: Query → Retrieval → Fusion → Re-rank |
| Editor | [editor-flow.md](/docs/spec/ui/editor-flow.md) | Obsidian-like markdown with wiki-links |
| Agent | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) | KV memory, YOLO mode, JSON prompts |
| Layout | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | 2/3 threshold (1280px breakpoint) |
| Notes | [notes.md](/docs/spec/domain/notes.md) | ID + title separation, datetime default |
| Root URL | [web-app.md](/docs/spec/ui/web-app.md) | Full app at `http://localhost:8080/` |
| File Structure | [final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | State A/B tree definitions |

### Key Acceptance IDs

| ID | Scenario | Governing Spec |
|----|----------|----------------|
| `API-NOTE-01` | Create note without title → datetime | [notes.md](/docs/spec/domain/notes.md) |
| `API-NOTE-02` | Note ID stable while title changes | [notes.md](/docs/spec/domain/notes.md) |
| `API-SEARCH-01` | Lexical search deterministic | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-02` | Hybrid mode (lexical+semantic) | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-03` | Embedding outage degrades to lexical | [search.md](/docs/spec/domain/search.md) |
| `WS-04` | Idempotency key deduplication | [websocket.md](/docs/spec/api/websocket.md) |
| `WS-05` | Reconnect + cursor replay | [websocket.md](/docs/spec/api/websocket.md) |
| `E2E-06` | Editor autosave (600ms) | [editor-flow.md](/docs/spec/ui/editor-flow.md) |
| `E2E-12` | Compact menu at ≤1280px | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-19` | 320px layout usable | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-23` | Create + select note immediately | [web-app.md](/docs/spec/ui/web-app.md) |
| `E2E-24` | Markdown editor workflows | [editor-flow.md](/docs/spec/ui/editor-flow.md) |
| `E2E-25` | Compact mode closes on select | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `AGENT-01` | Prompt loading from JSON | [agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) |
| `AGENT-02` | KV memory persistence | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-03` | YOLO mode mutations | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |

---

## Related

- [WORKFLOW.md](/docs/policy/WORKFLOW.md) — Gate sequence
- [STRUCTURE.md](/docs/policy/STRUCTURE.md) — Directory/file constraints
- [CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Verified behavior
- [IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md) — Backlog
