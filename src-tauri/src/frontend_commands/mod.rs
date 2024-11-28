use crate::{common::PROJECTS_DIR, utils::current_time};

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

#[tauri::command]
pub fn projects_dir() -> String {
    PROJECTS_DIR.to_string()
}

#[tauri::command]
pub fn has_permission() -> bool {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Shell::IsUserAnAdmin;
        unsafe { IsUserAnAdmin().as_bool() }
    }
    #[cfg(target_os = "macos")]
    {
        true
    }
}

#[tauri::command]
pub fn init() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        //add tesseract to PATH
        let current_path = std::env::var("PATH").unwrap_or_else(|_| String::new());
        let new_path = format!("{};{:?}", current_path, TESSERACT_DIR.to_string());
        std::env::set_var("PATH", new_path);
        //allow python can output chinese
        std::env::set_var("PYTHONIOENCODING", "utf-8");
        Ok(true)
    }
    #[cfg(target_os = "macos")]
    {
        Ok(true)
    }
}

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
