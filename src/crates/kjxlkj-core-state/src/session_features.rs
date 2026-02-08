//! Detailed session features: macros persistence,
//! register persistence, expression register eval,
//! and ex command batch processing.

use std::collections::HashMap;

/// Macro recording entry for session persistence.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MacroEntry {
    /// Register name character.
    pub register: char,
    /// Recorded key sequence as string representation.
    pub keys: String,
}

/// Macro persistence state.
#[derive(Debug, Clone, Default)]
pub struct MacroPersistence {
    /// Saved macros for session save/restore.
    pub saved_macros: Vec<MacroEntry>,
    /// Whether to persist macros across sessions.
    pub persist: bool,
}

impl MacroPersistence {
    pub fn new() -> Self {
        Self {
            saved_macros: Vec::new(),
            persist: true,
        }
    }

    /// Save a macro for persistence.
    pub fn save_macro(&mut self, register: char, keys: &str) {
        self.saved_macros
            .retain(|m| m.register != register);
        self.saved_macros.push(MacroEntry {
            register,
            keys: keys.to_string(),
        });
    }

    /// Get a saved macro.
    pub fn get_macro(&self, register: char) -> Option<&str> {
        self.saved_macros
            .iter()
            .find(|m| m.register == register)
            .map(|m| m.keys.as_str())
    }
}

/// Register persistence for session save/restore.
#[derive(Debug, Clone, Default)]
pub struct RegisterPersistence {
    /// Named register contents (a-z).
    pub named: HashMap<char, String>,
    /// Whether to persist registers across sessions.
    pub persist: bool,
}

impl RegisterPersistence {
    pub fn new() -> Self {
        Self {
            named: HashMap::new(),
            persist: true,
        }
    }

    /// Save a register value.
    pub fn save_register(&mut self, name: char, content: &str) {
        self.named.insert(name, content.to_string());
    }

    /// Get a saved register value.
    pub fn get_register(&self, name: char) -> Option<&str> {
        self.named.get(&name).map(|s| s.as_str())
    }
}

/// Simple expression evaluator for `=` register.
#[derive(Debug, Clone, Default)]
pub struct ExpressionEval {
    /// Last evaluated expression.
    pub last_expr: Option<String>,
    /// Last result.
    pub last_result: Option<String>,
    /// Variable bindings for expressions.
    pub variables: HashMap<String, String>,
}

impl ExpressionEval {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluate a simple arithmetic or string expression.
    pub fn eval(&mut self, expr: &str) -> String {
        self.last_expr = Some(expr.to_string());
        // Simple integer arithmetic
        let result = if let Some(val) = self.try_arithmetic(expr) {
            val.to_string()
        } else if let Some(val) = self.variables.get(expr) {
            val.clone()
        } else {
            expr.to_string()
        };
        self.last_result = Some(result.clone());
        result
    }

    fn try_arithmetic(&self, expr: &str) -> Option<i64> {
        let expr = expr.trim();
        if let Some((left, right)) = expr.split_once('+') {
            let l = left.trim().parse::<i64>().ok()?;
            let r = right.trim().parse::<i64>().ok()?;
            return Some(l + r);
        }
        if let Some((left, right)) = expr.split_once('-') {
            if !left.is_empty() {
                let l = left.trim().parse::<i64>().ok()?;
                let r = right.trim().parse::<i64>().ok()?;
                return Some(l - r);
            }
        }
        if let Some((left, right)) = expr.split_once('*') {
            let l = left.trim().parse::<i64>().ok()?;
            let r = right.trim().parse::<i64>().ok()?;
            return Some(l * r);
        }
        expr.parse::<i64>().ok()
    }

    /// Set a variable.
    pub fn set_var(&mut self, name: &str, value: &str) {
        self.variables
            .insert(name.to_string(), value.to_string());
    }
}

/// Ex command batch processor for `:source` and scripts.
#[derive(Debug, Clone, Default)]
pub struct ExCommandBatch {
    /// Queue of commands to execute.
    pub queue: Vec<String>,
    /// Current index in the queue.
    pub index: usize,
}

impl ExCommandBatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load commands from script text (one per line).
    pub fn load_script(&mut self, script: &str) {
        self.queue = script
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter(|l| !l.trim_start().starts_with('"'))
            .map(|l| l.to_string())
            .collect();
        self.index = 0;
    }

    /// Get the next command to execute.
    pub fn next_command(&mut self) -> Option<String> {
        if self.index < self.queue.len() {
            let cmd = self.queue[self.index].clone();
            self.index += 1;
            Some(cmd)
        } else {
            None
        }
    }

    /// Whether there are more commands.
    pub fn has_more(&self) -> bool {
        self.index < self.queue.len()
    }

    /// Reset the batch.
    pub fn reset(&mut self) {
        self.queue.clear();
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_persistence() {
        let mut mp = MacroPersistence::new();
        mp.save_macro('a', "dd");
        assert_eq!(mp.get_macro('a'), Some("dd"));
        mp.save_macro('a', "yy");
        assert_eq!(mp.get_macro('a'), Some("yy"));
    }

    #[test]
    fn expression_eval_arithmetic() {
        let mut eval = ExpressionEval::new();
        assert_eq!(eval.eval("3+5"), "8");
        assert_eq!(eval.eval("10*2"), "20");
        assert_eq!(eval.eval("42"), "42");
    }

    #[test]
    fn ex_command_batch() {
        let mut batch = ExCommandBatch::new();
        batch.load_script("set number\n\" comment\nset wrap\n");
        assert!(batch.has_more());
        assert_eq!(batch.next_command(), Some("set number".into()));
        assert_eq!(batch.next_command(), Some("set wrap".into()));
        assert!(!batch.has_more());
    }
}
