//! Buffer listing, filtering, and management utilities.

use crate::buffer_state::BufferState;
use std::collections::HashMap;
use kjxlkj_core_types::BufferId;

/// Buffer list filter for `:ls` variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferFilter { All, Listed, Unlisted, Modified, Active(BufferId) }

/// A single entry in the buffer list display.
#[derive(Debug, Clone)]
pub struct BufferListEntry {
    pub id: BufferId,
    pub name: String,
    pub line_count: usize,
    pub modified: bool,
    pub listed: bool,
    pub active: bool,
    pub alternate: bool,
    pub readonly: bool,
}

impl BufferListEntry {
    pub fn flags(&self) -> String {
        let mut f = String::with_capacity(4);
        f.push(if self.active { '%' } else if self.alternate { '#' } else { ' ' });
        f.push(if self.modified { '+' } else { ' ' });
        f.push(if self.readonly { '=' } else { ' ' });
        f
    }
}

/// Build buffer list from editor state.
pub fn build_buffer_list(
    buffers: &HashMap<BufferId, BufferState>,
    active_bid: Option<BufferId>,
    alternate_bid: Option<BufferId>,
    filter: BufferFilter,
) -> Vec<BufferListEntry> {
    let mut entries: Vec<BufferListEntry> = buffers.values()
        .filter(|b| match filter {
            BufferFilter::All => true,
            BufferFilter::Listed => b.listed,
            BufferFilter::Unlisted => !b.listed,
            BufferFilter::Modified => b.modified,
            BufferFilter::Active(bid) => b.id == bid,
        })
        .map(|b| BufferListEntry {
            id: b.id,
            name: b.file_path.clone().unwrap_or_else(|| "[No Name]".into()),
            line_count: b.line_count(),
            modified: b.modified, listed: b.listed,
            active: Some(b.id) == active_bid,
            alternate: Some(b.id) == alternate_bid,
            readonly: b.readonly,
        })
        .collect();
    entries.sort_by_key(|e| e.id.0);
    entries
}

/// Format buffer list for `:ls` display.
pub fn format_buffer_list(entries: &[BufferListEntry]) -> String {
    if entries.is_empty() { return String::new(); }
    entries.iter()
        .map(|e| format!("{:>3}{} \"{}\" line {}", e.id.0, e.flags(), e.name, e.line_count))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buffers() -> HashMap<BufferId, BufferState> {
        let mut m = HashMap::new();
        let mut b1 = BufferState::new(BufferId(1));
        b1.set_file_path("main.rs");
        b1.modified = true;
        m.insert(BufferId(1), b1);
        let mut b2 = BufferState::new(BufferId(2));
        b2.set_file_path("lib.rs");
        b2.listed = false;
        m.insert(BufferId(2), b2);
        let b3 = BufferState::from_text(BufferId(3), "hello\nworld\n");
        m.insert(BufferId(3), b3);
        m
    }

    #[test]
    fn list_all() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, Some(BufferId(1)), None, BufferFilter::All);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn list_listed_only() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, None, None, BufferFilter::Listed);
        assert_eq!(list.len(), 2); // b2 unlisted
    }

    #[test]
    fn list_modified_only() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, None, None, BufferFilter::Modified);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, BufferId(1));
    }

    #[test]
    fn active_and_alternate_flags() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, Some(BufferId(1)), Some(BufferId(2)), BufferFilter::All);
        let e1 = list.iter().find(|e| e.id == BufferId(1)).unwrap();
        assert!(e1.flags().contains('%'));
        let e2 = list.iter().find(|e| e.id == BufferId(2)).unwrap();
        assert!(e2.flags().contains('#'));
    }

    #[test]
    fn format_output() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, Some(BufferId(1)), None, BufferFilter::Listed);
        let output = format_buffer_list(&list);
        assert!(output.contains("main.rs"));
        assert!(output.contains("+"));
    }

    #[test]
    fn sorted_by_id() {
        let bufs = make_buffers();
        let list = build_buffer_list(&bufs, None, None, BufferFilter::All);
        assert!(list[0].id.0 < list[1].id.0);
    }

    #[test]
    fn empty_buffers() {
        let bufs = HashMap::new();
        let list = build_buffer_list(&bufs, None, None, BufferFilter::All);
        assert!(list.is_empty());
        assert!(format_buffer_list(&list).is_empty());
    }
}
