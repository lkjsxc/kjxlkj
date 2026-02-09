/// kjxlkj: terminal editor binary entry point.
///
/// Implements the startup sequence from
/// docs/spec/architecture/startup.md:
///   1. Parse CLI arguments
///   2. Install panic handler
///   3. Build Tokio runtime
///   4. Enter async run()
mod app;

use std::env;
use std::panic;

fn main() {
    // Step 1: Parse CLI arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Step 2: Install panic handler that restores terminal
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        // Restore terminal before printing panic
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(
            stdout,
            crossterm::cursor::Show,
            crossterm::event::DisableFocusChange,
            crossterm::event::DisableBracketedPaste,
            crossterm::terminal::LeaveAlternateScreen,
        );
        let _ = crossterm::terminal::disable_raw_mode();
        default_hook(info);
    }));

    // Step 3: Build Tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    // Step 4: Enter async run()
    let exit_code = runtime.block_on(run(args));

    std::process::exit(exit_code);
}

async fn run(files: Vec<String>) -> i32 {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    match app::run_inner(files).await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fatal error: {e}");
            1
        }
    }
}
