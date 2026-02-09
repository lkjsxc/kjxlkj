//! Event registry: handler registration, firing, and group management.
//!
//! Type definitions live in [`events_types`].

use crate::events_types::{EventData, EventHandler, EventKind};

/// The event/autocmd registry.
#[derive(Debug, Clone)]
pub struct EventRegistry {
    handlers: Vec<EventHandler>,
    next_id: u64,
    /// Re-entry depth guard.
    max_depth: usize,
    current_depth: usize,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            next_id: 1,
            max_depth: 10,
            current_depth: 0,
        }
    }

    /// Register an event handler. Returns its ID.
    pub fn register(
        &mut self,
        event: EventKind,
        command: String,
        pattern: Option<String>,
        group: Option<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.handlers.push(EventHandler {
            id,
            event,
            pattern,
            command,
            group,
            enabled: true,
        });
        id
    }

    /// Remove a handler by ID.
    pub fn remove(&mut self, id: u64) -> bool {
        let len_before = self.handlers.len();
        self.handlers.retain(|h| h.id != id);
        self.handlers.len() < len_before
    }

    /// Clear all handlers in a group.
    pub fn clear_group(&mut self, group: &str) {
        self.handlers
            .retain(|h| h.group.as_deref() != Some(group));
    }

    /// Clear all handlers for a specific event.
    pub fn clear_event(&mut self, event: EventKind) {
        self.handlers.retain(|h| h.event != event);
    }

    /// Clear all handlers.
    pub fn clear_all(&mut self) {
        self.handlers.clear();
    }

    /// Fire an event — returns commands to execute.
    /// Checks re-entry depth to prevent infinite loops.
    pub fn fire(
        &mut self,
        event: EventKind,
        data: &EventData,
    ) -> Vec<String> {
        if self.current_depth >= self.max_depth {
            return Vec::new();
        }

        self.current_depth += 1;

        let commands: Vec<String> = self
            .handlers
            .iter()
            .filter(|h| h.event == event && h.enabled)
            .filter(|h| match (&h.pattern, &data.file) {
                (Some(pat), Some(file)) => glob_match(pat, file),
                (Some(_), None) => false,
                (None, _) => true,
            })
            .map(|h| h.command.clone())
            .collect();

        self.current_depth -= 1;
        commands
    }

    /// List all handlers, optionally filtered by event.
    pub fn list(
        &self,
        event: Option<EventKind>,
    ) -> Vec<&EventHandler> {
        self.handlers
            .iter()
            .filter(|h| event.is_none() || event == Some(h.event))
            .collect()
    }

    /// Number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Whether any handlers are registered.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// Set max re-entry depth.
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple glob matching: supports `*` as wildcard.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == text;
    }

    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        match text[pos..].find(part) {
            Some(idx) => {
                if i == 0 && idx != 0 {
                    return false;
                }
                pos += idx + part.len();
            }
            None => return false,
        }
    }

    if !pattern.ends_with('*') {
        text.ends_with(parts.last().unwrap_or(&""))
    } else {
        true
    }
}
