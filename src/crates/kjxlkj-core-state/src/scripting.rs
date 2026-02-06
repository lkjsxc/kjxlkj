//! Scripting subsystem types: completion providers, user commands, user functions, timers.

use std::collections::HashMap;

/// A named completion source that can produce candidates.
#[derive(Debug, Clone)]
pub struct CompletionProvider { pub name: String, pub kind: CompletionProviderKind }

/// Kinds of completion providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionProviderKind { Buffer, Path, Line, Lsp, Dictionary, Command, User }

/// Registry of completion providers.
#[derive(Debug, Clone, Default)]
pub struct CompletionRegistry { providers: Vec<CompletionProvider> }

impl CompletionRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, p: CompletionProvider) {
        self.providers.retain(|x| x.name != p.name);
        self.providers.push(p);
    }
    pub fn unregister(&mut self, name: &str) -> bool {
        let len = self.providers.len();
        self.providers.retain(|p| p.name != name);
        self.providers.len() < len
    }
    pub fn list(&self) -> &[CompletionProvider] { &self.providers }
    pub fn get(&self, name: &str) -> Option<&CompletionProvider> {
        self.providers.iter().find(|p| p.name == name)
    }
    pub fn len(&self) -> usize { self.providers.len() }
    pub fn is_empty(&self) -> bool { self.providers.is_empty() }
}

/// A user-defined Ex command registered with `:command`.
#[derive(Debug, Clone)]
pub struct UserCommand {
    pub name: String,
    pub replacement: String,
    pub nargs: NArgs,
    pub bang: bool,
    pub range: bool,
    pub buffer_local: bool,
}

/// Argument count spec for user commands (-nargs=).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NArgs { Zero, One, Any, AtLeastOne, Optional }

/// Registry of user-defined commands.
#[derive(Debug, Clone, Default)]
pub struct UserCommandRegistry { cmds: HashMap<String, UserCommand> }

impl UserCommandRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn define(&mut self, cmd: UserCommand) { self.cmds.insert(cmd.name.clone(), cmd); }
    pub fn remove(&mut self, name: &str) -> bool { self.cmds.remove(name).is_some() }
    pub fn get(&self, name: &str) -> Option<&UserCommand> { self.cmds.get(name) }
    pub fn list(&self) -> Vec<&UserCommand> {
        let mut v: Vec<_> = self.cmds.values().collect();
        v.sort_by_key(|c| &c.name);
        v
    }
    pub fn len(&self) -> usize { self.cmds.len() }
    pub fn is_empty(&self) -> bool { self.cmds.is_empty() }
}

/// A user-defined function registered with `:function`.
#[derive(Debug, Clone)]
pub struct UserFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<String>,
    pub is_dict: bool,
    pub is_abort: bool,
}

/// Registry of user-defined functions.
#[derive(Debug, Clone, Default)]
pub struct UserFunctionRegistry { fns: HashMap<String, UserFunction> }

impl UserFunctionRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn define(&mut self, f: UserFunction) { self.fns.insert(f.name.clone(), f); }
    pub fn remove(&mut self, name: &str) -> bool { self.fns.remove(name).is_some() }
    pub fn get(&self, name: &str) -> Option<&UserFunction> { self.fns.get(name) }
    pub fn list(&self) -> Vec<&UserFunction> {
        let mut v: Vec<_> = self.fns.values().collect();
        v.sort_by_key(|f| &f.name);
        v
    }
    pub fn len(&self) -> usize { self.fns.len() }
    pub fn is_empty(&self) -> bool { self.fns.is_empty() }
}

/// A pending timer or debounced action.
#[derive(Debug, Clone)]
pub struct TimerHandle {
    pub id: u64,
    pub delay_ms: u64,
    pub repeat: bool,
    pub command: String,
}

/// Debounce entry for coalescing rapid operations.
#[derive(Debug, Clone)]
pub struct DebouncedAction {
    pub name: String,
    pub delay_ms: u64,
    pub command: String,
    pub pending: bool,
}

/// Simple scheduler tracking pending timers.
#[derive(Debug, Clone, Default)]
pub struct Scheduler {
    timers: Vec<TimerHandle>,
    debounces: HashMap<String, DebouncedAction>,
    next_id: u64,
}

impl Scheduler {
    pub fn new() -> Self { Self::default() }
    pub fn add_timer(&mut self, delay_ms: u64, repeat: bool, command: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.timers.push(TimerHandle { id, delay_ms, repeat, command: command.into() });
        id
    }
    pub fn cancel_timer(&mut self, id: u64) -> bool {
        let len = self.timers.len();
        self.timers.retain(|t| t.id != id);
        self.timers.len() < len
    }
    pub fn set_debounce(&mut self, name: &str, delay_ms: u64, command: &str) {
        self.debounces.insert(name.into(), DebouncedAction {
            name: name.into(), delay_ms, command: command.into(), pending: true,
        });
    }
    pub fn cancel_debounce(&mut self, name: &str) -> bool { self.debounces.remove(name).is_some() }
    pub fn pending_timers(&self) -> &[TimerHandle] { &self.timers }
    pub fn pending_debounces(&self) -> Vec<&DebouncedAction> {
        self.debounces.values().filter(|d| d.pending).collect()
    }
    pub fn timer_count(&self) -> usize { self.timers.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_registry_ops() {
        let mut reg = CompletionRegistry::new();
        reg.register(CompletionProvider { name: "buffer".into(), kind: CompletionProviderKind::Buffer });
        reg.register(CompletionProvider { name: "lsp".into(), kind: CompletionProviderKind::Lsp });
        assert_eq!(reg.len(), 2);
        assert!(reg.get("buffer").is_some());
        reg.unregister("buffer");
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn user_registries_and_scheduler() {
        let mut cmds = UserCommandRegistry::new();
        cmds.define(UserCommand {
            name: "Greet".into(), replacement: "echo 'hi'".into(),
            nargs: NArgs::Zero, bang: false, range: false, buffer_local: false,
        });
        assert_eq!(cmds.get("Greet").unwrap().replacement, "echo 'hi'");
        cmds.remove("Greet");
        assert!(cmds.is_empty());

        let mut fns = UserFunctionRegistry::new();
        fns.define(UserFunction {
            name: "MyFunc".into(), params: vec!["a".into()],
            body: vec!["return a + 1".into()], is_dict: false, is_abort: false,
        });
        assert_eq!(fns.list().len(), 1);

        let mut sched = Scheduler::new();
        let id = sched.add_timer(100, false, ":redraw");
        sched.set_debounce("completion", 150, ":complete");
        assert_eq!(sched.timer_count(), 1);
        assert_eq!(sched.pending_debounces().len(), 1);
        sched.cancel_timer(id);
        sched.cancel_debounce("completion");
        assert_eq!(sched.timer_count(), 0);
    }
}
