//! Notification queue with priority, dedup, and timeout management.

use std::collections::VecDeque;

/// Priority level for notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotifyPriority { Low = 0, Normal = 1, High = 2, Urgent = 3 }

/// A queued notification with lifetime tracking.
#[derive(Debug, Clone)]
pub struct QueuedNotification {
    pub id: u64,
    pub text: String,
    pub priority: NotifyPriority,
    pub timeout_ms: Option<u64>,
    pub elapsed_ms: u64,
    pub dismissed: bool,
    pub source: Option<String>,
}

/// A notification queue that manages ordering, dedup, and dismissal.
#[derive(Debug, Clone)]
pub struct NotificationQueue {
    queue: VecDeque<QueuedNotification>,
    next_id: u64,
    max_visible: usize,
}

impl Default for NotificationQueue {
    fn default() -> Self { Self { queue: VecDeque::new(), next_id: 1, max_visible: 5 } }
}

impl NotificationQueue {
    pub fn new(max_visible: usize) -> Self {
        Self { queue: VecDeque::new(), next_id: 1, max_visible }
    }

    /// Push a notification. Returns its ID.
    pub fn push(&mut self, text: &str, priority: NotifyPriority, timeout_ms: Option<u64>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push_back(QueuedNotification {
            id, text: text.to_string(), priority, timeout_ms,
            elapsed_ms: 0, dismissed: false, source: None,
        });
        id
    }

    /// Push with a source tag for dedup.
    pub fn push_dedup(&mut self, text: &str, priority: NotifyPriority, timeout_ms: Option<u64>, source: &str) -> u64 {
        // Remove existing from same source
        self.queue.retain(|n| n.source.as_deref() != Some(source) || n.dismissed);
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push_back(QueuedNotification {
            id, text: text.to_string(), priority, timeout_ms,
            elapsed_ms: 0, dismissed: false, source: Some(source.to_string()),
        });
        id
    }

    /// Dismiss a notification by ID.
    pub fn dismiss(&mut self, id: u64) -> bool {
        if let Some(n) = self.queue.iter_mut().find(|n| n.id == id) {
            n.dismissed = true;
            true
        } else { false }
    }

    /// Advance elapsed time and expire timed-out notifications.
    pub fn tick(&mut self, delta_ms: u64) {
        for n in self.queue.iter_mut() {
            if n.dismissed { continue; }
            n.elapsed_ms += delta_ms;
            if let Some(timeout) = n.timeout_ms {
                if n.elapsed_ms >= timeout { n.dismissed = true; }
            }
        }
        self.queue.retain(|n| !n.dismissed);
    }

    /// Get visible notifications (up to max_visible, highest priority first).
    pub fn visible(&self) -> Vec<&QueuedNotification> {
        let mut active: Vec<_> = self.queue.iter().filter(|n| !n.dismissed).collect();
        active.sort_by(|a, b| b.priority.cmp(&a.priority));
        active.truncate(self.max_visible);
        active
    }

    /// Clear all notifications.
    pub fn clear(&mut self) { self.queue.clear(); }
    pub fn len(&self) -> usize { self.queue.iter().filter(|n| !n.dismissed).count() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_visible() {
        let mut q = NotificationQueue::new(3);
        q.push("a", NotifyPriority::Low, None);
        q.push("b", NotifyPriority::High, None);
        q.push("c", NotifyPriority::Normal, None);
        let vis = q.visible();
        assert_eq!(vis.len(), 3);
        assert_eq!(vis[0].text, "b"); // high first
    }

    #[test]
    fn timeout_expires() {
        let mut q = NotificationQueue::new(5);
        q.push("temp", NotifyPriority::Normal, Some(100));
        assert_eq!(q.len(), 1);
        q.tick(50);
        assert_eq!(q.len(), 1);
        q.tick(60);
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn dedup_by_source() {
        let mut q = NotificationQueue::new(5);
        q.push_dedup("v1", NotifyPriority::Normal, None, "diag");
        q.push_dedup("v2", NotifyPriority::Normal, None, "diag");
        assert_eq!(q.len(), 1);
        assert_eq!(q.visible()[0].text, "v2");
    }

    #[test]
    fn dismiss_by_id() {
        let mut q = NotificationQueue::new(5);
        let id = q.push("msg", NotifyPriority::Normal, None);
        assert!(q.dismiss(id));
        q.tick(0); // prune
        assert!(q.is_empty());
    }

    #[test]
    fn max_visible_cap() {
        let mut q = NotificationQueue::new(2);
        q.push("a", NotifyPriority::Low, None);
        q.push("b", NotifyPriority::Normal, None);
        q.push("c", NotifyPriority::High, None);
        let vis = q.visible();
        assert_eq!(vis.len(), 2);
    }
}
