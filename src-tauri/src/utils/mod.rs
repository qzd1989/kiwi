use std::time::{SystemTime, UNIX_EPOCH};
pub mod fs;

pub fn current_time() -> f64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    duration_since_epoch.as_secs() as f64 + duration_since_epoch.subsec_nanos() as f64 * 1e-9
}
