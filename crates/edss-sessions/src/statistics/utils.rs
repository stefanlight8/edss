pub fn per_hour(value: u64, duration_seconds: f64) -> f64 {
    if duration_seconds == 0.0 {
        return 0.0;
    }

    value as f64 * 3600.0 / duration_seconds
}
