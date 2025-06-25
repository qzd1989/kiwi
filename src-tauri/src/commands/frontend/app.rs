use super::{error::CommandResult, utils::get_relative_image_data_path_buf};
use crate::{
    app::{App, Config as AppConfig, Log},
    websocket,
};
use anyhow::anyhow;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub async fn is_websocket_alive(port: String) -> CommandResult<bool> {
    let port = port.parse::<u16>().map_err(|error| error.to_string())?;
    websocket::is_alive(port)
        .await
        .map_err(|error| error.into())
}

#[tauri::command]
pub async fn shutdown_websocket() -> CommandResult<()> {
    websocket::shutdown();
    Ok(())
}

#[tauri::command]
pub async fn open_websocket(port: String) -> CommandResult<()> {
    let port = port.parse::<u16>().map_err(|error| error.to_string())?;

    if port == 0 {
        return Err("WebSocket port must be greater than 0.".into());
    }

    let serve_failed_handler = |error: anyhow::Error| {
        Log::error(error.to_string()).send_to_app_msg();
    };

    websocket::serve_in_background(port, Box::new(serve_failed_handler))
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_app_config() -> CommandResult<AppConfig> {
    let config = App::with_config(|config| config.clone());
    Ok(config)
}

#[tauri::command]
pub fn save_app_config(websocket_port: String) -> CommandResult<()> {
    let websocket_port = websocket_port
        .parse::<u16>()
        .map_err(|error| error.to_string())?;

    if websocket_port == 0 {
        return Err("WebSocket port must be greater than 0.".into());
    }

    // let websocket_port = websocket_port.clamp(1, u16::MAX as u32) as u16;
    App::with_config_mut(|config| {
        config.app.websocket_port = websocket_port;
        config.save()
    })?;
    Ok(())
}

#[tauri::command]
pub fn get_relative_image_data_path() -> String {
    get_relative_image_data_path_buf()
        .to_str()
        .unwrap()
        .to_string()
}

#[tauri::command]
pub fn show_windows_from_capture(app_handle: AppHandle, windows: Vec<String>) -> CommandResult<()> {
    crate::capture::Engine::show_windows_from_capture(&app_handle, &windows)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn hide_windows_from_capture(app_handle: AppHandle, windows: Vec<String>) -> CommandResult<()> {
    crate::capture::Engine::hide_windows_from_capture(&app_handle, &windows)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_app_version() -> CommandResult<String> {
    Ok(App::get_version().to_string())
}

#[tauri::command]
pub fn xattr_python() -> CommandResult<()> {
    use crate::interpreter::python::get_default_interpreter;
    use std::process::Command;
    let python_default_interpreter = get_default_interpreter();
    let python_default_interpreter_path_str = python_default_interpreter.to_str().unwrap();
    Command::new("xattr")
        .args(&[
            "-r",
            "-d",
            "com.apple.quarantine",
            python_default_interpreter_path_str,
        ])
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|error| {
            anyhow!("Failed to remove quarantine attribute.({})", error).to_string()
        })?;
    Ok(())
}

#[tauri::command]
pub fn xattr_vscode() -> CommandResult<()> {
    use std::process::Command;
    let resource_dir = App::get_resource_dir();
    let vscode_dir = resource_dir.join("editor").join("vscode").join("Code.app");
    let vscode_path_str = vscode_dir.to_str().unwrap();
    dbg!(vscode_path_str);
    Command::new("xattr")
        .args(&["-r", "-d", "com.apple.quarantine", vscode_path_str])
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|error| {
            anyhow!("Failed to remove quarantine attribute.({})", error).to_string()
        })?;
    dbg!("fuck");
    Ok(())
}

#[tauri::command]
pub fn path_exists(path: String) -> CommandResult<bool> {
    let path = PathBuf::from(path);
    Ok(path.exists())
}
