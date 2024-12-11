use std::process::{Command, Output};
use std::time::Duration;
use std::thread;

#[test]
fn test_graceful_shutdown() {
    let mut child = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("Failed to start the application");

    thread::sleep(Duration::from_secs(3));
    child.kill().expect("Failed to stop the process");

    let status = child.wait().expect("Failed to wait for process");
    assert!(!status.success(), "Process should be terminated gracefully");
}