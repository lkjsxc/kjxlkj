use std::collections::HashMap;

use super::node_ops::Node;
use super::{Axis, WindowKind};

pub(super) fn parse_fields(input: &str) -> Result<(String, HashMap<String, String>), String> {
    let mut parts = input.split(';');
    let version = parts
        .next()
        .ok_or_else(|| "session dump is empty".to_string())?
        .to_string();
    let mut fields = HashMap::new();
    for part in parts {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid session segment: {part}"))?;
        fields.insert(key.to_string(), value.to_string());
    }
    Ok((version, fields))
}

pub(super) fn required_field<'a>(
    fields: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, String> {
    fields
        .get(key)
        .map(|value| value.as_str())
        .ok_or_else(|| format!("missing session field: {key}"))
}

pub(super) fn parse_u64(input: &str, field: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|error| format!("invalid {field} value '{input}': {error}"))
}

pub(super) fn parse_kinds(input: &str) -> Result<HashMap<u64, WindowKind>, String> {
    if input.is_empty() {
        return Err("kinds field is empty".to_string());
    }
    let mut out = HashMap::new();
    for part in input.split(',') {
        let (id_raw, kind_raw) = part
            .split_once(':')
            .ok_or_else(|| format!("invalid kind entry: {part}"))?;
        let id = parse_u64(id_raw, "kind id")?;
        let kind = decode_kind(kind_raw)?;
        out.insert(id, kind);
    }
    Ok(out)
}

pub(super) fn parse_focus_seq(input: &str) -> Result<HashMap<u64, u64>, String> {
    if input.is_empty() {
        return Ok(HashMap::new());
    }
    let mut out = HashMap::new();
    for part in input.split(',') {
        let (id_raw, seq_raw) = part
            .split_once(':')
            .ok_or_else(|| format!("invalid focus-seq entry: {part}"))?;
        let id = parse_u64(id_raw, "focus-seq id")?;
        let seq = parse_u64(seq_raw, "focus-seq value")?;
        out.insert(id, seq);
    }
    Ok(out)
}

pub(super) fn parse_tree(input: &str) -> Result<Node, String> {
    let mut stack = Vec::new();
    for token in input.split(',') {
        if let Some(raw_id) = token.strip_prefix('L') {
            let id = parse_u64(raw_id, "leaf id")?;
            stack.push(Node::Leaf(id));
            continue;
        }
        let axis = match token {
            "H" => Axis::Horizontal,
            "V" => Axis::Vertical,
            _ => return Err(format!("invalid tree token: {token}")),
        };
        let right = stack
            .pop()
            .ok_or_else(|| "invalid tree: missing right child".to_string())?;
        let left = stack
            .pop()
            .ok_or_else(|| "invalid tree: missing left child".to_string())?;
        stack.push(Node::Split(axis, Box::new(left), Box::new(right)));
    }
    if stack.len() != 1 {
        return Err("invalid tree: stack did not collapse to one root".to_string());
    }
    stack
        .pop()
        .ok_or_else(|| "invalid tree: no root after parsing".to_string())
}

pub(super) fn encode_tree_tokens(node: &Node, out: &mut Vec<String>) {
    match node {
        Node::Leaf(id) => out.push(format!("L{id}")),
        Node::Split(axis, left, right) => {
            encode_tree_tokens(left, out);
            encode_tree_tokens(right, out);
            out.push(match axis {
                Axis::Horizontal => "H".to_string(),
                Axis::Vertical => "V".to_string(),
            });
        }
    }
}

pub(super) fn encode_kind(kind: WindowKind) -> char {
    match kind {
        WindowKind::Buffer => 'B',
        WindowKind::Explorer => 'E',
        WindowKind::Terminal => 'T',
    }
}

fn decode_kind(input: &str) -> Result<WindowKind, String> {
    match input {
        "B" => Ok(WindowKind::Buffer),
        "E" => Ok(WindowKind::Explorer),
        "T" => Ok(WindowKind::Terminal),
        _ => Err(format!("unknown window kind token: {input}")),
    }
}
