# Visual Verify Service Contract

## Purpose

Run browser-rendered screenshot and contrast checks against the live compose app.

## Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Behavior

- Launch against the compose `app` service.
- Capture desktop and compact layouts.
- Fail on unreadable controls, missing dark-shell structure, or loud compact-nav controls.

## Container Requirements

- Playwright runtime with Chromium.
- Access to the compose network.
- A deterministic script checked into the repository.
