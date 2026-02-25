# Note Editor — Obsidian-Like Markdown Workspace

**Back:** [UI Root](/docs/spec/ui/README.md)

---

## Editor Model — Bold Implementation

The editor MUST be a **full-featured Obsidian-like markdown workspace** that prioritizes:

1. **Plain markdown source** as the first-class editing surface
2. **Live preview** without leaving edit context (split-pane or toggle)
3. **Wiki-link authoring** (`[[note]]`) with autocomplete and backlink awareness
4. **Keyboard-centric** command palette (Cmd/Ctrl+P)
5. **Zero-config** — works immediately, extensible via settings

### Design Philosophy

```
┌─────────────────────────────────────────────────────────────┐
│                    Obsidian-Like Principles                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Markdown is the format — store plain .md files          │
│  2. Links are first-class — [[wiki]] syntax everywhere      │
│  3. Graph is implicit — backlinks auto-generated            │
│  4. Keyboard is king — mouse is optional                    │
│  5. Local-first — works offline, syncs when connected       │
│  6. Extensible — plugins, themes, custom snippets           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Required Markdown Features

### Core Syntax Support

| Feature | Syntax | Live Preview Behavior |
|---------|--------|----------------------|
| **Headings** | `# H1` to `###### H6` | Foldable sections, outline navigation |
| **Bold** | `**text**` or `__text__` | Font-weight: 600 highlight |
| **Italic** | `*text*` or `_text_` | Font-style: italic highlight |
| **Strikethrough** | `~~text~~` | Text-decoration: line-through |
| **Inline Code** | `` `code` `` | Monospace, background highlight |
| **Code Blocks** | ` ```lang ` | Syntax highlighting (Prism/Highlight.js) |
| **Blockquotes** | `> quote` | Left border, muted text |
| **Unordered Lists** | `- item` or `* item` | Bullet styling, nested indentation |
| **Ordered Lists** | `1. item` | Auto-numbering, nested indentation |
| **Task Lists** | `- [ ] task` / `- [x] done` | Interactive checkboxes |
| **Links** | `[text](url)` | Clickable, external open (new tab) |
| **Wiki-Links** | `[[note title]]` | Internal nav, autocomplete on `[[` |
| **Wiki-Links (ID)** | `[[note_id]]` | Resolve by UUID |
| **Wiki-Links (Alias)** | `[[note_id\|alias]]` | Custom display text |
| **Images** | `![alt](url)` | Inline render, lazy load, click to expand |
| **Tables** | `\| col \|` | GFM table rendering, sortable |
| **Horizontal Rule** | `---` or `***` | Visual separator |
| **Footnotes** | `[^1]` and `[^1]: def` | Popup footnote on hover |
| **Math (inline)** | `$x^2$` | KaTeX rendering |
| **Math (block)** | `$$...$$` | Display math, centered |

### Advanced Markdown Features

| Feature | Syntax | Behavior |
|---------|--------|----------|
| **Callouts** | `> [!NOTE]` | Styled alert boxes (info, warning, error) |
| **Highlight** | `==text==` | Background highlight (mark tag) |
| **Embeds** | `![[note]]` | Embed note content inline |
| **Transclusions** | `![[note#section]]` | Embed specific section |
| **Tag References** | `#tag` | Clickable tag search |
| **Cursors** | `%% comment %%` | Hidden comments (editor-only) |

---

## State Model — Synced Snapshot + Local Draft

### Dual-Buffer Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Client-Side State Model                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Local Draft Buffer (Mutable)              │ │
│  │  - Raw markdown text (current editor content)          │ │
│  │  - Cursor position (line, column)                      │ │
│  │  - Selection range (start, end)                        │ │
│  │  - Undo/redo stack (max 100 entries)                   │ │
│  │  - Pending changes (not yet acknowledged)              │ │
│  │  - Conflict markers (if resolution in progress)        │ │
│  │  - Scroll position (for restore after refresh)         │ │
│  └────────────────────────────────────────────────────────┘ │
│                          │                                   │
│                          │ autosave debounce (600ms)         │
│                          ▼                                   │
│  ┌────────────────────────────────────────────────────────┐ │
│  │            Synced Snapshot (Server-Acknowledged)       │ │
│  │  - Last committed version number                       │ │
│  │  - Acknowledged cursor (event_seq)                     │ │
│  │  - Base version for next patch                         │ │
│  │  - Last saved markdown (for diff computation)          │ │
│  │  - Last saved timestamp                                │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### State Machine

```
                    ┌──────────────┐
                    │    IDLE      │
                    └──────┬───────┘
                           │ user types
                           ▼
                    ┌──────────────┐
         ┌─────────│   EDITING    │──────────┐
         │         └──────┬───────┘          │
         │                │                   │
         │ debounce       │ debounce          │
         │ (600ms)        │ (600ms)           │
         │                │                   │
         ▼                ▼                   │
┌──────────────┐  ┌──────────────┐           │
│   OFFLINE    │  │    SAVING    │───────────┤
└──────┬───────┘  └──────┬───────┘           │
       │                 │ server ACK         │
       │ reconnect       │                    │
       ▼                 ▼                    │
┌──────────────┐  ┌──────────────┐           │
│ RECONNECTING │  │    SAVED     │───────────┘
└──────┬───────┘  └──────┬───────┘
       │                 │ 2s timeout
       │ replay done     ▼
       └─────────►┌──────────────┐
                  │    IDLE      │
                  └──────────────┘
```

### State Transitions

| From | Trigger | To | Side Effects |
|------|---------|----|--------------|
| `idle` | Keypress | `editing` | Start debounce timer |
| `editing` | Debounce timeout | `saving` | Generate patch, send WS |
| `saving` | Server ACK | `saved` | Update synced snapshot, show indicator |
| `saved` | 2s timeout | `idle` | Hide indicator |
| `saving` | Network error | `offline` | Queue patch, show offline indicator |
| `offline` | Reconnect success | `reconnecting` | Send ack with cursor |
| `reconnecting` | Replay complete | `idle` | Apply queued patches |
| `editing` | 409 Conflict | `conflict` | Show conflict resolution UI |

---

## Editing Rules

### Autosave Contract

**Debounce Timing:**
- Default: 600ms after last keystroke
- Configurable range: 200ms - 2000ms
- Manual save (Cmd/Ctrl+S): bypass debounce

**Optimistic Update:**
```
1. User types → update local draft immediately
2. Update UI (no waiting for server)
3. Start debounce timer
4. On timeout → generate patch, send to server
5. On ACK → update synced snapshot
6. On conflict → show resolution UI
```

**Retry Policy:**
```
Attempt 1: immediate retry (100ms delay)
Attempt 2: exponential backoff (500ms delay)
Attempt 3: exponential backoff (2000ms delay)
Attempt 4+: mark offline, queue for reconnect
```

### Patch Generation

**Algorithm:**
```rust
fn generate_patch(local_draft: &str, synced_snapshot: &str) -> Vec<PatchOp> {
    // Use diff-match-patch or similar library
    let diffs = dmp.diff_main(synced_snapshot, local_draft);
    let patches = dmp.patch_make(diffs);
    
    // Convert to our protocol format
    patches.into_iter().map(|p| {
        PatchOp {
            start: p.start1,
            length: p.length1,
            text: p.text,
        }
    }).collect()
}
```

**Patch Operations:**
```json
{
  "type": "apply_patch",
  "note_id": "uuid",
  "base_version": 5,
  "patch_ops": [
    {"op": "retain", "count": 50},
    {"op": "insert", "text": "new content"},
    {"op": "delete", "count": 10},
    {"op": "retain", "count": 100}
  ],
  "idempotency_key": "uuid",
  "client_ts": "2026-02-24T14:30:00Z"
}
```

### Write Paths

| Path | Protocol | Use Case | Fallback |
|------|----------|----------|----------|
| Primary | WebSocket `apply_patch` | Real-time connected | HTTP PATCH |
| Fallback | HTTP `PATCH /notes/{id}` | WS unavailable | Queue |
| Manual | Cmd/Ctrl+S | Explicit save | Same as primary |
| Offline | LocalStorage queue | Network unavailable | Flush on reconnect |

---

## Obsidian-Like Behaviors

### Wiki-Link Authoring

**Trigger:** User types `[[`

**Autocomplete Dropdown:**
```
┌─────────────────────────────────────────┐
│ [[meeting                               │
├─────────────────────────────────────────┤
│ 📄 Meeting Notes 2026-02-24             │
│ 📄 Meeting Notes 2026-02-17             │
│ 📄 Weekly Team Meeting                  │
│ ─────────────────────────────────────── │
│ ➕ Create "meeting notes"               │
│ 🔍 Search for "meeting"                 │
└─────────────────────────────────────────┘
```

**Behavior:**
1. Show matching note titles (fuzzy search)
2. Show recently viewed notes (context-aware)
3. Show option to create new note
4. Show option to search for more
5. On select: insert `[[note title]]` or `[[note_id|alias]]`
6. Visual styling: distinct color (theme-able), clickable

**Keyboard Navigation:**
- `Arrow Up/Down`: Navigate suggestions
- `Enter`: Select highlighted
- `Escape`: Close dropdown
- `Tab`: Accept first match

### Backlink Panel

**Location:** Right sidebar (desktop), bottom sheet (mobile)

**Content:**
```
┌─────────────────────────────────────────┐
│  Backlinks (3)                          │
├─────────────────────────────────────────┤
│ 📄 Project Plan                         │
│    "...timeline discussed in [[...]]"   │
│                                         │
│ 📄 Meeting Notes 2026-02-24             │
│    "...refer to [[...]] for details"    │
│                                         │
│ 📄 Architecture Decision                │
│    "...as documented in [[...]]"        │
└─────────────────────────────────────────┘
```

**Features:**
- Show all notes linking to current note
- Show snippet with link context (highlighted)
- Click to navigate (open linked note)
- Sort by: relevance, recency, alphabetical
- Toggle: show/hide backlink panel (Cmd/Ctrl+Shift+B)

### Command Palette (Cmd/Ctrl+P)

**Trigger:** Cmd/Ctrl+P

**Commands:**

| Command | Shortcut | Description |
|---------|----------|-------------|
| Create note | Cmd/Ctrl+N | New note, focus title |
| Quick switch | Cmd/Ctrl+O | Note search + navigate |
| Toggle preview | Cmd/Ctrl+Shift+P | Split-pane toggle |
| Insert wiki-link | Cmd/Ctrl+K | Wiki-link autocomplete |
| Search in note | Cmd/Ctrl+F | In-editor search |
| Export markdown | Cmd/Ctrl+E | Download .md file |
| Toggle backlinks | Cmd/Ctrl+Shift+B | Show/hide backlink panel |
| Toggle outline | Cmd/Ctrl+Shift+O | Show/hide outline |
| Version history | Cmd/Ctrl+Shift+H | Show version timeline |
| Delete note | Cmd/Ctrl+Shift+D | Soft-delete current note |
| Copy wiki-link | Cmd/Ctrl+Shift+L | Copy `[[note]]` to clipboard |
| Follow link | Cmd/Ctrl+Click | Navigate wiki-link |

### Markdown Shortcuts (Auto-Formatting)

| Trigger | Result | Example |
|---------|--------|---------|
| `# ` at line start | H1 heading | `# Title` → `<h1>Title</h1>` |
| `## ` at line start | H2 heading | `## Section` → `<h2>Section</h2>` |
| `- ` at line start | Bullet list | `- item` → `<li>item</li>` |
| `* ` at line start | Bullet list | `* item` → `<li>item</li>` |
| `1. ` at line start | Numbered list | `1. item` → `<ol><li>item</li>` |
| `- [ ] ` at line start | Task checkbox | `- [ ] task` → interactive |
| `> ` at line start | Blockquote | `> quote` → `<blockquote>` |
| ` ``` ` on empty line | Code fence | Opens code block |
| `---` on empty line | Horizontal rule | `<hr/>` |
| `**` + text + `**` | Bold | `**bold**` → `<strong>` |
| `*` + text + `*` | Italic | `*italic*` → `<em>` |
| `` ` `` + text + `` ` `` | Inline code | `` `code` `` → `<code>` |
| `==` + text + `==` | Highlight | `==text==` → `<mark>` |

---

## Replay and Idempotency Rules

### Idempotency Contract

**Key Generation:**
```rust
fn generate_idempotency_key() -> String {
    // UUID v4 for uniqueness
    uuid::Uuid::new_v4().to_string()
}
```

**Server-Side Deduplication:**
```
1. Receive patch with idempotency_key
2. Check if key exists in recent commits (last 100)
3. If exists → return cached response (same commit identity)
4. If new → process patch, cache response, return
```

**Client-Side Retry:**
```
1. Send patch with idempotency_key
2. If timeout → retry with SAME key
3. Server returns cached response → same result
4. If new key generated → potential duplicate commit
```

### Reconnect Contract

**Reconnect Flow:**
```
1. Detect WS disconnect (ping timeout or close event)
2. Enter `reconnecting` state
3. Show reconnecting indicator with countdown
4. Exponential backoff: 1s, 2s, 4s, 8s, 16s (max)
5. On reconnect: send `ack` with last acknowledged event_seq
6. Server replays missed events from cursor
7. Client applies replayed events to local draft
8. Flush queued offline patches
9. Resume normal operation
```

**Ack Message:**
```json
{
  "type": "ack",
  "note_id": "uuid",
  "event_seq": 42,
  "version": 5
}
```

**Replay Messages:**
```json
{
  "type": "note_event",
  "note_id": "uuid",
  "event_seq": 43,
  "version": 6,
  "event_type": "note.updated",
  "payload": {
    "patch_ops": [...],
    "actor": "user-uuid"
  }
}
```

### Cursor Semantics

**Client Cursor:**
```rust
struct EditorCursor {
    acknowledged_event_seq: u64,  // Last confirmed event
    pending_event_seq: u64,       // Latest local event
    base_version: u64,            // Version for next patch
}
```

**Stale Cursor Handling:**
```
1. Client sends patch with base_version=5
2. Server has current_version=7
3. Server returns `patch_rejected` with current_version
4. Client triggers full note refresh
5. Show conflict resolution UI
6. User resolves, new patch sent with updated base_version
```

---

## Conflict UX

### Conflict Detection

| Condition | Detection Method | Response |
|-----------|------------------|----------|
| HTTP 409 | Response code + body | Show conflict UI |
| WS `patch_rejected` | `reason: "VERSION_MISMATCH"` | Show conflict UI |
| Optimistic concurrency | `expected_version != current_version` | Refresh + resolve |

### Conflict Resolution UI

```
┌─────────────────────────────────────────────────────────────┐
│  ⚠️ Conflict Detected                                        │
│                                                             │
│  Another device modified this note while you were editing.  │
│  Choose how to resolve:                                     │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │  [ Keep Your Changes ]  [ Use Server Version ]        │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  Or resolve manually (merge view):                          │
│  ┌───────────────────────────────────────────────────────┐ │
│  │  <<<<<<< Yours (Local)                                │ │
│  │  Your local content changes                           │ │
│  │  =======                                              │ │
│  │  Server content from other device                     │ │
│  │  >>>>>>> Server (Version 7)                           │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  [ Reload from Server ]  [ Contact Support ]                │
└─────────────────────────────────────────────────────────────┘
```

### Conflict Prevention

**Strategies:**
1. **Debounce:** 600ms to batch rapid keystrokes (reduce patch frequency)
2. **Retry limit:** 2 attempts before showing conflict UI
3. **Optimistic UI:** Show changes immediately, reconcile later
4. **Event streaming:** Receive other users' changes in real-time
5. **Presence indicators:** Show when others are editing same note

---

## Title Edit Propagation

### Same-Cycle Update Flow

```
1. User edits title input
2. onBlur or Enter key → send PATCH
3. On success:
   a. Update note list immediately (same render cycle)
   b. Update browser tab title
   c. Update backlink displays (if any note links to this)
   d. Update search index (async, non-blocking)
   e. Update URL if on note detail page
4. On conflict: show inline error, revert to last saved
```

### Visual Feedback

**Title Input States:**
```
┌─────────────────────────────────┐
│ Meeting Notes 2026-02-24  ●     │  ← Unsaved changes (dot indicator)
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ Meeting Notes 2026-02-24  ⟳     │  ← Saving (spinner indicator)
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ Meeting Notes 2026-02-24  ✓     │  ← Saved (checkmark, fades after 2s)
└─────────────────────────────────┘
```

---

## Chrome Minimization

### Default State (Low-Noise)

**Always Visible:**
- Title input (top of editor)
- Editor content (full markdown surface)
- Sync status indicator (subtle, top-right corner)
- Menu toggle (per layout contract, top-right)

**Hidden by Default (Accessible via Shortcuts):**
- Save button → Cmd/Ctrl+S
- Version history → Cmd/Ctrl+Shift+H
- Delete → Cmd/Ctrl+Shift+D
- Export → Cmd/Ctrl+E
- Settings → Cmd/Ctrl+,

### Optional Modules (Toggleable)

| Module | Toggle Shortcut | Location |
|--------|-----------------|----------|
| Backlink panel | Cmd/Ctrl+Shift+B | Right sidebar |
| Outline/navigation | Cmd/Ctrl+Shift+O | Left sidebar |
| Metadata editor | Cmd/Ctrl+Shift+M | Below title |
| Command palette | Cmd/Ctrl+P | Modal overlay |
| Quick switcher | Cmd/Ctrl+O | Modal overlay |

---

## Accessibility Requirements

### Keyboard Navigation

| Key | Context | Action |
|-----|---------|--------|
| `Tab` | Anywhere | Cycle through interactive elements |
| `Shift+Tab` | Anywhere | Cycle backwards |
| `Escape` | Navigation open | Close navigation |
| `Escape` | Command palette | Close palette |
| `Escape` | Autocomplete | Close dropdown |
| `Enter` | Note list | Open selected note |
| `Arrow keys` | Note list | Navigate up/down |
| `Arrow keys` | Autocomplete | Navigate suggestions |

### Screen Reader Support

- Editor must have `role="textbox"` and `aria-multiline="true"`
- Title input must have `aria-label="Note title"`
- Sync status must use `aria-live="polite"` region
- Conflict banner must use `role="alert"`
- All icons must have `aria-label` or `aria-hidden`

### Focus Management

| Transition | Focus Target |
|------------|--------------|
| Open navigation | First note in list |
| Close navigation | Return focus to menu toggle |
| Select note | Focus editor title input |
| Open command palette | Focus search input |
| Close command palette | Return focus to editor |
| Show conflict UI | Focus "Keep Your Changes" button |

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Editor initial load | < 500ms |
| Keystroke to render | < 16ms (60fps) |
| Autosave debounce | 600ms (configurable) |
| Patch send to ACK | < 200ms (P95) |
| Conflict detection | < 100ms |
| Reconnect + replay | < 2s (100 events) |

---

## Related

- [Web app shell](web-app.md) — note creation + auth flows
- [Layout contract](layout-and-interaction.md) — responsive behavior
- [Notes domain](/docs/spec/domain/notes.md) — ID/title separation, versioning
- [WebSocket](/docs/spec/api/websocket.md) — realtime protocol
- [Findings traceability](findings-traceability.md) — requirement mapping
