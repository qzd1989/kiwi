use crate::common::PROJECTS_DIR;

pub mod capture;
pub mod find;
pub mod fs;
pub mod install;

#[tauri::command]
pub fn projects_dir() -> String {
    PROJECTS_DIR.to_string()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn init() -> Result<bool, String> {
    Ok(true)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn init() -> Result<bool, String> {
    // TODO:
    //将tesseract.exe临时写入PATH
    use std::env;
    use std::process::Command;

    // fn main() {
    //     // 获取现有的 PATH 环境变量
    //     let current_path = env::var("PATH").unwrap_or_else(|_| String::new());

    //     // 添加 Tesseract 的路径到 PATH 中
    //     let tesseract_path = r"C:\Program Files\Tesseract-OCR";
    //     let new_path = format!("{};{}", current_path, tesseract_path);
    //     env::set_var("PATH", new_path);

    //     // 现在可以直接调用 tesseract
    //     let output = Command::new("tesseract")
    //         .arg("input_image.png")
    //         .arg("output_text")
    //         .output()
    //         .expect("Failed to execute tesseract");

    //     if output.status.success() {
    //         println!("Tesseract executed successfully!");
    //     } else {
    //         eprintln!(
    //             "Tesseract failed with error: {}",
    //             String::from_utf8_lossy(&output.stderr)
    //         );
    //     }
    // }
    Ok(true)
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn has_permission() -> bool {
    true;
}
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn has_permission() -> bool {
    use windows::Win32::UI::Shell::IsUserAnAdmin;
    unsafe { IsUserAnAdmin().as_bool() }
}
