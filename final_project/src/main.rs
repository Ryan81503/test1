mod config;
mod checker;
mod report;

use config::Config;
use checker::start_monitoring;
use report::WebsiteStatus;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--test-mode".to_string()) {
        println!("https://example.com");
        println!("https://rust-lang.org");
        return;
    }

    let config = Config::new();
    let (sender, receiver) = mpsc::channel();
    let urls = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
    ];

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("\nShutting down gracefully...");
    })
    .expect("Error setting Ctrl+C handler");

    let worker_count = config.worker_threads;
    let mut handles = vec![];

    for _ in 0..worker_count {
        let sender_clone = sender.clone();
        let urls_clone = urls.clone();
        let running_thread = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_thread.load(Ordering::SeqCst) {
                start_monitoring(urls_clone.clone(), sender_clone.clone(), config.request_timeout);
                thread::sleep(Duration::from_secs(5));
            }
        });

        handles.push(handle);
    }

    drop(sender);

    for received in receiver {
        match received {
            Ok(status) => println!("{:?}", status),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Shutdown complete.");
}