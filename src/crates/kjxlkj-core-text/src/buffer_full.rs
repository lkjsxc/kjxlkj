/// Full buffer behaviors — alternate, hidden, special types, modified tracking.

/// Buffer type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType { Normal, Scratch, Help, QuickFix, Terminal, Prompt, Popup }

/// Buffer flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferFlags {
    pub modified: bool,
    pub readonly: bool,
    pub listed: bool,
    pub loaded: bool,
    pub modifiable: bool,
}

impl Default for BufferFlags {
    fn default() -> Self {
        Self { modified: false, readonly: false, listed: true, loaded: true, modifiable: true }
    }
}

/// Full buffer state extending the basic text buffer.
#[derive(Debug)]
pub struct BufferInfo {
    pub id: u64,
    pub name: Option<String>,
    pub buf_type: BufferType,
    pub flags: BufferFlags,
    pub line_count: usize,
    pub filetype: Option<String>,
    pub encoding: String,
}

impl BufferInfo {
    pub fn new(id: u64) -> Self {
        Self { id, name: None, buf_type: BufferType::Normal, flags: BufferFlags::default(),
            line_count: 0, filetype: None, encoding: "utf-8".into() }
    }

    pub fn scratch(id: u64, name: impl Into<String>) -> Self {
        let mut b = Self::new(id); b.buf_type = BufferType::Scratch;
        b.name = Some(name.into()); b.flags.listed = false; b
    }

    pub fn is_special(&self) -> bool { !matches!(self.buf_type, BufferType::Normal) }
    pub fn display_name(&self) -> &str { self.name.as_deref().unwrap_or("[No Name]") }
}

/// Alternate buffer tracker (:e #, Ctrl-^).
#[derive(Debug, Default)]
pub struct AlternateTracker { current: Option<u64>, alternate: Option<u64> }

impl AlternateTracker {
    pub fn new() -> Self { Self::default() }

    pub fn switch_to(&mut self, buf_id: u64) {
        if self.current != Some(buf_id) {
            self.alternate = self.current;
            self.current = Some(buf_id);
        }
    }

    pub fn current(&self) -> Option<u64> { self.current }
    pub fn alternate(&self) -> Option<u64> { self.alternate }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.alternate);
    }
}

/// Buffer list operations.
pub fn filter_listed(buffers: &[BufferInfo]) -> Vec<&BufferInfo> {
    buffers.iter().filter(|b| b.flags.listed).collect()
}

pub fn find_by_name<'a>(buffers: &'a [BufferInfo], name: &str) -> Option<&'a BufferInfo> {
    buffers.iter().find(|b| b.name.as_deref() == Some(name))
}

pub fn modified_count(buffers: &[BufferInfo]) -> usize {
    buffers.iter().filter(|b| b.flags.modified).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_info_defaults() {
        let b = BufferInfo::new(1); assert_eq!(b.display_name(), "[No Name]");
        assert!(!b.is_special()); assert!(!b.flags.modified);
    }

    #[test]
    fn scratch_buffer() {
        let b = BufferInfo::scratch(2, "[Scratch]");
        assert!(b.is_special()); assert!(!b.flags.listed);
    }

    #[test]
    fn alternate_tracker() {
        let mut t = AlternateTracker::new();
        t.switch_to(1); t.switch_to(2);
        assert_eq!(t.current(), Some(2));
        assert_eq!(t.alternate(), Some(1));
    }

    #[test]
    fn alternate_swap() {
        let mut t = AlternateTracker::new();
        t.switch_to(1); t.switch_to(2); t.swap();
        assert_eq!(t.current(), Some(1)); assert_eq!(t.alternate(), Some(2));
    }

    #[test]
    fn filter_listed_buffers() {
        let bufs = vec![BufferInfo::new(1), BufferInfo::scratch(2, "s")];
        assert_eq!(filter_listed(&bufs).len(), 1);
    }

    #[test]
    fn find_by_name_test() {
        let mut b = BufferInfo::new(1); b.name = Some("foo.rs".into());
        let bufs = vec![b];
        assert!(find_by_name(&bufs, "foo.rs").is_some());
        assert!(find_by_name(&bufs, "bar.rs").is_none());
    }

    #[test]
    fn modified_count_test() {
        let mut b1 = BufferInfo::new(1); b1.flags.modified = true;
        let b2 = BufferInfo::new(2);
        assert_eq!(modified_count(&[b1, b2]), 1);
    }

    #[test]
    fn buffer_type_special() {
        for bt in [BufferType::Scratch, BufferType::Help, BufferType::QuickFix,
                    BufferType::Terminal, BufferType::Prompt, BufferType::Popup] {
            let mut b = BufferInfo::new(1); b.buf_type = bt;
            assert!(b.is_special());
        }
    }
}
