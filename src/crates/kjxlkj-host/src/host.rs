//! Terminal host lifecycle – connects core, renderer, and input.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode as CtKeyCode, KeyModifiers};

use kjxlkj_core::core_loop::CoreProcessor;
use kjxlkj_core_types::{KeyCode, KeyEvent, Modifiers};
use kjxlkj_input::headless::{parse_script, script_step_to_keys, ScriptStep};
use kjxlkj_render::Renderer;

use crate::host_args::HostArgs;
use crate::terminal_setup::{restore_terminal, setup_terminal};

/// The main terminal host tying core, renderer, and event loop.
pub struct Host {
    core: CoreProcessor,
    renderer: Renderer,
    #[allow(dead_code)]
    headless: bool,
}

impl Host {
    /// Create a new host.
    pub fn new(headless: bool) -> Self {
        Self {
            core: CoreProcessor::new(),
            renderer: Renderer::new(80, 24),
            headless,
        }
    }

    /// Run the editor from parsed arguments.
    pub fn run(args: HostArgs) -> Result<()> {
        let mut host = Self::new(args.headless);
        if args.headless {
            return host.run_headless(&args);
        }
        setup_terminal()?;
        let result = host.event_loop(&args);
        restore_terminal()?;
        result
    }

    /// Run in headless mode: load file, execute script, return.
    pub fn run_headless(&mut self, args: &HostArgs) -> Result<()> {
        if let Some(ref path) = args.file {
            self.open_file(path)?;
        }
        if let Some(ref script_path) = args.script {
            let json = std::fs::read_to_string(script_path)?;
            let steps = parse_script(&json)
                .map_err(|e| anyhow::anyhow!("script parse error: {e}"))?;
            for step in &steps {
                let keys = script_step_to_keys(step);
                for key in keys {
                    self.core.process_key(key);
                }
                self.run_assertions(step)?;
            }
        }
        Ok(())
    }

    /// Open a file into the editor.
    pub fn open_file(&mut self, path: &str) -> Result<()> {
        use kjxlkj_core::text::TextBuffer;
        let content = std::fs::read_to_string(path)?;
        let state = self.core.state_mut();
        let buf_id = state.active_buffer_id();
        let mut buf = TextBuffer::from_text(buf_id, path.to_string(), &content);
        buf.set_path(path.to_string());
        state.buffers.insert(buf_id, buf);
        tracing::info!("opened file: {path}");
        Ok(())
    }

    /// Process a single crossterm event; returns false on quit.
    pub fn process_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(ke) => {
                if let Some(key) = map_crossterm_key(ke) {
                    self.core.process_key(key);
                }
            }
            Event::Resize(w, h) => {
                self.core.resize(w, h);
                self.renderer.resize(w, h);
            }
            _ => {}
        }
        !self.core.is_quit()
    }

    fn event_loop(&mut self, args: &HostArgs) -> Result<()> {
        if let Some(ref path) = args.file {
            self.open_file(path)?;
        }
        if let Ok((w, h)) = crossterm::terminal::size() {
            self.core.resize(w, h);
            self.renderer.resize(w, h);
        }
        loop {
            let snap = self.core.snapshot();
            self.renderer
                .render(&snap, &mut std::io::stdout())?;
            let ev = event::read()?;
            if !self.process_event(ev) {
                break;
            }
        }
        Ok(())
    }

    fn run_assertions(&self, step: &ScriptStep) -> Result<()> {
        match step {
            ScriptStep::AssertMode { mode } => {
                let current = self.core.state().mode.current();
                let actual = format!("{current:?}").to_lowercase();
                if actual != mode.to_lowercase() {
                    anyhow::bail!("mode assert: expected {mode}, got {actual}");
                }
            }
            ScriptStep::AssertCursor { line, col } => {
                let cur = self.core.state().active_window().cursor;
                if cur.line != *line || cur.col != *col {
                    anyhow::bail!(
                        "cursor assert: expected ({line},{col}), got ({},{})",
                        cur.line, cur.col
                    );
                }
            }
            ScriptStep::AssertLine { line, content } => {
                let actual = self.core.state().active_buffer()
                    .line(*line)
                    .unwrap_or_default();
                if actual != *content {
                    anyhow::bail!(
                        "line {line} assert: expected '{content}', got '{actual}'"
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn map_crossterm_key(ke: crossterm::event::KeyEvent) -> Option<KeyEvent> {
    let code = match ke.code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::Esc => KeyCode::Escape,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::Insert => KeyCode::Insert,
        CtKeyCode::F(n) => KeyCode::F(n),
        _ => return None,
    };
    let m = ke.modifiers;
    let mut mods = Modifiers::NONE;
    if m.contains(KeyModifiers::CONTROL) { mods = mods.union(Modifiers::CTRL); }
    if m.contains(KeyModifiers::ALT) { mods = mods.union(Modifiers::ALT); }
    if m.contains(KeyModifiers::SHIFT) { mods = mods.union(Modifiers::SHIFT); }
    Some(KeyEvent::new(code, mods))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_new_headless() {
        let h = Host::new(true);
        assert!(h.headless);
        let h2 = Host::new(false);
        assert!(!h2.headless);
    }

    #[test]
    fn open_missing_file() {
        let mut h = Host::new(true);
        assert!(h.open_file("/nonexistent/abc.txt").is_err());
    }

    #[test]
    fn headless_no_script() {
        let mut h = Host::new(true);
        let args = HostArgs { file: None, headless: true, script: None };
        assert!(h.run_headless(&args).is_ok());
    }
}
