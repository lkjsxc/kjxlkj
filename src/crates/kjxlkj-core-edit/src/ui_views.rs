//! UI view management: views, tab pages, and the view manager.

use kjxlkj_core_types::BufferId;
use serde::{Deserialize, Serialize};

/// The kind of view displayed in a pane.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViewKind {
    Buffer,
    Terminal,
    Explorer,
    Help,
    Preview,
    QuickFix,
    LocationList,
    Empty,
}

/// A single view (pane) in the editor layout.
#[derive(Debug, Clone)]
pub struct View {
    pub id: u64,
    pub kind: ViewKind,
    pub buffer_id: Option<BufferId>,
    pub active: bool,
}

/// A tab page containing one or more views.
#[derive(Debug, Clone)]
pub struct TabPage {
    pub views: Vec<View>,
}

impl Default for TabPage {
    fn default() -> Self {
        Self::new()
    }
}

impl TabPage {
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }

    pub fn active_view(&self) -> Option<&View> {
        self.views.iter().find(|v| v.active)
    }
}

/// Manages multiple tab pages.
pub struct ViewManager {
    pub tabs: Vec<TabPage>,
    pub active_tab: usize,
    next_view_id: u64,
}

impl Default for ViewManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewManager {
    pub fn new() -> Self {
        let empty_view = View {
            id: 1,
            kind: ViewKind::Empty,
            buffer_id: None,
            active: true,
        };
        Self {
            tabs: vec![TabPage {
                views: vec![empty_view],
            }],
            active_tab: 0,
            next_view_id: 2,
        }
    }

    /// Create a new view in the active tab.
    pub fn create_view(&mut self, kind: ViewKind, buffer_id: Option<BufferId>) -> u64 {
        let id = self.next_view_id;
        self.next_view_id += 1;
        // Deactivate current views
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            for v in &mut tab.views {
                v.active = false;
            }
            tab.views.push(View {
                id,
                kind,
                buffer_id,
                active: true,
            });
        }
        id
    }

    /// Close a view by ID. Returns true if found and removed.
    pub fn close_view(&mut self, view_id: u64) -> bool {
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            let len_before = tab.views.len();
            tab.views.retain(|v| v.id != view_id);
            if tab.views.len() < len_before {
                if !tab.views.iter().any(|v| v.active) {
                    if let Some(v) = tab.views.last_mut() {
                        v.active = true;
                    }
                }
                return true;
            }
        }
        false
    }

    /// Add a new empty tab page and switch to it.
    pub fn new_tab(&mut self) {
        let id = self.next_view_id;
        self.next_view_id += 1;
        let view = View {
            id,
            kind: ViewKind::Empty,
            buffer_id: None,
            active: true,
        };
        self.tabs.push(TabPage { views: vec![view] });
        self.active_tab = self.tabs.len() - 1;
    }

    /// Return a reference to the active view.
    pub fn active_view(&self) -> Option<&View> {
        self.tabs.get(self.active_tab)?.active_view()
    }

    /// Generate a tab line label string.
    pub fn tab_line_label(&self) -> String {
        self.tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                let marker = if i == self.active_tab { ">" } else { " " };
                let name = tab
                    .active_view()
                    .map(|v| format!("{:?}", v.kind))
                    .unwrap_or_else(|| "Empty".into());
                format!("{marker}{}", name)
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_close_view() {
        let mut mgr = ViewManager::new();
        let id = mgr.create_view(ViewKind::Buffer, Some(BufferId(1)));
        assert_eq!(mgr.active_view().unwrap().id, id);
        assert!(mgr.close_view(id));
    }

    #[test]
    fn new_tab() {
        let mut mgr = ViewManager::new();
        mgr.new_tab();
        assert_eq!(mgr.tabs.len(), 2);
        assert_eq!(mgr.active_tab, 1);
    }

    #[test]
    fn tab_label() {
        let mgr = ViewManager::new();
        let label = mgr.tab_line_label();
        assert!(label.contains(">"));
    }
}
