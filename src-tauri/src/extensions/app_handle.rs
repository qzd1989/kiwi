use crate::types::Emit;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter as _};

pub trait AppHandleExt {
    fn emit_with_timestamp(&self, target: &str, data: &str);
}

impl AppHandleExt for AppHandle {
    fn emit_with_timestamp(&self, target: &str, data: &str) {
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let time = duration_since_epoch.as_secs() as f64
            + duration_since_epoch.subsec_nanos() as f64 * 1e-9;
        let payload = Emit::new(data.to_string(), time);
        self.emit(target, payload).unwrap();
    }
}
