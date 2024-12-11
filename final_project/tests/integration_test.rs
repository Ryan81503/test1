use std::process::{Command, Output};
use std::time::Duration;
use std::thread::sleep;

#[test]
fn test_project_execution() {
    let output: Output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--test-mode")
        .output()
        .expect("Failed to execute the project");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("https://example.com"));
    assert!(stdout.contains("https://rust-lang.org"));
}

#[test]
fn test_concurrent_execution() {
    let output: Output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--test-mode")
        .output()
        .expect("Failed to run the project concurrently");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("https://example.com"));
}