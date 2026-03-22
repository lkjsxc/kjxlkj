# UI Contract Verification Checklist

## Core Page Checks

1. `GET /` redirects to `/setup` before setup completion.
2. `GET /setup` renders password-first setup form with fixed admin username.
3. `GET /login` renders password-only login form.
4. `GET /admin` renders dashboard, not dedicated editor page.
5. `GET /article/{slug}` renders last-updated and prev/next links.

## Inline Edit Checks

1. Authenticated admin sees inline edit form on article page.
2. Edit form fields include `title`, `private`, `body`, `last_known_revision`.
3. Private toggle appears above body field.
4. Save and preview buttons do not exist.
5. Autosave trigger window is 2 seconds and blur-triggered.

## History Checks

1. `GET /article/{slug}/history` is admin-only.
2. History list shows commit ID, timestamp, and message.
3. `POST /article/{slug}/history/restore` restores selected revision.

## Privacy and Visibility Checks

1. New articles default to private.
2. Public users cannot access private articles.
3. Admin sees private/public markers across dashboard/home.

## Required Validation Commands

```bash
cargo run --bin kjxlkj -- docs validate-topology
cargo run --bin kjxlkj -- docs validate-terms
cargo run --bin kjxlkj -- quality check-lines
```
