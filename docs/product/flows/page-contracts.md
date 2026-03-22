# Server-Rendered Page Contracts

This document defines target full-page contracts for `/`, `/setup`, `/login`, `/search`,
`/admin`, `/admin/settings`, and `/admin/trash` before richer HTMX and JavaScript behaviors
are layered on top.

## Global Page Rules

- Non-HTMX requests to page routes return complete HTML documents (`<!doctype html>` through `</html>`).
- Every page exposes one stable root container ID for automation and enhancement hooks.
- Validation and auth errors re-render the same page route with deterministic error containers.
- Forms remain functional without JavaScript.

## Route Matrix

| Route | Setup incomplete | Setup complete + logged out | Setup complete + admin session |
| --- | --- | --- | --- |
| `GET /` | `302` to `/setup` | `200` home page | `200` home page with admin visibility |
| `GET /setup` | `200` setup page | `404` | `404` |
| `GET /login` | `302` to `/setup` | `200` login page | `303` to `/admin` |
| `GET /search` | `302` to `/setup` | `200` search page | `200` search page |
| `GET /admin` | `302` to `/setup` | `302` to `/login` | `200` admin shell page |
| `GET /admin/settings` | `302` to `/setup` | `302` to `/login` | `200` admin settings page |
| `GET /admin/trash` | `302` to `/setup` | `302` to `/login` | `200` admin trash page |

## `/` Home Page Contract

- Root container: `<main id="home-page">`.
- Article list container: `#home-article-list`.
- Logged-out rendering includes only public articles.
- Logged-in admin rendering may include private items and admin affordances.
- Article links always use canonical `/article/{slug}` URLs.
- Article rendering does not display author/byline attribution.
- Page is rendered within shared shell IDs from [navigation-shell.md](navigation-shell.md).

## `/setup` Page Contract

- Root container: `<main id="setup-page">`.
- Form ID: `#setup-form` with fields:
  - `password`
- Error region: `#setup-errors` with `aria-live="polite"`.
- `POST /setup` outcomes:
  - `400` + setup page re-render for missing/invalid inputs.
  - `303` redirect to `/login` when first admin is created.
  - deterministic failure signal when setup is locked after first admin creation.

## `/login` Page Contract

- Root container: `<main id="login-page">`.
- Form ID: `#login-form` with fields:
  - `password`
- Login form does not accept or require username input; identity is fixed to `admin`.
- Error region: `#login-errors` with `aria-live="polite"`.
- `POST /login` outcomes:
  - `400` for malformed payloads (same page re-rendered).
  - `401` for invalid credentials (same page re-rendered).
  - `303` redirect to `/admin` on success with session cookie set.

## `/admin` Page Contract

- Root container: `<main id="admin-page">`.
- Required dashboard regions:
  - `#admin-create-form`
  - `#admin-article-list`
- Dashboard does not host dedicated edit/preview panes.
- Editing is performed on `/article/{slug}` via inline editor.

## `/search` Page Contract

- Root container: `<main id="search-page">`.
- Required regions:
  - `#search-form`
  - `#search-query`
  - `#search-results`
- Search page is available to non-admin and admin users after setup completion.
- Privacy filtering applies to results based on session role.
- Page is rendered within shared shell IDs from [navigation-shell.md](navigation-shell.md).

## `/admin/settings` Page Contract

- Root container: `<main id="admin-settings-page">`.
- Required regions:
  - `#admin-settings-form`
  - `#admin-settings-status`
  - `#admin-settings-errors`
- Route is admin-only and uses standard admin guards.
- Page is rendered within shared shell IDs from [navigation-shell.md](navigation-shell.md).

## `/admin/trash` Page Contract

- Root container: `<main id="admin-trash-page">`.
- Required regions:
  - `#admin-trash-list`
  - `#admin-trash-status`
- Route is admin-only and uses standard admin guards.
- Page is rendered within shared shell IDs from [navigation-shell.md](navigation-shell.md).
