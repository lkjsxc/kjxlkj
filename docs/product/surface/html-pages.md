# HTML Pages Contract

## Design System

### Color Palette

- Primary: `#1a1a2e` (deep navy)
- Secondary: `#16213e` (dark blue)
- Accent: `#0f3460` (medium blue)
- Highlight: `#e94560` (coral red)
- Success: `#00d9a5` (mint green)
- Warning: `#ffc107` (amber)
- Error: `#dc3545` (red)
- Text Primary: `#f8f9fa` (off-white)
- Text Secondary: `#a0aec0` (gray)
- Background: `#0f0f1a` (near black)

### Typography

- Font Family: `system-ui, -apple-system, sans-serif`
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
- Medium: 8px
- Large: 12px
- Full: 9999px

## Page Contracts

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
5. Footer text with version

#### Validation States

- Empty field: red border, error message below
- Password mismatch: error message below confirm field
- Success: redirect to `/login`

### Login Page (`GET /login`)

#### Layout

- Centered card matching setup style
- Max width: 400px

#### Elements

1. Logo/title header: "kjxlkj"
2. Subtitle: "Sign in to admin panel"
3. Form fields:
   - Username input (required)
   - Password input (required)
4. Submit button: "Sign In"
5. Error banner (hidden by default)

#### Error States

- Invalid credentials: red banner with message
- Session expired: amber banner with message

### Admin Page (`GET /admin`)

#### Layout

- Full-width responsive layout
- Sidebar navigation (collapsible on mobile)
- Main content area with header

#### Header Elements

1. Logo: "kjxlkj admin"
2. User indicator
3. Logout button

#### Sidebar Elements

1. Dashboard link
2. Records link (active indicator)
3. Settings link (future)

#### Main Content: Records View

1. Page title: "Records"
2. Action bar:
   - Create Record button (highlight color)
   - Search input
3. Records table:
   - Columns: ID, Title, Tags, Revision, Updated
   - Row actions: View, Edit, Delete
4. Empty state: "No records found" with create CTA

#### Record Create/Edit Modal

1. Modal overlay (dark backdrop)
2. Modal card (max-width 600px)
3. Form fields:
   - Title input (required)
   - Body textarea (resizable)
   - Tags input (comma-separated)
4. Action buttons:
   - Cancel (secondary)
   - Save (primary)

#### Delete Confirmation Modal

1. Warning icon
2. Confirmation text with record ID
3. Cancel and Delete buttons

### Home Page (`GET /`)

#### Before Setup

- Redirect to `/setup`

#### After Setup (Public Mode)

- Minimal page with service name
- "Records available at /v1/records"
- Login link

#### After Setup (Admin Session)

- Redirect to `/admin`

## Animation Contracts

### Transitions

- Default duration: 200ms
- Easing: ease-out
- Apply to: hover states, focus states, modals

### Hover Effects

- Buttons: slight brightness increase
- Links: underline appearance
- Table rows: subtle background change

### Modal Behavior

- Fade in backdrop
- Scale up modal from 95% to 100%
- Focus trap within modal

## Accessibility

- Focus visible outlines
- ARIA labels on interactive elements
- Keyboard navigation support
- Color contrast ratio >= 4.5:1
- Form validation messages linked to inputs
