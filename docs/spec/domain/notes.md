# Notes Domain

**Back:** [Domain Root](/docs/spec/domain/README.md)

---

## Canonical Content Model

### Core Identity

| Field | Type | Immutable | Description |
|-------|------|-----------|-------------|
| `note_id` | UUID | Yes | Stable unique identifier |
| `title` | String | No | Mutable display name |
| `markdown` | Text | No | Markdown body content |
| `workspace_id` | UUID | Yes | Owning workspace |
| `project_id` | UUID | No | Optional project scoping |
| `note_kind` | Enum | Yes | Note type (note, template, etc.) |
| `access_scope` | Enum | No | Visibility (private, workspace, public) |
| `version` | Integer | No | Optimistic concurrency counter |
| `created_at` | Timestamp | Yes | Creation time |
| `updated_at` | Timestamp | No | Last modification time |
| `deleted_at` | Timestamp | No | Soft-delete marker (null = active) |

### ID vs Title Separation — Critical Contract

**`note_id` (Immutable):**
- Generated once at creation (UUID v4)
- Never changes throughout note lifecycle
- Used for: API routing, backlinks, event sourcing, references
- Analogous to database primary key

**`title` (Mutable):**
- User-facing display name
- Can be edited freely
- Defaults to datetime if not provided
- Used for: Display, search, wiki-link resolution
- Analogous to a mutable label

**Contract:**
```
note_id: "550e8400-e29b-41d4-a716-446655440000"  // Never changes
title:   "Meeting Notes 2026-02-24"              // Can change anytime
```

---

## Default Title Rule

### Automatic Title Assignment

When creating a note **without** an explicit title:

1. **Server assigns:** Current timestamp as title
2. **Format:** `YYYY-MM-DD HH:mm:ss` (server local timezone)
3. **Timezone:** Server's configured timezone (default: UTC)
4. **Returned in:** Create response and all projection reads

### Example

**Request:**
```json
POST /notes
{
  "workspace_id": "ws-uuid",
  "markdown": "Content here..."
}
```

**Response:**
```json
{
  "note_id": "note-uuid",
  "title": "2026-02-24 14:30:00",
  "markdown": "Content here...",
  "version": 1,
  "created_at": "2026-02-24T14:30:00Z"
}
```

### User Experience

1. User clicks "Create New Note" without entering title
2. Note created with datetime title (e.g., "2026-02-24 14:30:00")
3. Title input immediately focused for quick rename
4. User can keep datetime or type custom title

---

## Wiki-Link Syntax

### Supported Formats

| Syntax | Resolution |
|--------|------------|
| `[[Note Title]]` | Fuzzy match by title |
| `[[note_id]]` | Exact match by ID |
| `[[note_id|Custom Alias]]` | ID with custom display text |
| `[[Note Title|Alias]]` | Title with custom display text |

### Parsing Rules

1. Parse `[[...]]` patterns in markdown body
2. Extract target (before `|`) and alias (after `|`, optional)
3. Resolve target:
   - First: Try exact title match
   - Second: Try fuzzy title match
   - Third: Try UUID parse + ID match
4. Store backlink: `(source_note_id, target_note_id)`

---

## Stream Lifecycle

### Note States

| State | Condition | Readable | Writable | Recoverable |
|-------|-----------|----------|----------|-------------|
| `Active` | `deleted_at IS NULL` | Yes (default list) | Yes | N/A |
| `SoftDeleted` | `deleted_at IS NOT NULL` | No (default), Yes (include-deleted mode) | No (except undelete) | Yes (undelete) |

### Deletion Rules

- Delete MUST be soft-delete at stream level by default
- Deleted notes MUST be excluded from:
  - Default note list
  - Search results
  - Backlink displays
- Include-deleted mode MAY expose tombstoned streams for recovery
- Hard delete (physical removal) is admin-only operation

---

## Write Rules — Optimistic Concurrency

### Version Contract

