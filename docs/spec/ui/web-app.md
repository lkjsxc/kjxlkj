# Web App Shell — Root URL Contract

**Back:** [UI Root](/docs/spec/ui/README.md)

---

## UX Intent

- **Note-first baseline** — immediate editing on page load
- **Authoring confidence** outranks feature density
- **Auth transitions** MUST be deterministic and non-blocking
- **Root URL accessibility** — app MUST be fully usable at `http://localhost:8080/`

---

## Root URL Contract

### Entry Point: `GET /`

The application **MUST** be fully functional when accessing the root URL directly.

#### Unauthenticated User Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                     GET / (Unauthenticated)                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Client requests GET /                                        │
│  2. Server serves static index.html (app shell)                 │
│  3. App loads, checks session via GET /api/auth/session         │
│  4. If 401 → render Login View                                  │
│  5. If setup incomplete → render Setup View                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### Authenticated User Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                     GET / (Authenticated)                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Client requests GET /                                        │
│  2. Server serves static index.html (app shell)                 │
│  3. App loads, checks session via GET /api/auth/session         │
│  4. If 200 → render Notes List + Editor                         │
│  5. Most recent note selected (or empty state)                  │
│  6. Editor immediately usable                                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### URL Structure

| Path | Purpose | Auth Required | Component |
|------|---------|---------------|-----------|
| `/` | App shell (notes + editor) | No (graceful degradation) | `AppShell` |
| `/setup` | First-run registration | No (setup-locked only) | `SetupForm` |
| `/login` | Login form | No | `LoginForm` |
| `/notes` | Note list view | Yes | `NotesList` |
| `/notes/:id` | Direct note link | Yes | `NoteEditor` |
| `/search?q=...` | Search results view | Yes | `SearchResults` |
| `/agent` | Agent runs (optional) | Yes | `AgentRuns` |

### Client-Side Routing

- **Use HTML5 History API** (`pushState`, `popstate`)
- **Root `/`** serves the app shell; client handles sub-routes
- **404 for unknown routes** → redirect to `/`
- **Deep linking** supported (`/notes/:id` works on refresh)

---

## Required Shell Views

### View 1: Setup (First-Run Only)

**Route:** `/setup`
**Condition:** No owner account exists (detected via API)

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   Welcome to kjxlkj                                     │
│   All-in-docs workspace platform                        │
│                                                         │
│   Create Owner Account                                  │
│   ┌─────────────────────────────────────────────────┐  │
│   │ Email                                           │  │
│   │ ●●●●●●●●●●●●●●●●                                │  │
│   │ ●●●●●●●●●●●●●●●● (confirm)                      │  │
│   └─────────────────────────────────────────────────┘  │
│                                                         │
│   [ Create Account ]                                    │
│                                                         │
│   Already have an account? Sign in                      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Behavior:**
- Only available while setup is incomplete
- After submission → lock setup, create owner session, redirect to `/`
- Subsequent visits to `/setup` → redirect to `/`

**API Contract:**
```
POST /api/setup/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword",
  "password_confirm": "securepassword"
}

Response: 201 Created
{
  "user_id": "uuid",
  "email": "user@example.com",
  "session_token": "..."
}
```

### View 2: Login

**Route:** `/login`
**Condition:** Setup complete, no active session

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   Sign in to kjxlkj                                     │
│                                                         │
│   ┌─────────────────────────────────────────────────┐  │
│   │ email@example.com                               │  │
│   │ ●●●●●●●●●●●●●●●●                                │  │
│   └─────────────────────────────────────────────────┘  │
│                                                         │
│   [ Sign In ]                                           │
│                                                         │
│   Forgot password? (future)                             │
│   Don't have an account? Contact administrator          │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Behavior:**
- Pre-auth `GET /api/auth/session` returns `401` (expected, non-fatal)
- On success → redirect to `/`
- On failure → show error inline (no redirect loop)

**API Contract:**
```
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}

Response: 200 OK
{
  "user_id": "uuid",
  "email": "user@example.com",
  "session_token": "..."
}
```

### View 3: Notes List (Default View)

**Route:** `/` (when no note selected)
**Condition:** Authenticated

