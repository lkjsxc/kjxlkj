/// Notification routing and dispatch system.

/// Notification severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity { Debug, Info, Warning, Error }

/// Notification source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifySource { Editor, Lsp, Plugin, Git, System }

/// A notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    pub severity: Severity,
    pub source: NotifySource,
    pub message: String,
    pub timestamp_ms: u64,
    pub dismissed: bool,
}

impl Notification {
    pub fn new(severity: Severity, source: NotifySource, message: impl Into<String>, ts: u64) -> Self {
        Self { severity, source, message: message.into(), timestamp_ms: ts, dismissed: false }
    }

    pub fn dismiss(&mut self) { self.dismissed = true; }
    pub fn age_ms(&self, now: u64) -> u64 { now.saturating_sub(self.timestamp_ms) }
}

/// Dispatch configuration controlling which notifications are shown.
#[derive(Debug, Clone)]
pub struct DispatchConfig {
    pub min_severity: Severity,
    pub max_visible: usize,
    pub auto_dismiss_ms: u64,
}

impl Default for DispatchConfig {
    fn default() -> Self { Self { min_severity: Severity::Info, max_visible: 5, auto_dismiss_ms: 5000 } }
}

/// Notification dispatcher — routes and manages notification lifecycle.
#[derive(Debug)]
pub struct Dispatcher {
    config: DispatchConfig,
    notifications: Vec<Notification>,
    next_id: u64,
}

impl Dispatcher {
    pub fn new(config: DispatchConfig) -> Self { Self { config, notifications: Vec::new(), next_id: 0 } }

    pub fn send(&mut self, severity: Severity, source: NotifySource, msg: impl Into<String>, ts: u64) -> Option<u64> {
        if severity < self.config.min_severity { return None; }
        let id = self.next_id; self.next_id += 1;
        self.notifications.push(Notification::new(severity, source, msg, ts));
        Some(id)
    }

    pub fn dismiss(&mut self, idx: usize) {
        if let Some(n) = self.notifications.get_mut(idx) { n.dismiss(); }
    }

    pub fn dismiss_source(&mut self, source: NotifySource) {
        for n in &mut self.notifications { if n.source == source { n.dismiss(); } }
    }

    pub fn gc(&mut self, now: u64) {
        self.notifications.retain(|n| !n.dismissed && n.age_ms(now) < self.config.auto_dismiss_ms);
    }

    pub fn visible(&self) -> impl Iterator<Item = &Notification> {
        self.notifications.iter().filter(|n| !n.dismissed).take(self.config.max_visible)
    }

    pub fn count_active(&self) -> usize { self.notifications.iter().filter(|n| !n.dismissed).count() }

    pub fn clear(&mut self) { self.notifications.clear(); }
}

/// Format a notification for display.
pub fn format_notification(n: &Notification) -> String {
    let prefix = match n.severity {
        Severity::Debug => "[D]",
        Severity::Info => "[I]",
        Severity::Warning => "[W]",
        Severity::Error => "[E]",
    };
    format!("{} {}", prefix, n.message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_and_visible() {
        let mut d = Dispatcher::new(DispatchConfig::default());
        d.send(Severity::Info, NotifySource::Editor, "hello", 100);
        assert_eq!(d.count_active(), 1);
        assert_eq!(d.visible().count(), 1);
    }

    #[test]
    fn filter_below_min() {
        let mut d = Dispatcher::new(DispatchConfig { min_severity: Severity::Warning, ..Default::default() });
        assert!(d.send(Severity::Debug, NotifySource::System, "dbg", 0).is_none());
        assert!(d.send(Severity::Info, NotifySource::System, "inf", 0).is_none());
        assert!(d.send(Severity::Warning, NotifySource::System, "wrn", 0).is_some());
    }

    #[test]
    fn dismiss_one() {
        let mut d = Dispatcher::new(DispatchConfig::default());
        d.send(Severity::Error, NotifySource::Lsp, "err1", 0);
        d.send(Severity::Error, NotifySource::Lsp, "err2", 0);
        d.dismiss(0);
        assert_eq!(d.count_active(), 1);
    }

    #[test]
    fn dismiss_source() {
        let mut d = Dispatcher::new(DispatchConfig::default());
        d.send(Severity::Info, NotifySource::Plugin, "p1", 0);
        d.send(Severity::Info, NotifySource::Editor, "e1", 0);
        d.dismiss_source(NotifySource::Plugin);
        assert_eq!(d.count_active(), 1);
    }

    #[test]
    fn gc_removes_old() {
        let mut d = Dispatcher::new(DispatchConfig { auto_dismiss_ms: 1000, ..Default::default() });
        d.send(Severity::Info, NotifySource::Editor, "old", 100);
        d.gc(2000);
        assert_eq!(d.count_active(), 0);
    }

    #[test]
    fn max_visible() {
        let mut d = Dispatcher::new(DispatchConfig { max_visible: 2, ..Default::default() });
        for i in 0..5 { d.send(Severity::Info, NotifySource::System, format!("m{}", i), 0); }
        assert_eq!(d.visible().count(), 2);
    }

    #[test]
    fn format() {
        let n = Notification::new(Severity::Warning, NotifySource::Git, "unstaged changes", 0);
        assert_eq!(format_notification(&n), "[W] unstaged changes");
    }

    #[test]
    fn notification_age() {
        let n = Notification::new(Severity::Info, NotifySource::Editor, "test", 500);
        assert_eq!(n.age_ms(1500), 1000);
    }
}
