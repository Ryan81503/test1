use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use ureq::{Error, Response};

const URLS: [&str; 5] = [
    "https://www.google.com",
    "https://www.rust-lang.org",
    "https://www.github.com",
    "https://www.microsoft.com",
    "https://www.apple.com",
];

#[test]
fn test_high_concurrency() {
    let (tx, rx) = mpsc::channel();
    let start_time = Instant::now();

    for &url in &URLS {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let response = ureq::get(url).call();
            match response {
                Ok(response) => {
                    let status = response.status();
                    tx_clone.send((url, status)).expect("Failed to send result");
                }
                Err(Error::Transport(_)) => {
                    tx_clone.send((url, 0)).expect("Failed to send result");
                }
                _ => {
                    tx_clone.send((url, 500)).expect("Failed to send result");
                }
            }
        });
    }

    for _ in &URLS {
        let (url, status) = rx.recv().expect("Failed to receive result");
        println!("URL: {}, Status: {}", url, status);
    }

    let elapsed_time = start_time.elapsed();
    println!("Total elapsed time: {:.2?}", elapsed_time);

    assert!(elapsed_time < Duration::from_secs(30), "Test took too long!");
}