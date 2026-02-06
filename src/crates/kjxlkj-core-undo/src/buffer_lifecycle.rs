/// Buffer lifecycle — swap, persistence, modification tracking, auto-save.
use std::collections::HashMap;
use std::path::PathBuf;

/// Swap file state for a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwapState { None, Clean(PathBuf), Dirty(PathBuf) }

/// Buffer modification tracking info.
#[derive(Debug, Clone)]
pub struct ModificationInfo {
    pub modified: bool,
    pub change_count: u64,
    pub last_saved_count: u64,
    pub auto_save_pending: bool,
}

impl ModificationInfo {
    pub fn new() -> Self {
        Self { modified: false, change_count: 0, last_saved_count: 0, auto_save_pending: false }
    }
    pub fn mark_changed(&mut self) {
        self.change_count += 1;
        self.modified = true;
        self.auto_save_pending = true;
    }
    pub fn mark_saved(&mut self) {
        self.last_saved_count = self.change_count;
        self.modified = false;
        self.auto_save_pending = false;
    }
    pub fn is_modified(&self) -> bool { self.modified }
    pub fn changes_since_save(&self) -> u64 { self.change_count - self.last_saved_count }
}

/// Lifecycle stage of a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleStage { Created, Loading, Loaded, Saving, Closing, Closed }

/// Buffer lifecycle state.
#[derive(Debug, Clone)]
pub struct BufferLifecycle {
    pub stage: LifecycleStage,
    pub mod_info: ModificationInfo,
    pub swap: SwapState,
    pub file_path: Option<PathBuf>,
    pub readonly: bool,
}

impl BufferLifecycle {
    pub fn new_empty() -> Self {
        Self { stage: LifecycleStage::Created, mod_info: ModificationInfo::new(),
            swap: SwapState::None, file_path: None, readonly: false }
    }
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        let p = path.into();
        Self { stage: LifecycleStage::Loading, mod_info: ModificationInfo::new(),
            swap: SwapState::None, file_path: Some(p), readonly: false }
    }
    pub fn mark_loaded(&mut self) { self.stage = LifecycleStage::Loaded; }
    pub fn mark_saving(&mut self) { self.stage = LifecycleStage::Saving; }
    pub fn mark_saved(&mut self) {
        self.stage = LifecycleStage::Loaded;
        self.mod_info.mark_saved();
        if let SwapState::Dirty(p) = &self.swap { self.swap = SwapState::Clean(p.clone()); }
    }
    pub fn mark_closing(&mut self) { self.stage = LifecycleStage::Closing; }
    pub fn mark_closed(&mut self) { self.stage = LifecycleStage::Closed; self.swap = SwapState::None; }
    pub fn can_close_safely(&self) -> bool { !self.mod_info.is_modified() }
}

/// Compute swap file path from a buffer's file path.
pub fn swap_path(file: &PathBuf) -> PathBuf {
    let name = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    file.with_file_name(format!(".{}.swp", name))
}

/// Auto-save policy configuration.
#[derive(Debug, Clone)]
pub struct AutoSavePolicy {
    pub enabled: bool,
    pub delay_ms: u64,
    pub on_focus_lost: bool,
}

impl Default for AutoSavePolicy {
    fn default() -> Self { Self { enabled: false, delay_ms: 1000, on_focus_lost: true } }
}

/// Determine which buffers need auto-saving.
pub fn buffers_needing_save(buffers: &HashMap<u64, BufferLifecycle>) -> Vec<u64> {
    buffers.iter()
        .filter(|(_, b)| b.mod_info.auto_save_pending && b.file_path.is_some()
            && b.stage == LifecycleStage::Loaded && !b.readonly)
        .map(|(id, _)| *id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_buffer() {
        let b = BufferLifecycle::new_empty();
        assert_eq!(b.stage, LifecycleStage::Created);
        assert!(!b.mod_info.is_modified());
        assert!(b.can_close_safely());
    }

    #[test]
    fn file_buffer_lifecycle() {
        let mut b = BufferLifecycle::from_file("/tmp/test.txt");
        assert_eq!(b.stage, LifecycleStage::Loading);
        b.mark_loaded();
        assert_eq!(b.stage, LifecycleStage::Loaded);
        b.mod_info.mark_changed();
        assert!(b.mod_info.is_modified()); assert!(!b.can_close_safely());
        b.mark_saving(); b.mark_saved();
        assert!(!b.mod_info.is_modified()); assert!(b.can_close_safely());
    }

    #[test]
    fn modification_tracking() {
        let mut m = ModificationInfo::new();
        assert_eq!(m.changes_since_save(), 0);
        m.mark_changed(); m.mark_changed(); m.mark_changed();
        assert_eq!(m.changes_since_save(), 3);
        m.mark_saved();
        assert_eq!(m.changes_since_save(), 0);
    }

    #[test]
    fn swap_path_computation() {
        let p = PathBuf::from("/home/user/file.txt");
        assert_eq!(swap_path(&p), PathBuf::from("/home/user/.file.txt.swp"));
    }

    #[test]
    fn auto_save_candidates() {
        let mut buffers = HashMap::new();
        let mut b1 = BufferLifecycle::from_file("/a.txt");
        b1.mark_loaded(); b1.mod_info.mark_changed();
        let mut b2 = BufferLifecycle::new_empty();
        b2.mod_info.mark_changed();
        buffers.insert(1, b1); buffers.insert(2, b2);
        let saves = buffers_needing_save(&buffers);
        assert_eq!(saves, vec![1]);
    }

    #[test]
    fn close_lifecycle() {
        let mut b = BufferLifecycle::new_empty();
        b.mark_closing();
        assert_eq!(b.stage, LifecycleStage::Closing);
        b.mark_closed();
        assert_eq!(b.stage, LifecycleStage::Closed);
        assert_eq!(b.swap, SwapState::None);
    }

    #[test]
    fn readonly_blocks_autosave() {
        let mut buffers = HashMap::new();
        let mut b = BufferLifecycle::from_file("/r.txt");
        b.mark_loaded(); b.mod_info.mark_changed(); b.readonly = true;
        buffers.insert(1, b);
        assert!(buffers_needing_save(&buffers).is_empty());
    }

    #[test]
    fn default_auto_save_policy() {
        let p = AutoSavePolicy::default();
        assert!(!p.enabled); assert_eq!(p.delay_ms, 1000); assert!(p.on_focus_lost);
    }
}
