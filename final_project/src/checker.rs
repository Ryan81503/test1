use crate::report::WebsiteStatus;
use std::sync::mpsc::Sender;
use std::time::Instant;
use ureq::{Error, Response};
use std::time::Duration;

#[derive(Debug)]
pub enum CheckerError {
    RequestFailed(String),
    SenderError(String),
}

impl std::fmt::Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckerError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            CheckerError::SenderError(msg) => write!(f, "Channel send error: {}", msg),
        }
    }
}

impl std::error::Error for CheckerError {}

pub fn start_monitoring(
    urls: Vec<String>,
    sender: Sender<Result<u16, CheckerError>>,
    timeout: Duration,
) {
    for url in urls {
        let result = ureq::get(&url).timeout(timeout).call();

        let status = match result {
            Ok(res) if res.status() >= 400 => Err(CheckerError::RequestFailed(format!(
                "Server returned status: {}",
                res.status()
            ))),
            Ok(res) => Ok(res.status()),

            Err(Error::Transport(transport_err)) => Err(CheckerError::RequestFailed(format!(
                "Transport error: {}",
                transport_err
            ))),
            Err(Error::Status(code, _)) => Err(CheckerError::RequestFailed(format!(
                "Server error - code: {}",
                code
            ))),
        };

        if sender.send(status).is_err() {
            eprintln!("Failed to send status for URL: {}", url);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_successful_response() {
        let (sender, receiver) = mpsc::channel();
        let url = "https://httpbin.org/status/200".to_string();

        start_monitoring(vec![url.clone()], sender, std::time::Duration::from_secs(5));

        let status = receiver.recv().unwrap();
        assert_eq!(status.unwrap(), 200);
    }

    #[test]
    fn test_failed_response() {
        let (sender, receiver) = mpsc::channel();
        let url = "http://nonexistent.example.invalid".to_string();  // More guaranteed to fail

        start_monitoring(vec![url.clone()], sender, std::time::Duration::from_secs(2));

        let status = receiver.recv().unwrap();
        assert!(
            status.is_err(),
            "Expected error, but got success with: {:?}",
            status.unwrap()
        );
    }

    #[test]
    fn test_request_failure() {
        let (sender, receiver) = mpsc::channel();
        let url = "http://nonexistent.example.invalid".to_string();  // Use guaranteed failure URL

        start_monitoring(vec![url.clone()], sender, std::time::Duration::from_secs(2));

        let result = receiver.recv().unwrap();
        assert!(
            result.is_err(),
            "Expected error, but got success with: {:?}",
            result.unwrap()
        );
    }

    #[test]
    fn test_channel_error_handling() {
        let (sender, _receiver) = mpsc::channel();
        let url = "https://example.com".to_string();

        drop(sender.clone());

        start_monitoring(vec![url.clone()], sender, std::time::Duration::from_secs(2));
    }
}