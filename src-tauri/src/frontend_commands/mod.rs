use crate::utils::current_time;

pub mod app;
pub mod capture;
pub mod find;
pub mod fs;
pub mod install;
pub mod project;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "windows")]
use install::TESSERACT_DIR;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

trait AppHandleExt {
    fn emit_with_timestamp(&self, target: &str, data: &str);
}

impl AppHandleExt for AppHandle {
    fn emit_with_timestamp(&self, target: &str, data: &str) {
        let time = current_time();
        let payload = Emit::new(data.to_string(), time);
        self.emit(target, payload).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Emit {
    data: String,
    time: f64,
}
impl Emit {
    fn new(data: String, time: f64) -> Self {
        Self { data, time }
    }
}

pub static PYTHON_VERSION: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| String::from("3.10"));
pub static PYTHON_EXEC_FILE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "macos")]
    {
        format!("/opt/homebrew/bin/python{}", *PYTHON_VERSION)
    }
    #[cfg(target_os = "windows")]
    {
        utils::fs::current_dir()
            .join("python")
            .join("pythonw.exe")
            .to_str()
            .unwrap()
            .to_string()
    }
});
