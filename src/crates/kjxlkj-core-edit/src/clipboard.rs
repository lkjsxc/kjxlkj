//! Clipboard integration via system commands.
//!
//! Detects Wayland (wl-copy/wl-paste), X11 (xclip/xsel),
//! macOS (pbcopy/pbpaste), or falls back to internal.

use std::process::Command;

/// Detected clipboard provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardProvider {
    WlClipboard,
    Xclip,
    Xsel,
    Pbcopy,
    Internal,
}

/// Detect the available clipboard provider.
pub fn detect_provider() -> ClipboardProvider {
    if which("wl-copy") && which("wl-paste") {
        return ClipboardProvider::WlClipboard;
    }
    if which("xclip") {
        return ClipboardProvider::Xclip;
    }
    if which("xsel") {
        return ClipboardProvider::Xsel;
    }
    if which("pbcopy") && which("pbpaste") {
        return ClipboardProvider::Pbcopy;
    }
    ClipboardProvider::Internal
}

/// Copy text to system clipboard.
pub fn clipboard_set(provider: ClipboardProvider, text: &str, primary: bool) -> bool {
    match provider {
        ClipboardProvider::WlClipboard => {
            let mut args = vec!["--type", "text/plain"];
            if primary {
                args.push("--primary");
            }
            run_stdin("wl-copy", &args, text)
        }
        ClipboardProvider::Xclip => {
            let sel = if primary { "primary" } else { "clipboard" };
            run_stdin("xclip", &["-selection", sel], text)
        }
        ClipboardProvider::Xsel => {
            let flag = if primary { "--primary" } else { "--clipboard" };
            run_stdin("xsel", &[flag, "--input"], text)
        }
        ClipboardProvider::Pbcopy => run_stdin("pbcopy", &[], text),
        ClipboardProvider::Internal => false,
    }
}

/// Get text from system clipboard.
pub fn clipboard_get(provider: ClipboardProvider, primary: bool) -> Option<String> {
    match provider {
        ClipboardProvider::WlClipboard => {
            let mut args = vec!["--no-newline"];
            if primary {
                args.push("--primary");
            }
            run_stdout("wl-paste", &args)
        }
        ClipboardProvider::Xclip => {
            let sel = if primary { "primary" } else { "clipboard" };
            run_stdout("xclip", &["-selection", sel, "-o"])
        }
        ClipboardProvider::Xsel => {
            let flag = if primary { "--primary" } else { "--clipboard" };
            run_stdout("xsel", &[flag, "--output"])
        }
        ClipboardProvider::Pbcopy => run_stdout("pbpaste", &[]),
        ClipboardProvider::Internal => None,
    }
}

fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_stdin(cmd: &str, args: &[&str], input: &str) -> bool {
    use std::io::Write;
    let child = Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    match child {
        Ok(mut c) => {
            if let Some(ref mut stdin) = c.stdin {
                let _ = stdin.write_all(input.as_bytes());
            }
            c.wait().map(|s| s.success()).unwrap_or(false)
        }
        Err(_) => false,
    }
}

fn run_stdout(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_provider_returns() {
        let p = detect_provider();
        // Should return something on any platform.
        assert!(matches!(
            p,
            ClipboardProvider::WlClipboard
                | ClipboardProvider::Xclip
                | ClipboardProvider::Xsel
                | ClipboardProvider::Pbcopy
                | ClipboardProvider::Internal
        ));
    }
}
