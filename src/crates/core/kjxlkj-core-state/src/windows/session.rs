use std::collections::HashSet;

use super::node_ops::collect_leaves;
use super::session_codec::{
    encode_kind, encode_tree_tokens, parse_fields, parse_focus_seq, parse_kinds, parse_tree,
    parse_u64, required_field,
};
use super::{WindowKind, WindowTree};

impl WindowTree {
    pub fn session_dump(&self) -> String {
        let mut kind_ids: Vec<u64> = self.kinds.keys().copied().collect();
        kind_ids.sort_unstable();
        let kinds = kind_ids
            .iter()
            .map(|id| {
                let kind = self.kinds.get(id).copied().unwrap_or(WindowKind::Buffer);
                format!("{id}:{}", encode_kind(kind))
            })
            .collect::<Vec<_>>()
            .join(",");

        let mut seq_ids: Vec<u64> = self.focus_seq.keys().copied().collect();
        seq_ids.sort_unstable();
        let focus_seq = seq_ids
            .iter()
            .map(|id| {
                let seq = self.focus_seq.get(id).copied().unwrap_or(0);
                format!("{id}:{seq}")
            })
            .collect::<Vec<_>>()
            .join(",");

        let mut tree_tokens = Vec::new();
        encode_tree_tokens(&self.root, &mut tree_tokens);
        let previous = self.previous.unwrap_or(0);
        format!(
            "v1;f={};p={};s={};n={};k={};q={};t={}",
            self.focused,
            previous,
            self.seq,
            self.next_id,
            kinds,
            focus_seq,
            tree_tokens.join(",")
        )
    }

    pub fn restore_session(&mut self, dump: &str) -> Result<(), String> {
        let (version, fields) = parse_fields(dump)?;
        if version != "v1" {
            return Err(format!("unsupported session version: {version}"));
        }

        let focused = parse_u64(required_field(&fields, "f")?, "focused")?;
        let previous_raw = parse_u64(required_field(&fields, "p")?, "previous")?;
        let seq_raw = parse_u64(required_field(&fields, "s")?, "seq")?;
        let next_raw = parse_u64(required_field(&fields, "n")?, "next_id")?;
        let kinds = parse_kinds(required_field(&fields, "k")?)?;
        let mut focus_seq = parse_focus_seq(required_field(&fields, "q")?)?;
        let root = parse_tree(required_field(&fields, "t")?)?;

        let mut leaves = Vec::new();
        collect_leaves(&root, &mut leaves);
        if leaves.is_empty() {
            return Err("restored tree has no leaves".to_string());
        }

        let leaf_set: HashSet<u64> = leaves.iter().copied().collect();
        if leaf_set.len() != leaves.len() {
            return Err("restored tree has duplicate leaf ids".to_string());
        }
        if !leaf_set.contains(&focused) {
            return Err(format!(
                "focused leaf {focused} is absent from restored tree"
            ));
        }
        if kinds.len() != leaf_set.len() || !kinds.keys().all(|id| leaf_set.contains(id)) {
            return Err("restored kinds map does not match tree leaves".to_string());
        }

        let mut seq = seq_raw;
        focus_seq.retain(|id, _| leaf_set.contains(id));
        for id in &leaves {
            if !focus_seq.contains_key(id) {
                seq = seq.saturating_add(1);
                focus_seq.insert(*id, seq);
            }
        }
        seq = seq.max(focus_seq.values().copied().max().unwrap_or(1));
        let previous = if previous_raw == 0 || !leaf_set.contains(&previous_raw) {
            None
        } else {
            Some(previous_raw)
        };
        let max_leaf = leaves.iter().copied().max().unwrap_or(1);
        let next_id = next_raw.max(max_leaf.saturating_add(1));

        self.root = root;
        self.kinds = kinds;
        self.focus_seq = focus_seq;
        self.focused = focused;
        self.previous = previous;
        self.seq = seq.max(1);
        self.next_id = next_id;
        Ok(())
    }
}
