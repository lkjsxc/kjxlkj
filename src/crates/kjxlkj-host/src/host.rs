//! Terminal host lifecycle – connects core, renderer, and input.

use anyhow::Result;

use kjxlkj_core::core_loop::CoreProcessor;
use kjxlkj_input::headless::{parse_script, script_step_to_keys};
use kjxlkj_render::Renderer;

use crate::host_args::HostArgs;

/// The main terminal host tying core, renderer, and event loop.
pub struct Host {
    pub(crate) core: CoreProcessor,
    pub(crate) renderer: Renderer,
    #[allow(dead_code)]
    pub(crate) headless: bool,
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
        crate::terminal_setup::setup_terminal()?;
        let result = host.event_loop(&args);
        crate::terminal_setup::restore_terminal()?;
        result
    }

    /// Run in headless mode: load file, execute script, return.
    pub fn run_headless(&mut self, args: &HostArgs) -> Result<()> {
        if let Some(ref path) = args.file {
            self.open_file(path)?;
        }
        if let Some(ref script_path) = args.script {
            let json = std::fs::read_to_string(script_path)?;
            let steps =
                parse_script(&json).map_err(|e| anyhow::anyhow!("script parse error: {e}"))?;
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
        let args = HostArgs {
            file: None,
            headless: true,
            script: None,
        };
        assert!(h.run_headless(&args).is_ok());
    }
}
