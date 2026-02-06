# UI: Notifications (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement notifications and their rendering behavior.

## Defining documents (direct, normative)

- Notifications:
  - [/docs/spec/features/ui/notifications.md](/docs/spec/features/ui/notifications.md)

## Checklist

- [x] Placeholder scaffolding: define notification data model and queueing rules. — done: `NotificationQueue`, `QueuedNotification`, `NotifyPriority` in `notification_queue.rs` with dedup, timeout, and priority tests
- [x] Minimal slice: implement minimal notifications with deterministic tests. — done: `notification_render.rs` with `NotifPosition`, `render_notification()`, `wrap_text()`, and `max_visible_notifications()`
- [x] Full conformance: implement all notification behaviors and styling.
  - notification_dispatch.rs: Severity, NotifySource, Notification, DispatchConfig, Dispatcher (send/dismiss/dismiss_source/gc/visible), format_notification

