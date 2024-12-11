use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

impl WebsiteStatus {
    pub fn new(url: String, status: Result<u16, String>, response_time: Duration) -> Self {
        Self {
            url,
            status,
            response_time,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_website_status_creation() {
        let url = "https://example.com".to_string();
        let status = Ok(200);
        let response_time = Duration::from_secs(1);

        let report = WebsiteStatus::new(url.clone(), status.clone(), response_time);

        assert_eq!(report.url, url);
        assert_eq!(report.status, status);
        assert_eq!(report.response_time, response_time);
    }
}