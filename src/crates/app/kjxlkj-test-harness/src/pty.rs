use anyhow::{anyhow, Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::repo_root;

static BUILD_ONCE: OnceLock<Result<(), String>> = OnceLock::new();

pub fn ensure_kjxlkj_built() -> Result<PathBuf> {
    let build_result = BUILD_ONCE.get_or_init(|| {
        let status = Command::new("cargo")
            .args(["build", "-p", "kjxlkj"])
            .current_dir(repo_root())
            .status();
        let status = match status {
            Ok(status) => status,
            Err(error) => return Err(format!("failed to execute cargo build for kjxlkj: {error}")),
        };
        if status.success() {
            Ok(())
        } else {
            Err(format!("cargo build -p kjxlkj failed: {status}"))
        }
    });
    if let Err(message) = build_result {
        return Err(anyhow!(message.clone()));
    }
    let mut binary_path = repo_root().join("target").join("debug").join("kjxlkj");
    if cfg!(windows) {
        binary_path.set_extension("exe");
    }
    Ok(binary_path)
}

pub struct PtySession {
    child: Box<dyn portable_pty::Child + Send>,
    master: Box<dyn portable_pty::MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    output: Arc<Mutex<Vec<u8>>>,
    _reader_thread: thread::JoinHandle<()>,
}

impl PtySession {
    pub fn spawn(binary: &PathBuf, cols: u16, rows: u16, env: &[(&str, &str)]) -> Result<Self> {
        let pty = native_pty_system();
        let pair = pty
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("failed to open PTY pair")?;
        let mut cmd = CommandBuilder::new(binary);
        for (key, value) in env {
            cmd.env(key, value);
        }
        let child = pair
            .slave
            .spawn_command(cmd)
            .context("failed to spawn child in PTY")?;
        drop(pair.slave);
        let mut reader = pair
            .master
            .try_clone_reader()
            .context("failed to clone PTY reader")?;
        let writer = pair
            .master
            .take_writer()
            .context("failed to open PTY writer")?;
        let master = pair.master;
        let output = Arc::new(Mutex::new(Vec::new()));
        let output_copy = Arc::clone(&output);
        let reader_thread = thread::spawn(move || {
            let mut buffer = [0_u8; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut out) = output_copy.lock() {
                            out.extend_from_slice(&buffer[..n]);
                        } else {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(Self {
            child,
            master,
            writer,
            output,
            _reader_thread: reader_thread,
        })
    }

    pub fn send_raw(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer
            .write_all(bytes)
            .context("failed writing raw bytes to PTY")?;
        self.writer.flush().context("failed flushing PTY writer")
    }

    pub fn send_symbolic_key(&mut self, key: &str) -> Result<()> {
        let bytes = match key {
            "a" => b"a".as_slice(),
            "A" => b"A".as_slice(),
            "Ctrl-w" => b"\x17".as_slice(),
            "Esc" => b"\x1B".as_slice(),
            "Enter" => b"\x0D".as_slice(),
            _ => return Err(anyhow!("unknown symbolic key: {key}")),
        };
        self.send_raw(bytes)
    }

    pub fn wait_for_pattern(&self, pattern: &str, timeout: Duration) -> Result<String> {
        let deadline = Instant::now() + timeout;
        loop {
            let snapshot = self.snapshot();
            if snapshot.contains(pattern) {
                return Ok(snapshot);
            }
            if Instant::now() >= deadline {
                return Err(anyhow!(
                    "pattern '{pattern}' not found before timeout; snapshot:\n{snapshot}"
                ));
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn snapshot(&self) -> String {
        let output = self
            .output
            .lock()
            .expect("output lock should not be poisoned");
        String::from_utf8_lossy(&output).into_owned()
    }

    pub fn capture_frame(&self) -> String {
        self.snapshot()
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("failed resizing PTY")
    }

    pub fn quit(&mut self) -> Result<String> {
        self.send_raw(b"\x1Bq")
            .context("failed sending quit sequence")?;
        self.wait_for_pattern("FINAL", Duration::from_secs(1))
    }

    pub fn terminate(&mut self) -> Result<()> {
        self.child
            .kill()
            .context("failed to kill PTY child process")?;
        self.child
            .wait()
            .context("failed waiting for PTY child process")?;
        Ok(())
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
