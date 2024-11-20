use crate::utils;
use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: String = String::from("1.0.0");
    static ref BASE_DIR: String = utils::fs::current_dir();
    static ref WINDOWS_PROJECTS_DIR: String = format!("{}{}", *BASE_DIR, "\\projects");
    static ref WINDOWS_PYTHON_DIR: String = format!("{}{}", *BASE_DIR, "\\python");
    static ref WINDOWS_INSTALL_FILE: String =
        format!("{}{}", *BASE_DIR, "\\resources\\python-windows-amd64.exe");
    static ref WINDOWS_EXEC_FILE: String = format!("{}{}", *WINDOWS_PYTHON_DIR, "\\python.exe");
    static ref WINDOWS_WHL_FILE: String = format!(
        "{}\\resources\\kiwi-{}-cp310-abi3-win_amd64.whl",
        *BASE_DIR, *VERSION
    );
}

#[tauri::command]
pub fn is_installed() -> bool {
    let lock_file = format!("{}{}", *BASE_DIR, "\\.installed");
    utils::fs::exists(lock_file).unwrap()
}

#[tauri::command]
pub fn lock_install_file() {
    utils::fs::write_file(format!("{}{}", *BASE_DIR, "\\.installed"), "".to_string()).unwrap();
}

#[tauri::command]
pub fn install_projects(platform: String, architecture: String) -> Result<bool, String> {
    println!("install_projects");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        if let Err(error) = utils::fs::create_dir(WINDOWS_PROJECTS_DIR.to_string()) {
            return Err(error.to_string());
        }
        return Ok(true);
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_python(platform: String, architecture: String) -> Result<bool, String> {
    println!("install_python");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        println!("python is installing...");
        println!("WINDOWS_INSTALL_FILE: {}", WINDOWS_INSTALL_FILE.to_string());
        println!("WINDOWS_PYTHON_DIR: {}", WINDOWS_PYTHON_DIR.to_string());
        let result = std::process::Command::new(WINDOWS_INSTALL_FILE.to_string())
            .arg("/quiet")
            .arg("/norestart")
            .arg(format!("TargetDir={}", WINDOWS_PYTHON_DIR.to_string()))
            .arg("PrependPath=0")
            .status();
        return match result {
            Ok(status) => {
                println!("install_python status is {:?}", status.success());
                if status.success() {
                    return Ok(true);
                }
                println!("failed to install_python");
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn uninstall_python(platform: String, architecture: String) -> Result<bool, String> {
    println!("uninstall_python");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        let result = std::process::Command::new(WINDOWS_INSTALL_FILE.to_string())
            .arg("/uninstall")
            .arg("/quiet")
            .arg(format!("TargetDir={}", WINDOWS_PYTHON_DIR.to_string()))
            .status();
        return match result {
            Ok(status) => {
                println!("uninstall_python status is {:?}", status.success());
                if status.success() {
                    return Ok(true);
                }
                println!("failed to uninstall_python");
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn repair_python(platform: String, architecture: String) -> Result<bool, String> {
    println!("repair_python");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        let result = std::process::Command::new(WINDOWS_INSTALL_FILE.to_string())
            .arg("/quiet")
            .arg("/repair")
            .arg("/norestart")
            .arg(format!("TargetDir={}", WINDOWS_PYTHON_DIR.to_string()))
            .arg("PrependPath=0")
            .status();
        return match result {
            Ok(status) => {
                println!("repair_python status is {:?}", status.success());
                if status.success() {
                    return Ok(true);
                }
                println!("failed to repair_python");
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_pip(platform: String, architecture: String) -> Result<bool, String> {
    println!("install_pip");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        let result = std::process::Command::new(WINDOWS_EXEC_FILE.to_string())
            .arg("-m")
            .arg("ensurepip")
            .arg("--upgrade")
            .status();
        return match result {
            Ok(status) => {
                println!("install_pip status is {:?}", status.success());
                if status.success() {
                    return Ok(true);
                }
                println!("failed to install_pip");
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}

#[tauri::command]
pub fn install_whl(platform: String, architecture: String) -> Result<bool, String> {
    println!("install_whl");
    if platform == "windows" && (architecture == "x86_64" || architecture == "aarch64") {
        let result = std::process::Command::new(WINDOWS_EXEC_FILE.to_string())
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(WINDOWS_WHL_FILE.to_string())
            .status();
        return match result {
            Ok(status) => {
                println!("install_whl status is {:?}", status.success());
                if status.success() {
                    return Ok(true);
                }
                println!("failed to install_whl");
                Ok(false)
            }
            Err(error) => Err(error.to_string()),
        };
    }
    Err("Not supported yet".to_string())
}
