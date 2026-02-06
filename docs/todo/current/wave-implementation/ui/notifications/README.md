# UI: Notifications (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement notifications and their rendering behavior.

## Defining documents (direct, normative)

- Notifications:
  - [/docs/spec/features/ui/notifications.md](/docs/spec/features/ui/notifications.md)

## Checklist

- [x] Placeholder scaffolding: define notification data model and queueing rules. — done: `NotificationQueue`, `QueuedNotification`, `NotifyPriority` in `notification_queue.rs` with dedup, timeout, and priority tests
- [ ] Minimal slice: implement minimal notifications with deterministic tests.
- [ ] Full conformance: implement all notification behaviors and styling.

