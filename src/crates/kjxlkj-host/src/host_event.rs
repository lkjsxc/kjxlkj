//! Host event processing and crossterm key mapping.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode as CtKeyCode, KeyModifiers};

use kjxlkj_core_types::{KeyCode, KeyEvent, Modifiers};
use kjxlkj_input::headless::ScriptStep;

use crate::host::Host;
use crate::host_args::HostArgs;

impl Host {
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

    pub(crate) fn event_loop(&mut self, args: &HostArgs) -> Result<()> {
        if let Some(ref path) = args.file {
            self.open_file(path)?;
        }
        if let Ok((w, h)) = crossterm::terminal::size() {
            self.core.resize(w, h);
            self.renderer.resize(w, h);
        }
        loop {
            let snap = self.core.snapshot();
            self.renderer.render(&snap, &mut std::io::stdout())?;
            let ev = event::read()?;
            if !self.process_event(ev) {
                break;
            }
        }
        Ok(())
    }

    pub(crate) fn run_assertions(&self, step: &ScriptStep) -> Result<()> {
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
                        cur.line,
                        cur.col
                    );
                }
            }
            ScriptStep::AssertLine { line, content } => {
                let actual = self
                    .core
                    .state()
                    .active_buffer()
                    .line(*line)
                    .unwrap_or_default();
                if actual != *content {
                    anyhow::bail!("line {line} assert: expected '{content}', got '{actual}'");
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
    if m.contains(KeyModifiers::CONTROL) {
        mods = mods.union(Modifiers::CTRL);
    }
    if m.contains(KeyModifiers::ALT) {
        mods = mods.union(Modifiers::ALT);
    }
    if m.contains(KeyModifiers::SHIFT) {
        mods = mods.union(Modifiers::SHIFT);
    }
    Some(KeyEvent::new(code, mods))
}
