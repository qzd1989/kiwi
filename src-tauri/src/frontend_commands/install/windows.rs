use super::{PYTHON_INSTALL_FILE, WHL_FILE};
use crate::common::{PROJECTS_DIR, PYTHON_EXEC_FILE};
use crate::utils;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOCK_FILE: String = utils::fs::current_dir()
        .join(".installed")
        .to_str()
        .unwrap()
        .to_string();
    static ref PYTHON_DIR: String = utils::fs::current_dir()
        .join("python")
        .to_str()
        .unwrap()
        .to_string();
}

#[tauri::command]
pub fn is_installed() -> bool {
    utils::fs::exists(LOCK_FILE.to_string()).unwrap()
}

#[tauri::command]
pub fn lock_install_file() {
    utils::fs::write_file(LOCK_FILE.to_string(), "".to_string()).unwrap();
}

#[tauri::command]
pub fn install_projects(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        if let Err(error) = utils::fs::create_dir(PROJECTS_DIR.to_string()) {
            return Err(error.to_string());
        }
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_python(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(PYTHON_INSTALL_FILE.to_string())
            .arg("/quiet")
            .arg("/norestart")
            .arg(format!("TargetDir={}", PYTHON_DIR.to_string()))
            .arg("PrependPath=0")
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn uninstall_python(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(PYTHON_INSTALL_FILE.to_string())
            .arg("/uninstall")
            .arg("/quiet")
            .arg(format!("TargetDir={}", PYTHON_DIR.to_string()))
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn repair_python(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(PYTHON_INSTALL_FILE.to_string())
            .arg("/quiet")
            .arg("/repair")
            .arg("/norestart")
            .arg(format!("TargetDir={}", PYTHON_DIR.to_string()))
            .arg("PrependPath=0")
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_pip(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(PYTHON_EXEC_FILE.to_string())
            .arg("-m")
            .arg("ensurepip")
            .arg("--upgrade")
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_whl(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(PYTHON_EXEC_FILE.to_string())
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(WHL_FILE.to_string())
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}