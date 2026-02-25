# Improvement Backlog

**Generated:** 2026-02-25  
**Status:** Active

---

## High Priority

### H001: PostgreSQL Integration
**Description:** Replace in-memory repositories with PostgreSQL-backed implementation using sqlx.

**Acceptance Criteria:**
- [ ] sqlx pool configured from `data/config.json`
- [ ] Migrations applied on startup
- [ ] All CRUD operations use database
- [ ] Connection pooling with configured limits

**Estimated Effort:** 2-3 hours

---

### H002: Vector Search Implementation
**Description:** Implement semantic search with pgvector HNSW index.

**Acceptance Criteria:**
- [ ] pgvector extension enabled
- [ ] Embedding generation on note write
- [ ] HNSW index for ANN search
- [ ] ColBERT late interaction (optional)

**Estimated Effort:** 3-4 hours

---

### H003: LLM Provider Integration
**Description:** Connect kjxlkj-agent to LMStudio/Ollama/OpenRouter.

**Acceptance Criteria:**
- [ ] HTTP client for OpenAI-compatible API
- [ ] Timeout and retry handling
- [ ] Health check endpoint
- [ ] Fallback model support

**Estimated Effort:** 2 hours

---

### H004: Secure Session Management
**Description:** Implement proper cookie-based sessions with CSRF protection.

**Acceptance Criteria:**
- [ ] HttpOnly, Secure cookies
- [ ] CSRF token validation
- [ ] Session expiry handling
- [ ] Logout invalidates session

**Estimated Effort:** 2 hours

---

## Medium Priority

### M001: File Size Enforcement
**Description:** Ensure all .rs/.ts files ≤200 lines.

**Current Violations:**
- `src/crates/automation/kjxlkj-automation/src/agent.rs` - ~350 lines
- `src/crates/http/kjxlkj-http/src/handlers/note.rs` - ~250 lines
- `src/crates/db/kjxlkj-db/src/repo/note.rs` - ~200 lines

**Action:** Split into submodules

**Estimated Effort:** 1 hour

---

### M002: Property-Based Tests
**Description:** Add proptest tests for domain invariants.

**Test Cases:**
- Note ID immutability
- Title mutation doesn't affect ID
- Version always increments
- Soft-delete excludes from queries

**Estimated Effort:** 2 hours

---

### M003: WebSocket Reconnect Logic
**Description:** Implement full cursor replay on reconnect.

**Acceptance Criteria:**
- [ ] Exponential backoff (1s, 2s, 4s, 8s, 16s max)
- [ ] Ack with last event_seq
- [ ] Server replays missed events
- [ ] Queue offline patches

**Estimated Effort:** 2 hours

---

### M004: Editor Conflict Resolution UI
**Description:** Implement merge view for version conflicts.

**Acceptance Criteria:**
- [ ] 3-way merge display
- [ ] Keep yours / Use theirs buttons
- [ ] Manual merge editor
- [ ] Conflict markers in markdown

**Estimated Effort:** 3 hours

---

## Low Priority

### L001: Dashboard Module
**Description:** Workspace overview with statistics.

**Features:**
- Note count by kind
- Recent activity timeline
- Storage usage
- Search analytics

**Estimated Effort:** 4 hours

---

### L002: Export/Import
**Description:** Backup and restore workspace data.

**Formats:**
- Markdown export (per note)
- JSON export (full workspace)
- ZIP archive with attachments

**Estimated Effort:** 3 hours

---

### L003: Theme Support
**Description:** Light/dark mode toggle.

**Implementation:**
- CSS custom properties
- Persist preference in localStorage
- System preference detection

**Estimated Effort:** 1 hour

---

### L004: Keyboard Shortcut Customization
**Description:** Allow users to configure shortcuts.

**UI:**
- Settings modal
- Shortcut recorder
- Conflict detection

**Estimated Effort:** 2 hours

---

## Completed

- [x] S01: Runtime skeleton (10 crates)
- [x] S02: Notes + Search core
- [x] S03: HTTP + WebSocket protocol
- [x] S04: kjxlkj-agent loop
- [x] S05: Auth baseline
- [x] S06: REST API endpoints
- [x] S07: WebSocket sync
- [x] S08: Frontend application
- [x] S09: CI configuration
- [x] S10: Module splits

---

## Related

- [CONFORMANCE.md](/docs/reference/CONFORMANCE.md) - Verified behavior
- [LIMITATIONS.md](/docs/reference/LIMITATIONS.md) - Open gaps
- [DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) - Mismatch tracking
