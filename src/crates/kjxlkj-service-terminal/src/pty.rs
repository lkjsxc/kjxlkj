//! PTY spawning and management.

use std::os::unix::io::{AsRawFd, OwnedFd, RawFd};

use nix::pty::{openpty, OpenptyResult};
use nix::unistd::{close, dup2, execvp, fork, setsid, ForkResult};

/// PTY handle for a spawned process.
pub struct Pty {
    /// Master file descriptor.
    pub master_fd: OwnedFd,
    /// Child process ID.
    pub child_pid: nix::unistd::Pid,
}

impl Pty {
    /// Spawn a new PTY process.
    pub fn spawn(
        command: &str,
        cols: u16,
        rows: u16,
    ) -> Result<Self, String> {
        let OpenptyResult { master, slave } =
            openpty(None, None)
                .map_err(|e| format!("openpty: {e}"))?;

        // Set initial size.
        let winsize = nix::pty::Winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        unsafe {
            nix::libc::ioctl(
                slave.as_raw_fd(),
                nix::libc::TIOCSWINSZ,
                &winsize as *const _,
            );
        }

        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                // Child process.
                let _ = setsid();
                let slave_raw = slave.as_raw_fd();
                let _ = dup2(slave_raw, 0);
                let _ = dup2(slave_raw, 1);
                let _ = dup2(slave_raw, 2);

                // Close master/slave in child.
                drop(master);
                drop(slave);

                // Exec the command.
                let shell =
                    std::env::var("SHELL")
                        .unwrap_or_else(|_| {
                            String::from("/bin/sh")
                        });
                let c_shell =
                    std::ffi::CString::new(shell).unwrap();
                let c_flag =
                    std::ffi::CString::new("-c").unwrap();
                let c_cmd =
                    std::ffi::CString::new(command).unwrap();
                let _ = execvp(
                    &c_shell,
                    &[&c_shell, &c_flag, &c_cmd],
                );
                std::process::exit(1);
            }
            Ok(ForkResult::Parent { child }) => {
                // Parent: close slave fd.
                drop(slave);
                Ok(Pty {
                    master_fd: master,
                    child_pid: child,
                })
            }
            Err(e) => Err(format!("fork: {e}")),
        }
    }

    /// Resize the PTY.
    pub fn resize(
        &self,
        cols: u16,
        rows: u16,
    ) -> Result<(), String> {
        let winsize = nix::pty::Winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        unsafe {
            let ret = nix::libc::ioctl(
                self.master_fd.as_raw_fd(),
                nix::libc::TIOCSWINSZ,
                &winsize as *const _,
            );
            if ret < 0 {
                return Err("ioctl TIOCSWINSZ failed".into());
            }
        }
        // Signal the child.
        let _ = nix::sys::signal::kill(
            self.child_pid,
            nix::sys::signal::Signal::SIGWINCH,
        );
        Ok(())
    }

    /// Get the master fd for reading/writing.
    pub fn master_raw_fd(&self) -> RawFd {
        self.master_fd.as_raw_fd()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pty_types_exist() {
        // Compile-only test.
        use super::Pty;
        let _ = std::mem::size_of::<Pty>();
    }
}
