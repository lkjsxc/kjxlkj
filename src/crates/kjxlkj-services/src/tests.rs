//! Integration tests for services.

use crate::*;

#[test]
fn supervisor_creation() {
    let (supervisor, _msg_tx, _req_rx) = ServiceSupervisor::new();
    assert!(!supervisor.is_running());
}
