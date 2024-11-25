use crate::common::PROJECTS_DIR;

pub mod capture;
pub mod find;
pub mod fs;
pub mod install;

use install::TESSERACT_DIR;

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
        Ok(true)
    }
    #[cfg(target_os = "macos")]
    {
        Ok(true)
    }
}
