# Acceptance Contract

## Required Pass Conditions

1. All gates in verify service pass.
2. App service responds on `/healthz`.
3. Fresh instances redirect `/` to `/setup`.
4. Setup and login flows create an admin session.
5. Guest note pages expose previous/next and public history links for public notes.
6. Admin note pages expose dashboard navigation, history navigation, and the `Public` checkbox.
7. Write endpoints enforce session auth.

## Completion Rule

A release candidate is accepted only when all pass conditions hold.
