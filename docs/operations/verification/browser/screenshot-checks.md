# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop guest note page.
- Desktop admin note page.
- Desktop admin dashboard.
- Compact note page with drawer closed.
- Compact note page with drawer open.

## Required Assertions

- Buttons render readable text or intentional icon-only chrome.
- The product uses dark layered surfaces by default.
- Compact menu and close controls are visually quiet.
- Rail content, note content, and history entries remain readable at captured widths.
