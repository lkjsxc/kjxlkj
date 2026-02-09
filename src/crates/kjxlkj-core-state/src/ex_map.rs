/// Map/unmap command handling and key notation parsing.
use crate::editor::EditorState;
use crate::mappings::{KeyMapping, MapMode};

/// Known map command prefixes.
const MAP_PREFIXES: &[&str] = &[
    "nmap", "nnoremap", "nunmap", "imap", "inoremap", "iunmap", "vmap", "vnoremap", "vunmap",
    "xmap", "xnoremap", "xunmap", "cmap", "cnoremap", "cunmap", "omap", "onoremap", "ounmap",
    "tmap", "tnoremap", "tunmap", "smap", "snoremap", "sunmap", "map", "noremap", "unmap", "map!",
    "noremap!", "unmap!",
];

/// Check whether `cmd` is a mapping-related ex command.
pub(crate) fn is_map_command(cmd: &str) -> bool {
    let word = cmd.split_whitespace().next().unwrap_or("");
    MAP_PREFIXES.contains(&word)
}

/// Parse the map prefix → (mode, recursive, is_unmap, rest).
fn parse_map_cmd_prefix(cmd: &str) -> (MapMode, bool, bool, &str) {
    let word = cmd.split_whitespace().next().unwrap_or("");
    let rest = cmd[word.len()..].trim_start();

    match word {
        "nmap" => (MapMode::Normal, true, false, rest),
        "nnoremap" => (MapMode::Normal, false, false, rest),
        "nunmap" => (MapMode::Normal, false, true, rest),
        "imap" => (MapMode::Insert, true, false, rest),
        "inoremap" => (MapMode::Insert, false, false, rest),
        "iunmap" => (MapMode::Insert, false, true, rest),
        "vmap" => (MapMode::Visual, true, false, rest),
        "vnoremap" => (MapMode::Visual, false, false, rest),
        "vunmap" => (MapMode::Visual, false, true, rest),
        "xmap" => (MapMode::Visual, true, false, rest),
        "xnoremap" => (MapMode::Visual, false, false, rest),
        "xunmap" => (MapMode::Visual, false, true, rest),
        "cmap" => (MapMode::CmdLine, true, false, rest),
        "cnoremap" => (MapMode::CmdLine, false, false, rest),
        "cunmap" => (MapMode::CmdLine, false, true, rest),
        "omap" => (MapMode::OperatorPending, true, false, rest),
        "onoremap" => (MapMode::OperatorPending, false, false, rest),
        "ounmap" => (MapMode::OperatorPending, false, true, rest),
        "tmap" => (MapMode::Terminal, true, false, rest),
        "tnoremap" => (MapMode::Terminal, false, false, rest),
        "tunmap" => (MapMode::Terminal, false, true, rest),
        "smap" => (MapMode::Select, true, false, rest),
        "snoremap" => (MapMode::Select, false, false, rest),
        "sunmap" => (MapMode::Select, false, true, rest),
        "map" => (MapMode::Normal, true, false, rest),
        "noremap" => (MapMode::Normal, false, false, rest),
        "unmap" => (MapMode::Normal, false, true, rest),
        "map!" => (MapMode::Insert, true, false, rest),
        "noremap!" => (MapMode::Insert, false, false, rest),
        "unmap!" => (MapMode::Insert, false, true, rest),
        _ => (MapMode::Normal, true, false, rest),
    }
}

impl EditorState {
    pub(crate) fn handle_map_command(&mut self, cmd: &str) {
        let (mode, recursive, is_unmap, rest) = parse_map_cmd_prefix(cmd);
        let rest = rest.trim();

        if is_unmap {
            if rest.is_empty() {
                self.notify_error("E474: Missing key sequence");
                return;
            }
            let keys = super::key_notation::parse_key_notation(rest);
            self.mappings.remove(mode, &keys);
            return;
        }

        if rest.is_empty() {
            let maps = self.mappings.list(mode);
            if maps.is_empty() {
                self.notify_info("No mappings found");
            } else {
                let count = maps.len();
                self.notify_info(&format!("{count} mapping(s) defined"));
            }
            return;
        }

        let (lhs, rhs) = match rest.split_once(char::is_whitespace) {
            Some((l, r)) => (l.trim(), r.trim()),
            None => {
                let keys = super::key_notation::parse_key_notation(rest);
                let maps = self.mappings.list(mode);
                let found = maps.iter().find(|m| m.from == keys);
                if let Some(m) = found {
                    self.notify_info(&format!(
                        "{rest} → {}",
                        m.description.as_deref().unwrap_or("(mapped)")
                    ));
                } else {
                    self.notify_info(&format!("No mapping for {rest}"));
                }
                return;
            }
        };

        let from_keys = super::key_notation::parse_key_notation(lhs);
        let to_keys = super::key_notation::parse_key_notation(rhs);

        self.mappings.add(
            mode,
            KeyMapping {
                from: from_keys,
                to: to_keys,
                recursive,
                description: Some(format!("{lhs} → {rhs}")),
            },
        );
        self.notify_info(&format!("Mapped: {lhs} → {rhs}"));
    }
}
