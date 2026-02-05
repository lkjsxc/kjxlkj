//! kjxlkj - Neovim-inspired TUI text editor.

use anyhow::Result;
use kjxlkj_core::state::Editor;
use kjxlkj_core::types::InputEvent;
use kjxlkj_host::TerminalHost;
use kjxlkj_input::poll_event;
use kjxlkj_render::Renderer;
use kjxlkj_services::ServiceSupervisor;
use std::env;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let args: Vec<String> = env::args().collect();

    let mut host = TerminalHost::new();
    host.enter()?;

    let result = run_editor(&args[1..]);

    host.leave()?;

    result
}

fn run_editor(args: &[String]) -> Result<()> {
    let mut editor = Editor::new();
    let mut renderer = Renderer::new();
    let mut _services = ServiceSupervisor::new();

    if let Some(path) = args.first() {
        editor.apply_intent(kjxlkj_core::types::Intent::OpenFile(path.clone()));
    }

    let (cols, rows) = TerminalHost::size()?;
    editor.handle_input(InputEvent::Resize { cols, rows });

    loop {
        let snapshot = editor.snapshot();
        renderer.render(&snapshot)?;

        if let Some(event) = poll_event(Duration::from_millis(100)) {
            editor.handle_input(event);
        }
    }
}
