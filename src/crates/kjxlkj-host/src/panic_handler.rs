//! Panic handler that restores terminal state before printing.

use std::panic;

/// Install a panic handler that restores the terminal before panicking.
pub fn install_panic_handler() {
    let original = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        // Attempt to restore terminal.
        let _ =
            crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen,
        );

        // Write crash info.
        eprintln!("\n=== kjxlkj CRASH ===");
        if let Some(location) = info.location() {
            eprintln!(
                "Panic at {}:{}",
                location.file(),
                location.line()
            );
        }
        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            eprintln!("Message: {msg}");
        } else if let Some(msg) =
            info.payload().downcast_ref::<String>()
        {
            eprintln!("Message: {msg}");
        }
        eprintln!("=== END CRASH ===\n");

        // Call original hook.
        original(info);
    }));
}

#[cfg(test)]
mod tests {
    #[test]
    fn install_handler() {
        // Just verify it doesn't panic when called.
        super::install_panic_handler();
    }
}
