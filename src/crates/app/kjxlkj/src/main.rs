//! kjxlkj - Vim-compatible terminal text editor.
//!
//! Entry point per /docs/spec/architecture/startup.md:
//! 1. Parse CLI arguments
//! 2. Install panic handler
//! 3. Build Tokio runtime
//! 4. Enter async run()

mod channels;
mod core_task;
mod input_task;
mod render_task;
mod run;
mod signal;

use std::panic;

fn main() {
    // Step 1: Parse CLI arguments.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Step 2: Install panic handler that restores terminal.
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = kjxlkj_host::restore_terminal();
        default_panic(info);
    }));

    // Step 3: Build Tokio runtime.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    // Step 4: Enter async run().
    let result = runtime.block_on(run::run(args));
    if let Err(e) = result {
        eprintln!("kjxlkj: {}", e);
        std::process::exit(1);
    }
}
