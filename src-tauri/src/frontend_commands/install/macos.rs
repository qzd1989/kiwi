#[tauri::command]
pub fn is_installed() -> bool {
    true
}

#[tauri::command]
pub fn lock_install_file() {}

#[tauri::command]
pub fn install_projects(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_python(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn uninstall_python(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn repair_python(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_pip(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_whl(architecture: String) -> Result<bool, String> {
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}
