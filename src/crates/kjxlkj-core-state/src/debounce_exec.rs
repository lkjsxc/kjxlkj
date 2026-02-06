//! Debounce execution — deterministic timer firing and coalescing with fake clock support.

use std::collections::HashMap;

/// A monotonic clock abstraction for deterministic testing.
#[derive(Debug, Clone, Copy)]
pub struct FakeClock { pub now_ms: u64 }

impl FakeClock {
    pub fn new() -> Self { Self { now_ms: 0 } }
    pub fn advance(&mut self, ms: u64) { self.now_ms += ms; }
}

impl Default for FakeClock { fn default() -> Self { Self::new() } }

/// A pending debounced action with absolute deadline.
#[derive(Debug, Clone)]
pub struct PendingAction {
    pub name: String,
    pub command: String,
    pub deadline_ms: u64,
    pub coalesced_count: u32,
}

/// Debounce manager using a fake clock for deterministic behavior.
#[derive(Debug, Clone)]
pub struct DebounceManager {
    pending: HashMap<String, PendingAction>,
    clock: FakeClock,
    fired: Vec<(String, String)>,
}

impl DebounceManager {
    pub fn new() -> Self { Self { pending: HashMap::new(), clock: FakeClock::new(), fired: Vec::new() } }

    /// Schedule or reschedule a debounced action.
    pub fn schedule(&mut self, name: &str, delay_ms: u64, command: &str) {
        let deadline = self.clock.now_ms + delay_ms;
        let entry = self.pending.entry(name.into()).or_insert_with(|| PendingAction {
            name: name.into(), command: command.into(), deadline_ms: deadline, coalesced_count: 0,
        });
        entry.deadline_ms = deadline;
        entry.command = command.into();
        entry.coalesced_count += 1;
    }

    /// Cancel a pending debounced action. Returns true if it was pending.
    pub fn cancel(&mut self, name: &str) -> bool { self.pending.remove(name).is_some() }

    /// Advance the clock and fire any expired actions. Returns fired command names.
    pub fn tick(&mut self, advance_ms: u64) -> Vec<String> {
        self.clock.advance(advance_ms);
        let now = self.clock.now_ms;
        let expired: Vec<_> = self.pending.iter()
            .filter(|(_, a)| a.deadline_ms <= now)
            .map(|(k, a)| (k.clone(), a.command.clone()))
            .collect();
        let mut names = Vec::new();
        for (name, command) in expired {
            self.pending.remove(&name);
            self.fired.push((name.clone(), command));
            names.push(name);
        }
        names
    }

    /// Get all fired actions (for testing).
    pub fn fired_actions(&self) -> &[(String, String)] { &self.fired }
    pub fn pending_count(&self) -> usize { self.pending.len() }
    pub fn current_time(&self) -> u64 { self.clock.now_ms }

    /// Check if a specific action is pending.
    pub fn is_pending(&self, name: &str) -> bool { self.pending.contains_key(name) }

    /// Get coalesced count for a pending action (how many times it was rescheduled).
    pub fn coalesced_count(&self, name: &str) -> u32 {
        self.pending.get(name).map(|a| a.coalesced_count).unwrap_or(0)
    }
}

impl Default for DebounceManager { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_and_fire() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("save", 100, ":w");
        assert_eq!(mgr.pending_count(), 1);
        let fired = mgr.tick(50);
        assert!(fired.is_empty());
        let fired = mgr.tick(60); // total 110ms
        assert_eq!(fired, vec!["save"]);
        assert_eq!(mgr.pending_count(), 0);
    }

    #[test]
    fn coalesce_resets_deadline() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("lint", 100, ":lint");
        mgr.tick(80); // not yet
        mgr.schedule("lint", 100, ":lint"); // reschedule at t=180
        let fired = mgr.tick(30); // t=110, not yet (deadline=180)
        assert!(fired.is_empty());
        let fired = mgr.tick(80); // t=190
        assert_eq!(fired, vec!["lint"]);
    }

    #[test]
    fn coalesced_count() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("typing", 50, ":complete");
        mgr.schedule("typing", 50, ":complete");
        mgr.schedule("typing", 50, ":complete");
        assert_eq!(mgr.coalesced_count("typing"), 3);
    }

    #[test]
    fn cancel_prevents_firing() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("save", 100, ":w");
        assert!(mgr.cancel("save"));
        let fired = mgr.tick(200);
        assert!(fired.is_empty());
    }

    #[test]
    fn multiple_independent() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("a", 50, ":a");
        mgr.schedule("b", 100, ":b");
        let fired = mgr.tick(60);
        assert_eq!(fired, vec!["a"]);
        let fired = mgr.tick(50); // t=110
        assert_eq!(fired, vec!["b"]);
    }

    #[test]
    fn fired_actions_log() {
        let mut mgr = DebounceManager::new();
        mgr.schedule("x", 10, ":x");
        mgr.tick(20);
        assert_eq!(mgr.fired_actions().len(), 1);
        assert_eq!(mgr.fired_actions()[0], ("x".into(), ":x".into()));
    }

    #[test]
    fn fake_clock_tracks_time() {
        let mut mgr = DebounceManager::new();
        assert_eq!(mgr.current_time(), 0);
        mgr.tick(100);
        assert_eq!(mgr.current_time(), 100);
        mgr.tick(50);
        assert_eq!(mgr.current_time(), 150);
    }
}
