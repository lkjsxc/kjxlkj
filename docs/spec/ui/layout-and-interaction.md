# Layout and Interaction Contract

**Back:** [UI Root](/docs/spec/ui/README.md)

---

## Layout Rules — Single Responsive Tree

### Core Principles

1. **One component tree** MUST serve desktop and mobile
2. **Independent scroll** for navigation/list and editor/content regions
3. **Editor/content priority** — visual hierarchy favors markdown content

### Region Definitions

```
┌─────────────────────────────────────────────────────────────┐
│  Header (fixed height: 56px desktop, 48px mobile)           │
│  - App title / workspace name                                │
│  - Menu toggle (conditional, per threshold below)            │
│  - Sync status indicator                                     │
├─────────────────────────────────────────────────────────────┤
│  Body (flex, fills remaining height)                         │
│  ┌─────────────────┬─────────────────────────────────────┐  │
│  │ Navigation      │  Editor/Content                     │  │
│  │ (conditional)   │  (always visible)                   │  │
│  │ - Note list     │  - Title input                      │  │
│  │ - Search        │  - Markdown editor                  │  │
│  │ - Backlinks     │  - Preview (optional split)         │  │
│  │                 │                                     │  │
│  │ Width: varies   │  Width: fills remaining             │  │
│  │ Scroll: auto    │  Scroll: auto                       │  │
│  └─────────────────┴─────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## Menu Toggle Threshold Rule — 2/3 Screen Width

### Canonical Breakpoint

The compact-menu behavior threshold is set at **two-thirds of common desktop viewport widths**:

| Viewport Width | Menu Behavior |
|----------------|---------------|
| `> 1280px` | Persistent split navigation (always visible) |
| `≤ 1280px` | Compact mode — top-right toggle controls visibility |

### Rationale

- 1280px represents approximately the 2/3 point of common desktop ranges (1920px → 1280px ≈ 67%)
- Below this threshold, navigation consumes too much editor space
- Toggle pattern matches mobile UX familiarity

### CSS Implementation

```css
/* Persistent navigation above 1280px */
@media (min-width: 1281px) {
  .navigation-panel {
    display: block;
    width: 280px;
    flex-shrink: 0;
  }
  .menu-toggle {
    display: none; /* Not needed */
  }
}

/* Compact mode at or below 1280px */
@media (max-width: 1280px) {
  .navigation-panel {
    position: fixed;
    top: 56px; /* Below header */
    left: 0;
    width: 280px;
    height: calc(100vh - 56px);
    transform: translateX(-100%);
    transition: transform 0.2s ease;
    z-index: 100;
  }
  .navigation-panel.open {
    transform: translateX(0);
  }
  .menu-toggle {
    display: block;
    position: fixed;
    top: 12px;
    right: 12px;
    z-index: 101;
  }
  .overlay {
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 99;
  }
  .overlay.visible {
    display: block;
  }
}
```

### Interaction Rules

| Width | Navigation State | User Action | Result |
|-------|------------------|-------------|--------|
| `> 1280px` | Always visible | N/A | N/A |
| `≤ 1280px`, closed | Hidden | Click toggle | Open navigation, show overlay |
| `≤ 1280px`, open | Visible | Click toggle | Close navigation, hide overlay |
| `≤ 1280px`, open | Visible | Click overlay | Close navigation, hide overlay |
| `≤ 1280px`, open | Visible | Select note | Close navigation, hide overlay, focus editor |
| `≤ 1280px`, open | Visible | Press Escape | Close navigation, hide overlay |

---

## Responsive Rules

### Compact Mode Auto-Close

**On note selection in compact mode (≤1280px):**

1. Close navigation panel (animate out)
2. Hide overlay
3. Focus editor content area
4. Update browser history (pushState)

### Touch Targets

| Element | Minimum Size |
|---------|--------------|
| Menu toggle | 44×44px |
| Note list items | 48px height |
| Editor toolbar buttons | 40×40px |
| Navigation tap targets | 44px minimum dimension |

### 320px Minimum Width

At `320px` viewport width:

- **NO horizontal scroll** in core note workflows
- Navigation panel: full-width overlay when open
- Editor: full-width, no sidebars
- Touch targets: maintain 44px minimum

### Focus Transitions

| Transition | Behavior |
|------------|----------|
| Open navigation | Focus first note in list |
| Close navigation | Return focus to menu toggle |
| Select note | Focus editor title input |
| Open command palette | Focus search input |

---

## Interaction Quality Rules

### Visual Hierarchy

1. **Markdown content** — highest contrast, largest area
2. **Title input** — prominent, always visible
3. **Navigation** — secondary, collapsible
4. **Chrome (buttons, indicators)** — minimal, unobtrusive

### Sync State Feedback

| State | Visual Indicator |
|-------|------------------|
| `saved` | Green dot (subtle, top-right) |
| `saving` | Spinning indicator (subtle) |
| `conflict` | Orange warning icon + banner |
| `offline` | Grey icon + "Offline" label |
| `reconnecting` | Pulsing indicator + countdown |

### Keyboard Navigation

| Key | Context | Action |
|-----|---------|--------|
| `Tab` | Anywhere | Cycle through interactive elements |
| `Escape` | Navigation open | Close navigation |
| `Escape` | Command palette | Close palette |
| `Enter` | Note list | Open selected note |
| `Arrow keys` | Note list | Navigate up/down |

---

## Layout States

### Desktop (`> 1280px`)

```
┌─────────────────────────────────────────────────────────┐
│ Header: Title                    [Sync] [User]          │
├──────────────┬──────────────────────────────────────────┤
│ Navigation   │  Editor                                  │
│ - Search     │  ┌────────────────────────────────────┐ │
│ - Note List  │  │ Title Input                        │ │
│ - Backlinks  │  ├────────────────────────────────────┤ │
│              │  │ Markdown Editor                    │ │
│              │  │                                    │ │
│              │  │                                    │ │
│              │  └────────────────────────────────────┘ │
└──────────────┴──────────────────────────────────────────┘
```

### Mobile (`≤ 1280px`, Navigation Closed)

```
┌─────────────────────────────────────────┐
│ Header: Title         [Sync] [☰] [User] │
├─────────────────────────────────────────┤
│                                         │
│  Editor                                 │
│  ┌───────────────────────────────────┐ │
│  │ Title Input                       │ │
│  ├───────────────────────────────────┤ │
│  │ Markdown Editor                   │ │
│  │                                   │ │
│  └───────────────────────────────────┘ │
│                                         │
└─────────────────────────────────────────┘
```

### Mobile (`≤ 1280px`, Navigation Open)

```
┌─────────────────────────────────────────┐
│ Header: Title         [Sync] [✕] [User] │
├──────────────┬──────────────────────────┤
│ Navigation   │ Overlay (tap to close)   │
│ - Search     │                          │
│ - Note List  │                          │
│ - Backlinks  │                          │
│              │                          │
│              │                          │
│              │                          │
└──────────────┴──────────────────────────┘
```

---

## Related

- [Editor flow](editor-flow.md) — markdown editing behavior
- [UX requirements](reconstruction-ux-requirements.md) — normative UX matrix
- [Findings traceability](findings-traceability.md) — requirement mapping
- [Web app shell](web-app.md) — view definitions
