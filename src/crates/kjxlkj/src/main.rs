//! Binary entrypoint for the kjxlkj editor.

mod cli;


fn main() {
    let args = cli::Cli::parse_args();

    // Install panic handler before anything else.
    kjxlkj_host::install_panic_handler();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    rt.block_on(async move {
        if let Err(e) = run(args).await {
            eprintln!("kjxlkj: {e}");
            std::process::exit(1);
        }
    });
}

async fn run(
    args: cli::Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::sync::{broadcast, mpsc};

    let (quit_tx, _quit_rx) = broadcast::channel::<()>(1);
    let (action_tx, mut action_rx) =
        mpsc::channel::<kjxlkj_core_types::Action>(256);
    let (key_tx, mut _key_rx) =
        mpsc::channel::<kjxlkj_core_types::Key>(256);
    let (response_tx, mut response_rx) =
        mpsc::channel::<kjxlkj_core_types::ServiceResponse>(
            256,
        );

    // Get initial terminal size.
    let (init_cols, init_rows) =
        crossterm::terminal::size().unwrap_or((80, 24));

    // Editor state.
    let mut editor =
        kjxlkj_core::state::EditorState::new(
            init_cols, init_rows,
        );

    // Load files from CLI.
    for file in &args.files {
        if let Ok(content) =
            tokio::fs::read_to_string(file).await
        {
            let buf_id = editor.alloc_buffer_id();
            let buf =
                kjxlkj_core::state::BufferState::from_content(
                    buf_id,
                    file.clone(),
                    &content,
                );
            editor.buffers.insert(buf_id, buf);
        }
    }

    // Terminal host.
    let mut host = kjxlkj_host::TerminalHost::new()?;
    host.enter()?;

    // Signals.
    let signal_handler = kjxlkj_host::SignalHandler::new(
        quit_tx.clone(),
    );
    tokio::spawn(signal_handler.run());

    // SIGWINCH watcher.
    #[cfg(unix)]
    {
        let action_tx_sig = action_tx.clone();
        tokio::spawn(
            kjxlkj_host::watch_sigwinch(action_tx_sig),
        );
    }

    // Services.
    let mut supervisor =
        kjxlkj_services::ServiceSupervisor::new(
            response_tx,
            quit_tx.clone(),
        );
    supervisor.start_all();

    // Input reader.
    let quit_rx_input = quit_tx.subscribe();
    let input_reader = kjxlkj_input::InputReader::new(
        action_tx.clone(),
        key_tx,
    );
    tokio::spawn(async move {
        input_reader.run(quit_rx_input).await;
    });

    // Renderer.
    let mut renderer =
        kjxlkj_render::Renderer::new(init_cols, init_rows);

    // Initial render.
    let snap = editor.snapshot();
    let _ = renderer.render(&snap);

    // Main loop.
    loop {
        tokio::select! {
            Some(action) = action_rx.recv() => {
                use kjxlkj_core_types::Action;
                match action {
                    Action::Resize(cols, rows) => {
                        editor.handle_resize(cols, rows);
                        renderer.resize(cols, rows);
                    }
                    other => {
                        editor.dispatch(other);
                    }
                }
                let snap = editor.snapshot();
                let _ = renderer.render(&snap);

                if editor.should_quit {
                    break;
                }
            }
            Some(_resp) = response_rx.recv() => {
                // Handle service responses.
            }
        }
    }

    // Shutdown.
    supervisor.shutdown().await;
    host.leave()?;

    Ok(())
}
