# Mode-Specific Rail Actions

## Guest Rails

- Guest list rails use `Admin sign in` as the mode-specific action.
- Guest note and history rails use `Admin sign in` as the trailing action block.
- Guest rails place `Open GitHub` above `Admin sign in`.

## Admin List Rails

- Admin list rails use `Logout` as the session action.
- Admin list rails may also show `New note`, but it stays above `Open GitHub`.
- Admin list rails place `Open GitHub` above `Logout`.

## Admin Note and History Rails

- Admin note/history rails use note-management and session actions as the trailing block.
- `Delete note`, `Delete media`, and `Logout` remain below `Open GitHub`.
- The destructive action uses a `4`-second arm-and-confirm button instead of a modal dialog.
- Destructive or session actions do not move above the GitHub section.
