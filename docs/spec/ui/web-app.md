# Web App Shell

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

The application MUST be fully functional when accessing the root URL directly:

1. **Unauthenticated user:**
   - If setup is incomplete → redirect to `/setup`
   - If setup is complete → redirect to `/login`
   - After login → redirect to `/` (now shows notes)

2. **Authenticated user:**
   - Redirect to `/` which renders the full app shell
   - Most recent note selected (or empty state if none)
   - Editor immediately usable

### URL Structure

| Path | Purpose | Auth Required |
|------|---------|---------------|
| `/` | App shell (notes + editor) | Yes |
| `/setup` | First-run registration | No (setup-locked only) |
| `/login` | Login form | No |
| `/notes` | Note list view (optional) | Yes |
| `/notes/{id}` | Direct note link | Yes |
| `/search?q=...` | Search results view | Yes |

### Client-Side Routing

- Use HTML5 History API (`pushState`, `popstate`)
- Root `/` serves the app shell; client handles sub-routes
- 404 for unknown routes → redirect to `/`

---

## Required Shell Views

### View 1: Setup (First-Run Only)

**Route:** `/setup`
**Condition:** No owner account exists

```
┌─────────────────────────────────────────┐
│                                         │
│   Welcome to kjxlkj                     │
│                                         │
│   Create Owner Account                  │
│   ┌─────────────────────────────────┐  │
│   │ Email                           │  │
│   │ Password                        │  │
│   │ Confirm Password                │  │
│   └─────────────────────────────────┘  │
│                                         │
│   [ Create Account ]                    │
│                                         │
└─────────────────────────────────────────┘
```

**Behavior:**
- Only available while setup is incomplete
- After submission → lock setup, create owner session, redirect to `/`

### View 2: Login

**Route:** `/login`
**Condition:** Setup complete, no active session

```
┌─────────────────────────────────────────┐
│                                         │
│   Sign in to kjxlkj                     │
│                                         │
│   ┌─────────────────────────────────┐  │
│   │ Email                           │  │
│   │ Password                        │  │
│   └─────────────────────────────────┘  │
│                                         │
│   [ Sign In ]                           │
│                                         │
│   Forgot password? (future)             │
│                                         │
└─────────────────────────────────────────┘
```

**Behavior:**
- Pre-auth `GET /api/auth/session` returns `401` (expected, non-fatal)
- On success → redirect to `/`
- On failure → show error inline

### View 3: Notes List (Default View)

**Route:** `/` (when no note selected)
**Condition:** Authenticated

```
┌─────────────────────────────────────────────────────────┐
│ Header: kjxlkj                        [Sync] [☰] [User] │
├──────────────┬──────────────────────────────────────────┤
│ ┌──────────┐ │  No note selected                        │
│ │ Search   │ │                                          │
│ ├──────────┤ │  Select a note from the list or         │
│ │ Note 1   │ │  create a new one.                       │
│ │ Note 2   │ │                                          │
│ │ Note 3   │ │  [ Create New Note ]                     │
│ │ ...      │ │                                          │
│ └──────────┘ │                                          │
└──────────────┴──────────────────────────────────────────┘
```

**Behavior:**
- Search filters list in real-time
- Click note → navigate to `/notes/{id}`, show editor
- "Create New Note" → POST `/notes`, navigate to new note

### View 4: Note Detail (Editor)

**Route:** `/notes/{id}` or `/` (with note selected)
**Condition:** Authenticated, note selected

```
┌─────────────────────────────────────────────────────────┐
│ Header: kjxlkj                        [Sync] [☰] [User] │
├──────────────┬──────────────────────────────────────────┤
│ ┌──────────┐ │  ┌────────────────────────────────────┐ │
│ │ Search   │ │  │ Note Title (editable)              │ │
│ ├──────────┤ │  ├────────────────────────────────────┤ │
│ │ Note 1   │ │  │                                    │ │
│ │ Note 2   │ │  │ Markdown editor content...         │ │
│ │ ● Note 3 │ │  │                                    │ │
│ │ Note 4   │ │  │                                    │ │
│ └──────────┘ │  └────────────────────────────────────┘ │
└──────────────┴──────────────────────────────────────────┘
```

**Behavior:**
- Title edit propagates same-cycle to list
- Autosave on keystroke (600ms debounce)
- Wiki-link autocomplete on `[[`
- Backlink panel (optional, right sidebar)

### View 5: Agent Runs (Optional Module)

**Route:** `/agent` (optional)
**Condition:** Authenticated, automation enabled

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
1. `POST /notes` with optional payload
2. If no title provided → server assigns datetime title
3. Response includes `note_id` and assigned title
4. Immediately select new note in list
5. Navigate to `/notes/{id}`
6. Focus title input for quick rename

### Default Title Rule

When creating a note without explicit title:

- **Server assigns:** Current timestamp as title
- **Format:** `YYYY-MM-DD HH:mm:ss` (server local timezone)
- **Returned in:** Create response and projection reads
- **User can:** Immediately edit title after creation

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

## Optional Modules

### Module Policy

- Optional modules MUST NOT displace baseline note editing
- Modules load lazily (on demand)
- Feature flags control visibility (per `data/config.json`)

### Module List

| Module | Feature Flag | Description |
|--------|--------------|-------------|
| Dashboard | `dashboard_enabled` | Workspace overview, stats |
| Librarian | `librarian_enabled` | Agent run review queue |
| Saved Views | `saved_views_enabled` | Persisted filters/sorts |

---

## Related

- [Editor flow](editor-flow.md) — markdown editing behavior
- [Layout contract](layout-and-interaction.md) — responsive behavior
- [Notes domain](/docs/spec/domain/notes.md) — ID/title separation
- [HTTP API](/docs/spec/api/http.md) — note endpoints
