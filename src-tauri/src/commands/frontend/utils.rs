use crate::types::Progress;
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter as _};

pub fn emit_progress(app: &AppHandle, name: impl Into<String>, progress: Progress) {
    let event = format!("progress:{}", name.into());
    app.emit(event.as_str(), progress).unwrap();
}

pub fn emit<S>(app: &AppHandle, event: impl Into<String>, payload: S)
where
    S: Serialize + Clone,
{
    app.emit(&event.into(), payload).unwrap();
}

pub fn get_relative_image_data_path_buf() -> PathBuf {
    PathBuf::from("data").join("images")
}
