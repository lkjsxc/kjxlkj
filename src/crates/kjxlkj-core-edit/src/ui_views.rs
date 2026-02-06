/// View types and composition — editor views, splits, tab pages.

/// Unique view identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(pub u64);

/// View type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    Buffer, Terminal, Explorer, Help, Preview, QuickFix, LocationList, Empty,
}

/// A view in the editor layout.
#[derive(Debug, Clone)]
pub struct View {
    pub id: ViewId,
    pub kind: ViewKind,
    pub title: String,
    pub width: u16,
    pub height: u16,
    pub focusable: bool,
    pub closeable: bool,
}

impl View {
    pub fn buffer(id: ViewId, title: &str) -> Self {
        Self { id, kind: ViewKind::Buffer, title: title.into(),
            width: 0, height: 0, focusable: true, closeable: true }
    }
    pub fn terminal(id: ViewId) -> Self {
        Self { id, kind: ViewKind::Terminal, title: "Terminal".into(),
            width: 0, height: 0, focusable: true, closeable: true }
    }
    pub fn explorer(id: ViewId) -> Self {
        Self { id, kind: ViewKind::Explorer, title: "Explorer".into(),
            width: 30, height: 0, focusable: true, closeable: false }
    }
}

/// Tab page containing views.
#[derive(Debug, Clone)]
pub struct TabPage {
    pub id: usize,
    pub views: Vec<ViewId>,
    pub active_view: Option<ViewId>,
}

impl TabPage {
    pub fn new(id: usize) -> Self { Self { id, views: Vec::new(), active_view: None } }
    pub fn add_view(&mut self, vid: ViewId) {
        self.views.push(vid);
        if self.active_view.is_none() { self.active_view = Some(vid); }
    }
    pub fn remove_view(&mut self, vid: ViewId) {
        self.views.retain(|v| *v != vid);
        if self.active_view == Some(vid) { self.active_view = self.views.first().copied(); }
    }
}

/// View manager for the editor.
#[derive(Debug, Default)]
pub struct ViewManager {
    views: Vec<View>,
    tabs: Vec<TabPage>,
    active_tab: usize,
    next_view_id: u64,
}

impl ViewManager {
    pub fn new() -> Self { let mut vm = Self::default(); vm.tabs.push(TabPage::new(0)); vm }

    pub fn create_view(&mut self, kind: ViewKind, title: &str) -> ViewId {
        let id = ViewId(self.next_view_id);
        self.next_view_id += 1;
        self.views.push(View { id, kind, title: title.into(), width: 0, height: 0,
            focusable: true, closeable: true });
        if let Some(tab) = self.tabs.get_mut(self.active_tab) { tab.add_view(id); }
        id
    }

    pub fn close_view(&mut self, vid: ViewId) {
        self.views.retain(|v| v.id != vid);
        for tab in &mut self.tabs { tab.remove_view(vid); }
    }

    pub fn get_view(&self, vid: ViewId) -> Option<&View> { self.views.iter().find(|v| v.id == vid) }
    pub fn active_view(&self) -> Option<ViewId> { self.tabs.get(self.active_tab)?.active_view }

    pub fn new_tab(&mut self) -> usize {
        let id = self.tabs.len();
        self.tabs.push(TabPage::new(id));
        self.active_tab = id;
        id
    }

    pub fn tab_count(&self) -> usize { self.tabs.len() }
    pub fn view_count(&self) -> usize { self.views.len() }
}

/// Compose a status string for tab line.
pub fn tab_line_label(tabs: &[TabPage], active: usize) -> String {
    tabs.iter().enumerate().map(|(i, t)| {
        let marker = if i == active { "*" } else { "" };
        format!("[{}{}:{}]", i + 1, marker, t.views.len())
    }).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_get_view() {
        let mut vm = ViewManager::new();
        let id = vm.create_view(ViewKind::Buffer, "main.rs");
        assert!(vm.get_view(id).is_some());
        assert_eq!(vm.view_count(), 1);
    }

    #[test]
    fn close_view_updates_active() {
        let mut vm = ViewManager::new();
        let id1 = vm.create_view(ViewKind::Buffer, "a.rs");
        let _id2 = vm.create_view(ViewKind::Buffer, "b.rs");
        vm.close_view(id1);
        assert_eq!(vm.view_count(), 1);
    }

    #[test]
    fn new_tab() {
        let mut vm = ViewManager::new();
        vm.new_tab();
        assert_eq!(vm.tab_count(), 2);
    }

    #[test]
    fn tab_line_label_format() {
        let tabs = vec![TabPage { id: 0, views: vec![ViewId(1)], active_view: Some(ViewId(1)) },
            TabPage { id: 1, views: vec![ViewId(2), ViewId(3)], active_view: Some(ViewId(2)) }];
        let label = tab_line_label(&tabs, 0);
        assert!(label.contains("[1*:1]"));
        assert!(label.contains("[2:2]"));
    }

    #[test]
    fn view_kinds() {
        let v = View::buffer(ViewId(1), "test");
        assert_eq!(v.kind, ViewKind::Buffer);
        let t = View::terminal(ViewId(2));
        assert_eq!(t.kind, ViewKind::Terminal);
        let e = View::explorer(ViewId(3));
        assert_eq!(e.kind, ViewKind::Explorer);
        assert_eq!(e.width, 30);
    }

    #[test]
    fn tab_page_add_remove() {
        let mut tp = TabPage::new(0);
        tp.add_view(ViewId(1));
        tp.add_view(ViewId(2));
        assert_eq!(tp.active_view, Some(ViewId(1)));
        tp.remove_view(ViewId(1));
        assert_eq!(tp.active_view, Some(ViewId(2)));
    }

    #[test]
    fn active_view() {
        let mut vm = ViewManager::new();
        let id = vm.create_view(ViewKind::Buffer, "x");
        assert_eq!(vm.active_view(), Some(id));
    }
}
