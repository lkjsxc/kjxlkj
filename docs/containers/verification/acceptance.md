# Acceptance Contract

## Required Pass Conditions

1. All gates in verify service pass.
2. App service responds on `/healthz`.
3. Public read endpoints return valid JSON.
4. Write endpoints enforce token auth.

## Completion Rule

A release candidate is accepted only when all pass conditions hold.
