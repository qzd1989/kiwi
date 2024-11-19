#[tauri::command]
pub fn current_dir() -> String {
    _current_dir()
}

#[tauri::command]
pub fn install_python(platform: String, architecture: String) -> Result<String, String> {
    if platform == "macos" && architecture == "aarch64" {
        return Err(format!(
            "{:?}_{:?} not supported yet 1",
            platform, architecture
        ));
    }

    if platform == "windows" && architecture == "x86_64" {
        let base_dir = _current_dir();
        let python_dir = format!("{:?}{:?}", base_dir, r"\python");
        //install python
        let python_install_file = format!("{:?}{:?}", base_dir, r"\python-3.10.11-amd64.exe");
        let python_result = std::process::Command::new(python_install_file)
            .arg("/quiet")
            .arg("/norestart")
            .arg(format!("TargetDir={:?}", python_dir))
            .arg("PrependPath=0")
            .status();
        return match python_result {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install Python".to_string());
                }
                //install pip
                let python_exec_file = format!("{:?}{:?}", python_dir, r"\python.exe");
                let pip_result = std::process::Command::new(python_exec_file.clone())
                    .arg("-m")
                    .arg("ensurepip")
                    .arg("--upgrade")
                    .status();
                return match pip_result {
                    Ok(status) => {
                        if !status.success() {
                            return Err("Failed to install pip".to_string());
                        }
                        let whl_file = format!(
                            "{:?}{:?}",
                            base_dir, r"\kiwi-0.1.0-cp310-abi3-win_amd64.whl"
                        );
                        //install whl
                        let whl_result = std::process::Command::new(python_exec_file)
                            .arg("-m")
                            .arg("pip")
                            .arg("install")
                            .arg(whl_file)
                            .status();
                        return match whl_result {
                            Ok(status) => {
                                if !status.success() {
                                    return Err(
                                        "Failed to install kiwi module for python".to_string()
                                    );
                                }
                                Ok(format!("{:?}_{:?} install success", platform, architecture))
                            }
                            Err(error) => Err(error.to_string()),
                        };
                    }
                    Err(error) => Err(error.to_string()),
                };
            }
            Err(error) => Err(error.to_string()),
        };
    }

    if platform == "windows" && architecture == "aarch64" {
        return Err(format!(
            "{:?}_{:?} not supported yet 2",
            platform, architecture
        ));
    }
    Err(format!(
        "{:?}_{:?} not supported yet 3",
        platform, architecture
    ))
}

pub fn _current_dir() -> String {
    let dir = std::env::current_dir().unwrap();
    dir.to_str().unwrap().to_string()
}
