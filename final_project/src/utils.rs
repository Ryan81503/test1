pub fn format_duration(duration: std::time::Duration) -> String {
    format!("{} ms", duration.as_millis())
}