```
┌─────────────────────────────────────────────────────────┐
│ Header: kjxlkj                        [●] [☰] [User ▼]  │
├──────────────┬──────────────────────────────────────────┤
│ ┌──────────┐ │  No note selected                        │
│ │ 🔍 Search│ │                                          │
│ ├──────────┤ │  Select a note from the list or         │
│ │ Note 1   │ │  create a new one.                       │
│ │ Note 2   │ │                                          │
│ │ Note 3   │ │  [+ Create New Note]                     │
│ │ ...      │ │                                          │
│ └──────────┘ │                                          │
└──────────────┴──────────────────────────────────────────┘

Legend: [●] = sync status, [☰] = menu toggle (mobile)
```

**Behavior:**
- Search filters list in real-time (debounced, 300ms)
- Click note → navigate to `/notes/:id`, show editor
- "Create New Note" → POST `/api/notes`, navigate to new note
- Empty state shown when no notes exist

**API Contract:**
```
GET /api/notes?workspace_id=uuid&limit=50

Response: 200 OK
{
  "notes": [
    {
      "note_id": "uuid",
      "title": "Meeting Notes",
      "updated_at": "2026-02-24T14:30:00Z"
    }
  ]
}
```

### View 4: Note Detail (Editor)

**Route:** `/notes/:id` or `/` (with note selected)
**Condition:** Authenticated, note selected

```
┌─────────────────────────────────────────────────────────┐
│ Header: kjxlkj                        [●] [☰] [User ▼]  │
├──────────────┬──────────────────────────────────────────┤
│ ┌──────────┐ │  ┌────────────────────────────────────┐ │
│ │ 🔍 Search│ │  │ Meeting Notes 2026-02-24      [✎] │ │
│ ├──────────┤ │  ├────────────────────────────────────┤ │
│ │ Note 1   │ │  │                                    │ │
│ │ Note 2   │ │  │ # Meeting Notes                    │ │
│ │ ● Note 3 │ │  │                                    │ │
│ │ Note 4   │ │  │ ## Attendees                       │ │
│ │          │ │  │ - John                             │ │
│ │          │ │  │ - Jane                             │ │
│ └──────────┘ │  │                                    │ │
│              │  └────────────────────────────────────┘ │
└──────────────┴──────────────────────────────────────────┘

Legend: [●] = sync status, [✎] = edit mode indicator
```

**Behavior:**
- Title edit propagates same-cycle to list
- Autosave on keystroke (600ms debounce)
- Wiki-link autocomplete on `[[`
- Backlink panel (optional, right sidebar)
- Keyboard shortcuts (Cmd/Ctrl+P for commands)

**API Contract:**
```
GET /api/notes/:id

Response: 200 OK
{
  "note_id": "uuid",
  "title": "Meeting Notes 2026-02-24",
  "markdown": "# Meeting Notes\n\n...",
  "version": 5,
  "updated_at": "2026-02-24T14:30:00Z"
}
```

### View 5: Agent Runs (Optional Module)

**Route:** `/agent`
**Condition:** Authenticated, automation enabled (`features.librarian_enabled`)

Shows kjxlkj-agent run history, status, and review queue.

---

## Session UX Rules

### Pre-Auth Behavior

- `GET /api/auth/session` returns `401` → expected, non-fatal
- App renders login view, no error toast
- Local draft recovery path preserved (localStorage)

### Session Expiry

- On `401` from API → redirect to `/login`
- Preserve local draft in localStorage
- After re-login → restore draft, show "Session expired, draft recovered"

### Logout

- `POST /api/auth/logout` → clear session cookie
- Redirect to `/login`
- Clear localStorage drafts (optional, configurable)

---

## Note Creation Rules

### Create New Note Behavior

**Trigger:** Click "Create New Note" button or Cmd/Ctrl+N

**Steps:**
1. `POST /api/notes` with optional payload
2. If no title provided → server assigns datetime title
3. Response includes `note_id` and assigned title
4. Immediately select new note in list
5. Navigate to `/notes/:id`
6. Focus title input for quick rename

### Default Title Rule

When creating a note **without** an explicit title:

| Field | Value |
|-------|-------|
| **Server assigns** | Current timestamp as title |
| **Format** | `YYYY-MM-DD HH:mm:ss` (server local timezone) |
| **Returned in** | Create response and projection reads |
| **User can** | Immediately edit title after creation |

**Example:**
```
POST /api/notes
{
  "workspace_id": "uuid",
  "markdown": "Content here..."
}

Response: 201 Created
{
  "note_id": "note-uuid",
  "title": "2026-02-24 14:30:00",
  "markdown": "Content here...",
  "version": 1
}
```

### Note ID vs Title

| Field | Immutable | Description |
|-------|-----------|-------------|
| `note_id` | Yes | UUID, stable identity |
| `title` | No | Mutable display name |

