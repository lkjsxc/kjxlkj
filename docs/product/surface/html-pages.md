# HTML Pages Contract

## Design System

### Color Palette (Dark Theme)

- Background Primary: `#1e1e1e` (main background)
- Background Secondary: `#252526` (sidebar)
- Background Tertiary: `#2d2d30` (cards, inputs)
- Accent Primary: `#7c3aed` (purple - active items)
- Accent Secondary: `#a78bfa` (light purple - hover)
- Success: `#22c55e` (green)
- Warning: `#f59e0b` (amber)
- Error: `#ef4444` (red)
- Text Primary: `#e4e4e7` (off-white)
- Text Secondary: `#a1a1aa` (gray)
- Text Muted: `#71717a` (dark gray)
- Border: `#3f3f46` (subtle borders)

### Typography

- Font Family: `system-ui, -apple-system, sans-serif`
- Monospace: `ui-monospace, Consolas, monospace`
- Heading Weight: 600
- Body Weight: 400
- Base Size: 16px
- Line Height: 1.6

### Spacing Scale

- xs: 4px
- sm: 8px
- md: 16px
- lg: 24px
- xl: 32px
- 2xl: 48px

### Border Radius

- Small: 4px
- Medium: 6px
- Large: 8px

## Page Contracts

### Landing Page (`GET /`)

#### Layout

- Full-height two-column layout
- Left sidebar (250px width)
- Main content area

#### Sidebar Elements

1. Logo/brand: "kjxlkj" at top
2. List of public notes (title extracted from first `# heading`)
3. Login link at bottom (if not logged in)

#### Main Content

- Welcome message or featured note
- Link to login for admin access

### Note Page (`GET /{slug}`)

#### Layout

- Same two-column layout as landing page
- Sidebar persistent across pages

#### Header Elements

1. Back navigation (to `/` or `/admin`)
2. Visibility toggle (admin only, above content)
3. Note slug display

#### Main Content

1. Rendered Markdown body
2. If admin: SimpleMDE editor (live-editable)
3. If guest: Read-only rendered Markdown

#### Footer Elements

1. Last updated date: "March 25, 2026 at 1:34 AM"
2. Previous/Next navigation links

### Admin Dashboard (`GET /admin`)

#### Layout

- Same two-column layout
- Sidebar shows ALL notes (including private)

#### Header Elements

1. "Admin" indicator
2. Logout button
3. Create New Note button (accent color)

#### Main Content: Notes List

1. Flat list of all notes
2. Each item shows:
   - Title (from first `# heading`)
   - Last updated date
   - Visibility indicator (lock icon for private)
3. Click to navigate to note
4. Empty state: "No notes yet" with create CTA

### History Page (`GET /records/{slug}/history`)

#### Layout

- Single-column modal or page
- Overlays note view

#### Elements

1. Header: "Revision History for {slug}"
2. List of revisions:
   - Revision number
   - Created timestamp
   - Preview of first line
3. Click to view full revision content
4. Close/back button

### Setup Page (`GET /setup`)

#### Layout

- Centered card on dark gradient background
- Max width: 400px
- Padding: 2xl

#### Elements

1. Logo/title header: "kjxlkj Setup"
2. Subtitle: "Create your admin account"
3. Form fields:
   - Username input (required, minlength 3)
   - Password input (required, minlength 8)
   - Confirm password input (required)
4. Submit button: "Create Account"

### Login Page (`GET /login`)

#### Layout

- Centered card matching setup style
- Max width: 400px

#### Elements

1. Logo/title header: "kjxlkj"
2. Subtitle: "Sign in"
3. Form fields:
   - Username input (required)
   - Password input (required)
4. Submit button: "Sign In"
5. Error banner (hidden by default)

## Interaction Contracts

### Auto-Save on Blur

- Editor triggers save when focus is lost
- Save indicator shows status: "Saving...", "Saved", "Error"
- Debounce: 500ms after blur before save

### Visibility Toggle

- Toggle switch positioned above editor
- Label: "Private" (locked) / "Public" (unlocked)
- Immediate save on toggle change
- Icon changes based on state

### Navigation

- Previous/Next arrows in footer
- Keyboard shortcuts: Left/Right arrows (when not editing)
- Updates URL without full page reload

## Accessibility

- Focus visible outlines
- ARIA labels on interactive elements
- Keyboard navigation support
- Color contrast ratio >= 4.5:1
