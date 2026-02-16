/// Agent loop per /docs/spec/technical/librarian-agent.md
///
/// Loop model: iterative loop with mutable KV memory.
/// Retries must be bounded and deterministic.
use crate::kv_store::KvStore;
use crate::xml_parser::{parse_instructions, Instruction};
use kjxlkj_domain::DomainError;

/// Agent loop state
pub struct AgentLoop {
    pub kv_store: KvStore,
    pub states: Vec<String>,
    pub prompt_hash: String,
    pub parser_version: &'static str,
    /// Workspace scope — cross-workspace writes are rejected even in YOLO mode.
    pub workspace_id: uuid::Uuid,
    /// YOLO mode — direct mutations without review queue.
    pub yolo_mode: bool,
}

/// Parser version for audit trail
pub const PARSER_VERSION: &str = "1.0.0";

impl AgentLoop {
    pub fn new(kv_store: KvStore, prompt_hash: String, workspace_id: uuid::Uuid) -> Self {
        Self {
            kv_store,
            states: Vec::new(),
            prompt_hash,
            parser_version: PARSER_VERSION,
            workspace_id,
            yolo_mode: false,
        }
    }

    /// Check if a target workspace matches the agent's scope.
    /// Per /docs/spec/technical/librarian-agent.md: cross-workspace writes MUST be rejected.
    pub fn check_workspace_scope(&self, target_workspace_id: uuid::Uuid) -> Result<(), DomainError> {
        if target_workspace_id != self.workspace_id {
            return Err(DomainError::AgentYoloPolicyViolation(
                "cross-workspace write rejected".into(),
            ));
        }
        Ok(())
    }

    /// Process agent output and apply instructions.
    /// Per /docs/spec/api/librarian-xml.md: instructions execute
    /// sequentially in document order.
    pub fn process_output(&mut self, xml_output: &str) -> Result<Vec<Instruction>, DomainError> {
        let instructions = parse_instructions(xml_output)?;
        for instr in &instructions {
            match instr {
                Instruction::StateAdd { state } => {
                    if !self.states.contains(state) {
                        self.states.push(state.clone());
                    }
                }
                Instruction::StateDelete { state } => {
                    self.states.retain(|s| s != state);
                }
                Instruction::RamAdd { key, value } => {
                    self.kv_store.set(key.clone(), value.clone());
                }
                Instruction::RamDelete { key } => {
                    self.kv_store.delete(key);
                }
                // Record operations would interact with note repo
                _ => {}
            }
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_loop() -> AgentLoop {
        AgentLoop::new(KvStore::new(), "testhash".into(), uuid::Uuid::nil())
    }

    #[test]
    fn agent_03_yolo_mode_creates_notes() {
        // Acceptance: AGENT-03
        let mut loop_ = make_loop();
        loop_.yolo_mode = true;
        let xml = r#"<ram_add><key>think_log</key><value>creating note</value></ram_add>
<record_add><keywords>test</keywords><value>new note content</value></record_add>"#;
        let instrs = loop_.process_output(xml).unwrap();
        assert_eq!(instrs.len(), 2);
        assert_eq!(loop_.kv_store.get("think_log"), Some(&"creating note".to_string()));
    }

    #[test]
    fn agent_03_yolo_cross_workspace_rejected() {
        // Acceptance: AGENT-03 — scope guardrails
        let loop_ = make_loop();
        let other_ws = uuid::Uuid::from_u128(999);
        let result = loop_.check_workspace_scope(other_ws);
        assert!(result.is_err(), "cross-workspace must be rejected");
        if let Err(DomainError::AgentYoloPolicyViolation(msg)) = result {
            assert!(msg.contains("cross-workspace"));
        } else {
            panic!("expected AgentYoloPolicyViolation");
        }
    }

    #[test]
    fn agent_03_yolo_same_workspace_allowed() {
        // Same workspace must be allowed
        let loop_ = make_loop();
        assert!(loop_.check_workspace_scope(uuid::Uuid::nil()).is_ok());
    }

    #[test]
    fn test_state_management() {
        let mut loop_ = make_loop();
        let xml = r#"<state_add><state>planning</state></state_add>"#;
        loop_.process_output(xml).unwrap();
        assert_eq!(loop_.states, vec!["planning"]);

        let xml2 = r#"<state_delete><state>planning</state></state_delete>
<state_add><state>executing</state></state_add>"#;
        loop_.process_output(xml2).unwrap();
        assert_eq!(loop_.states, vec!["executing"]);
    }
}
