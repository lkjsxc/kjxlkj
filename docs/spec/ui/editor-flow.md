# Note Editor — Obsidian-Like Markdown Editor

**Back:** [UI Root](/docs/spec/ui/README.md)

---

## Editor Model — Bold Implementation

The editor MUST be a **full-featured Obsidian-like markdown workspace** with:

### Core Principles
1. **Plain markdown source** as first-class editing surface
2. **Live preview** without leaving edit context (split-pane or toggle)
3. **Wiki-link authoring** (`[[note]]`) with autocomplete and backlink awareness
4. **Keyboard-centric** command palette (Cmd/Ctrl+P)
5. **Zero-config** — works immediately, extensible via settings

### Required Markdown Features

| Feature | Syntax | Behavior |
|---------|--------|----------|
| Headings | `# H1` to `###### H6` | Foldable, outline navigation |
| Bold | `**text**` or `__text__` | Live preview highlight |
| Italic | `*text*` or `_text_` | Live preview highlight |
| Strikethrough | `~~text~~` | Visual strike-through |
| Code inline | `` `code` `` | Monospace styling |
| Code blocks | ` ```lang ` | Syntax highlighting |
| Blockquotes | `> quote` | Indented block styling |
| Lists (ordered) | `1. item` | Auto-numbering |
| Lists (unordered) | `- item` or `* item` | Bullet styling |
| Task lists | `- [ ] task` | Checkbox interaction |
| Links | `[text](url)` | Clickable, external open |
| Wiki-links | `[[note title]]` | Internal navigation, autocomplete |
| Wiki-links (alias) | `[[note_id|alias]]` | Resolve by ID with custom text |
| Images | `![alt](url)` | Inline render with lazy load |
| Tables | `| col |` | GFM table rendering |
| Horizontal rule | `---` or `***` | Visual separator |

---

## State Model — Synced Snapshot + Local Draft

### Dual-Buffer Architecture

```
┌─────────────────────────────────────────────────────┐
│  Local Draft Buffer (client-side, mutable)          │
│  - Cursor position                                   │
│  - Undo/redo stack                                   │
│  - Pending changes (not yet saved)                   │
│  - Conflict markers (if any)                         │
└─────────────────────────────────────────────────────┘
                          ↓ autosave debounce
