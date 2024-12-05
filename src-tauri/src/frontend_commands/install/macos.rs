use std::path::PathBuf;

#[tauri::command]
pub fn is_installed() -> bool {
    //todo
    true
}

#[tauri::command]
pub fn initialize_projects(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_python(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn uninstall_python(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn repair_python(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_pip(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_whl(architecture: String) -> Result<bool, String> {
    //todo
    if architecture == "aarch64" {
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn uninstall_tesseract(architecture: String) -> Result<bool, String> {
    //todo
    Ok(true)
}
#[tauri::command]
pub fn install_tesseract(architecture: String) -> Result<bool, String> {
    //todo
    Ok(true)
}
#[tauri::command]
pub fn install_tessdata(architecture: String) -> Result<bool, String> {
    //todo
    Ok(true)
}

pub static TESSERACT_DIR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    PathBuf::from(std::env::var("ProgramFiles").unwrap())
        .join("Tesseract-OCR")
        .to_str()
        .unwrap()
        .to_string()
});