| Rule | Description |
|------|-------------|
| Every write includes `base_version` | Client sends last known version |
| Version check | Server compares `base_version` with `current_version` |
| Match | Accept write, increment version by 1 |
| Mismatch | Reject with `409 Conflict`, return current version |

### Write Flow

```
Client                          Server
  │                               │
  │  PATCH /notes/{id}            │
  │  { base_version: 5, ... }     │
  │  ─────────────────────────>   │
  │                               │
  │                               │  Check: current_version == 5?
  │                               │  YES → Apply, version = 6
  │                               │  NO  → Reject 409
  │                               │
  │  { version: 6, ... }          │
  │  <─────────────────────────   │
  │                               │
```

---

## Conflict Contract

### HTTP Conflict

**Request:**
```http
PATCH /notes/{id}
Content-Type: application/json

{
  "base_version": 5,
  "title": "Updated Title",
  "markdown": "Updated content"
}
```

**Success Response:**
```http
HTTP/1.1 200 OK
{
  "note_id": "...",
  "title": "Updated Title",
  "version": 6,
  "updated_at": "2026-02-24T14:35:00Z"
}
```

**Conflict Response:**
```http
HTTP/1.1 409 Conflict
{
  "error": "VERSION_CONFLICT",
  "expected_version": 5,
  "current_version": 7,
  "current_title": "Concurrent Update",
  "current_markdown": "...",
  "updated_at": "2026-02-24T14:34:00Z"
}
```

### WebSocket Conflict

**Client:**
```json
{
  "type": "apply_patch",
  "note_id": "...",
  "base_version": 5,
  "patch_ops": [...],
  "idempotency_key": "uuid"
}
```

**Server (Success):**
```json
{
  "type": "patch_committed",
  "note_id": "...",
  "version": 6,
  "event_seq": 42,
  "idempotency_key": "uuid"
}
```

**Server (Conflict):**
```json
{
  "type": "patch_rejected",
  "note_id": "...",
  "expected_version": 5,
  "current_version": 7,
  "reason": "VERSION_MISMATCH"
}
```

---

## Agent Mutation Rules

### kjxlkj-agent Writes

| Rule | Description |
|------|-------------|
| YOLO mode | Agent MAY create/edit notes directly |
| Version checks | Agent MUST obey optimistic concurrency |
| Permission scope | Agent MUST respect workspace boundaries |
| Cross-workspace | Cross-workspace writes MUST be rejected |
| Audit trail | Agent writes auditable as `actor_type=agent` |

### Agent Note Metadata

Agent-created notes MUST include:

```json
{
  "note_id": "...",
  "title": "...",
  "metadata": {
    "created_by_agent": "kjxlkj-agent",
    "agent_run_id": "run-uuid",
    "agent_operation": "create|update"
  }
}
```

---

## Event Sourcing

### Event Types

| Event | Trigger | Payload |
|-------|---------|---------|
| `note.created` | First creation | `title`, `markdown`, `workspace_id`, `project_id` |
| `note.updated` | Patch applied | `patch_ops`, `new_version` |
| `note.title_changed` | Title update | `old_title`, `new_title` |
| `note.deleted` | Soft delete | `deleted_at` |
| `note.undeleted` | Restore | `deleted_at` cleared |

### Event Schema

```json
{
  "event_id": "uuid",
  "note_id": "uuid",
  "event_type": "note.updated",
  "event_seq": 42,
  "version": 6,
  "actor_type": "user|agent",
  "actor_id": "uuid",
  "timestamp": "ISO8601",
  "payload": {
    "patch_ops": [...],
    "new_version": 6
  }
}
```

---

## Related

- [Events](events.md) — event sourcing rules
- [Permissions](permissions.md) — access control
- [Editor flow](/docs/spec/ui/editor-flow.md) — editing UX
- [HTTP API](/docs/spec/api/http.md) — note endpoints
- [WebSocket](/docs/spec/api/websocket.md) — realtime protocol