┌─────────────────────────────────────────────────────┐
│  Synced Snapshot (server-acknowledged, immutable)   │
│  - Last committed version                            │
│  - Acknowledged cursor (event_seq)                   │
│  - Base version for next patch                       │
└─────────────────────────────────────────────────────┘
```

### State Transitions

| State | Trigger | Exit Condition |
|-------|---------|----------------|
| `idle` | Initial load | User starts typing |
| `editing` | Keypress | Debounce timeout |
| `saving` | Autosave trigger | Server ACK |
| `saved` | Server ACK | 2s display timeout |
| `conflict` | 409 or `patch_rejected` | User resolves |
| `offline` | Network error | Reconnect success |
| `reconnecting` | WS disconnect | WS reconnect + replay |

---

## Editing Rules

### Autosave Contract

- **Debounce:** 600ms after last keystroke (configurable)
- **Optimistic:** Update UI immediately, reconcile on server ACK
- **Retry:** 3 attempts with exponential backoff (100ms, 500ms, 2s)
- **Conflict detection:** Compare `base_version` with server response

### Patch Generation

1. Capture local draft state
2. Compute diff against synced snapshot (using `diff-match-patch` or similar)
3. Generate patch operations (retain/insert/delete)
4. Include `idempotency_key` (UUID v4) for deduplication
5. Send via WS `apply_patch` (primary) or HTTP `PATCH` (fallback)

### Write Paths

| Path | Protocol | Use Case |
|------|----------|----------|
| Primary | WebSocket `apply_patch` | Real-time connected |
| Fallback | HTTP `PATCH /notes/{id}` | WS unavailable |
| Manual | Cmd/Ctrl+S | Explicit save (bypass debounce) |

---

## Obsidian-Like Behaviors

### Wiki-Link Authoring

1. User types `[[`
2. Show autocomplete dropdown with:
   - Matching note titles (fuzzy search)
   - Recently viewed notes
   - Option to create new note
3. On select: insert `[[note title]]` or `[[note_id|alias]]`
4. Visual styling: distinct color, clickable

### Backlink Panel

- Show in right sidebar (desktop) or bottom sheet (mobile)
- List all notes linking to current note
- Show snippet with link context
- Click to navigate

### Command Palette (Cmd/Ctrl+P)

| Command | Shortcut | Action |
|---------|----------|--------|
| Create note | Cmd/Ctrl+N | New note, focus title |
| Quick switch | Cmd/Ctrl+O | Note search + navigate |
| Toggle preview | Cmd/Ctrl+Shift+P | Split-pane toggle |
| Insert wiki-link | Cmd/Ctrl+K | Wiki-link autocomplete |
| Search in note | Cmd/Ctrl+F | In-editor search |
| Export markdown | Cmd/Ctrl+E | Download .md file |

### Markdown Shortcuts

| Trigger | Result |
|---------|--------|
| `# ` at line start | H1 heading |
| `## ` at line start | H2 heading |
| `- ` at line start | Bullet list item |
| `1. ` at line start | Numbered list item |
| `- [ ] ` at line start | Task checkbox |
| `> ` at line start | Blockquote |
| ` ``` ` on empty line | Code fence |
| `---` on empty line | Horizontal rule |

---

## Replay and Idempotency Rules

### Idempotency Contract

- Each patch MUST include unique `idempotency_key` (UUID v4)
- Duplicate `idempotency_key` MUST replay existing commit identity
- Server MUST return same response for duplicate keys
- Client MUST NOT retry without new key

### Reconnect Contract

1. Detect WS disconnect
2. Enter `reconnecting` state with exponential backoff
3. On reconnect: send `ack` with last `event_seq`
4. Server replays missed events from cursor
5. Apply replayed events to local draft
6. Resume normal operation

### Cursor Semantics

- Client maintains `acknowledged_cursor` (last confirmed event_seq)
- Patches include `base_version` derived from cursor
- Stale cursor (base_version < current) returns `patch_rejected`
- Rejected patch triggers full note refresh + conflict resolution

---

## Conflict UX

### Conflict Detection

| Condition | Detection |
|-----------|-----------|
| HTTP 409 | Response code + `Conflict` body |
| WS `patch_rejected` | `reason: "VERSION_MISMATCH"` |
| Optimistic concurrency | `expected_version != current_version` |

### Conflict Resolution UI

```
┌─────────────────────────────────────────────────────────┐
│  ⚠️ Conflict Detected                                    │
│                                                         │
│  Another device modified this note. Choose:             │
│                                                         │
│  [ Keep Your Changes ]  [ Use Server Version ]          │
│                                                         │
│  Or resolve manually:                                   │
│  ┌───────────────────────────────────────────────────┐ │
│  │ <<<<<<< Yours                                     │ │
│  │ Your local content                                │ │
│  │ =======                                           │ │
│  │ Server content                                    │ │
│  │ >>>>>>> Server                                    │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  [ Reload from Server ]                                 │
└─────────────────────────────────────────────────────────┘
```

### Conflict Prevention

- Retry limit: 2 attempts before showing conflict UI
- Debounce: 600ms to batch rapid keystrokes
- Optimistic UI: Show changes immediately, reconcile later

---

## Title Edit Propagation

### Same-Cycle Update

When title changes:

1. Send `PATCH /notes/{id}/title` with `base_version`
2. On success: update note list/navigation IMMEDIATELY
3. Update browser tab title
4. Update backlink displays (if any note links to this)
5. Update search index (async, non-blocking)

### Visual Feedback

- Title input shows `saving...` → `saved` → idle
- Note list updates in same render cycle
- No flicker or stale display

---

## Chrome Minimization

### Default State (Low-Noise)

**Visible:**
- Title input (always)
- Editor content (always)
- Sync status indicator (subtle, corner)
- Menu toggle (per layout contract)

**Hidden by Default:**
- Save button (autosave handles it)
- Version history (accessible via Cmd/Ctrl+Shift+H)
- Delete (accessible via Cmd/Ctrl+Shift+D)
- Share/export (accessible via Cmd/Ctrl+E)

### Optional Modules

- Backlink panel (toggle, right sidebar)
- Outline/navigation (toggle, left sidebar)
- Metadata editor (toggle, below title)
- Command palette (on shortcut)

---

## Related

- [Web app shell](web-app.md) — note creation + auth flows
- [Layout contract](layout-and-interaction.md) — responsive behavior
- [Notes domain](notes.md) — ID/title separation, versioning
- [Findings traceability](findings-traceability.md) — requirement mapping
