use std::time::Duration;

pub struct Config {
    pub worker_threads: usize,
    pub request_timeout: Duration,
    pub max_retries: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            worker_threads: 4,
            request_timeout: Duration::from_secs(5),
            max_retries: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new();
        assert_eq!(config.worker_threads, 4);
        assert_eq!(config.request_timeout, std::time::Duration::from_secs(5));
        assert_eq!(config.max_retries, 3);
    }
}