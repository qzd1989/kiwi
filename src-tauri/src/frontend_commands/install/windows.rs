use super::{PYTHON_INSTALL_FILE, TESSERACT_INSTALL_FILE, WHL_FILE};
use crate::common::{HOME_DIR, PYTHON_EXEC_FILE};
use crate::utils;
use crate::utils::fs::{current_dir, exists};
use std::path::PathBuf;

#[tauri::command]
pub fn is_installed() -> bool {
    //python exec file
    if !exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
        return false;
    }
    //tesseract folder
    if !exists(TESSERACT_DIR.to_string()).unwrap() {
        return false;
    }
    true
}

#[tauri::command]
pub fn initialize_projects(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let _ = utils::fs::create_dir(HOME_DIR.to_string());
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_python(architecture: String) -> Result<bool, String> {
    println!("install_python");
    if architecture == "x86_64" || architecture == "aarch64" {
        if exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
            return Ok(true);
        }
        let result = std::process::Command::new(PYTHON_INSTALL_FILE.to_string())
            .arg("/quiet")
            .arg("/norestart")
            .arg(format!("TargetDir={}", PYTHON_DIR.to_string()))
            .arg("PrependPath=0")
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    if exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
                        return Ok(true);
                    }
                    return Ok(false);
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
    println!("uninstall_python");
    if architecture == "x86_64" || architecture == "aarch64" {
        if exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
            return Ok(true);
        }
        let result = std::process::Command::new(PYTHON_INSTALL_FILE.to_string())
            .arg("/uninstall")
            .arg("/quiet")
            .arg(format!("TargetDir={}", PYTHON_DIR.to_string()))
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    if exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
                        return Ok(true);
                    }
                    return Ok(false);
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
    println!("repair_python");
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
                    if exists(PYTHON_EXEC_FILE.to_string()).unwrap() {
                        return Ok(true);
                    }
                    return Ok(false);
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
    println!("install_pip");
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
    println!("install_whl");
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

#[tauri::command]
pub fn uninstall_tesseract(architecture: String) -> Result<bool, String> {
    println!("uninstall_tesseract");
    if architecture == "x86_64" || architecture == "aarch64" {
        let installed = exists(TESSERACT_UNINSTALL_FILE.to_string()).unwrap();
        if !installed {
            return Ok(true);
        }
        let result = std::process::Command::new(TESSERACT_UNINSTALL_FILE.to_string())
            .arg("/S")
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
pub fn install_tesseract(architecture: String) -> Result<bool, String> {
    println!("install_tesseract");
    if architecture == "x86_64" || architecture == "aarch64" {
        let result = std::process::Command::new(TESSERACT_INSTALL_FILE.to_string())
            .arg("/S")
            .status();
        return match result {
            Ok(status) => {
                if status.success() {
                    //move tessdata to tesseract
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
pub fn install_tessdata(architecture: String) -> Result<bool, String> {
    if architecture == "x86_64" || architecture == "aarch64" {
        let data_names = vec![
            "chi_sim.traineddata",
            "eng.traineddata",
            "osd.traineddata",
            "snum.traineddata",
        ];
        for data_name in data_names {
            let from = current_dir()
                .join("resources")
                .join("tessdata")
                .join(data_name);
            let to = PathBuf::from(std::env::var("ProgramFiles").unwrap())
                .join("Tesseract-OCR")
                .join("tessdata")
                .join(data_name);
            if let Err(error) = std::fs::copy(from, to) {
                return Err(error.to_string());
            }
        }
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

pub static PYTHON_DIR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    utils::fs::current_dir()
        .join("python")
        .to_str()
        .unwrap()
        .to_string()
});
pub static TESSERACT_UNINSTALL_FILE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    PathBuf::from(std::env::var("ProgramFiles").unwrap())
        .join("Tesseract-OCR")
        .join("tesseract-uninstall.exe")
        .to_str()
        .unwrap()
        .to_string()
});
pub static TESSERACT_DIR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    PathBuf::from(std::env::var("ProgramFiles").unwrap())
        .join("Tesseract-OCR")
        .to_str()
        .unwrap()
        .to_string()
});