**Contract:**
- `note_id` MUST remain stable while title changes
- API responses MUST include both fields
- UI MUST display title, use `note_id` for routing/links

---

## Editing Surface Rules

### Markdown-First

- Plain text editor with live preview (split or toggle)
- Syntax highlighting for code fences
- Wiki-link autocomplete and navigation

### Autosave-First

- Default: autosave on 600ms debounce
- Manual save: Cmd/Ctrl+S (bypass debounce)
- Status indicator: saving → saved → idle

### Low-Noise Chrome

**Visible by default:**
- Title input
- Editor content
- Sync status (subtle corner indicator)
- Menu toggle (per layout contract)

**Hidden (accessible via shortcuts):**
- Save button → Cmd/Ctrl+S
- Version history → Cmd/Ctrl+Shift+H
- Delete → Cmd/Ctrl+Shift+D
- Export → Cmd/Ctrl+E

---

## Responsive Shell

### Desktop (>1280px)

```
┌─────────────────────────────────────────────────────────┐
│ Header: Title                    [Sync] [User]          │
├──────────────┬──────────────────────────────────────────┤
│ Navigation   │  Editor                                  │
│ (persistent) │  (full width remaining)                  │
│ - 280px      │                                          │
│ - scroll     │                                          │
└──────────────┴──────────────────────────────────────────┘
```

### Mobile (≤1280px)

```
┌─────────────────────────────────────────┐
│ Header: Title         [Sync] [☰] [User] │
├─────────────────────────────────────────┤
│                                         │
│  Editor (full width)                    │
│                                         │
│  [Navigation opens as overlay]          │
│                                         │
└─────────────────────────────────────────┘
```

**Per:** [layout-and-interaction.md](layout-and-interaction.md)

---

## Optional Modules

### Module Policy

- Optional modules MUST NOT displace baseline note editing
- Modules load lazily (on demand)
- Feature flags control visibility (per `data/config.json`)

### Module List

| Module | Feature Flag | Route | Description |
|--------|--------------|-------|-------------|
| Dashboard | `dashboard_enabled` | `/dashboard` | Workspace overview, stats |
| Librarian | `librarian_enabled` | `/agent` | Agent run review queue |
| Saved Views | `saved_views_enabled` | `/views` | Persisted filters/sorts |

---

## Component Structure

### App Shell (`AppShell.tsx`)

```tsx
// Main application container
// - Header (always visible)
// - Navigation (conditional, per responsive rules)
// - Main content area (notes list or editor)
// - Overlay (mobile navigation backdrop)
```

### Header (`Header.tsx`)

```tsx
// Top navigation bar (56px desktop, 48px mobile)
// - App title / workspace name
// - Sync status indicator
// - Menu toggle (mobile only)
// - User menu
```

### Navigation (`Navigation.tsx`)

```tsx
// Note list sidebar (280px desktop, full-width overlay mobile)
// - Search box
// - Note list (scrollable)
// - Create new note button
```

### Editor (`MarkdownEditor.tsx`)

```tsx
// Main editing surface
// - Title input
// - Markdown editor (CodeMirror/ProseMirror)
// - Toolbar (optional)
// - Preview pane (optional, split)
```

---

## Accessibility Requirements

### Keyboard Navigation

| Key | Context | Action |
|-----|---------|--------|
| `Tab` | Anywhere | Cycle through interactive elements |
| `Shift+Tab` | Anywhere | Cycle backwards |
| `Escape` | Navigation open | Close navigation |
| `Escape` | Command palette | Close palette |
| `Enter` | Note list | Open selected note |
| `Arrow keys` | Note list | Navigate up/down |

### Screen Reader Support

- Editor MUST have `role="textbox"` and `aria-multiline="true"`
- Title input MUST have `aria-label="Note title"`
- Sync status MUST use `aria-live="polite"` region
- Conflict banner MUST use `role="alert"`

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Initial app load | < 2s (3G), < 500ms (cached) |
| Route transition | < 100ms |
| Note list render | < 200ms (100 notes) |
| Editor initial load | < 500ms |
| Keystroke to render | < 16ms (60fps) |

---

## Related

- [Editor flow](editor-flow.md) — markdown editing behavior
- [Layout contract](layout-and-interaction.md) — responsive behavior
- [Notes domain](/docs/spec/domain/notes.md) — ID/title separation
- [HTTP API](/docs/spec/api/http.md) — note endpoints
- [Sessions](/docs/spec/security/sessions.md) — auth contract
