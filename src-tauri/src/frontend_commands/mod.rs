use crate::common::PROJECTS_DIR;

pub mod capture;
pub mod find;
pub mod fs;
pub mod install;

#[tauri::command]
pub fn projects_dir() -> String {
    PROJECTS_DIR.to_string()
}
