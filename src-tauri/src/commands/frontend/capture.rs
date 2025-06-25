use super::error::CommandResult;
use crate::{
    app::{App, Log},
    commands::frontend::utils::emit,
    types::Size,
};
use std::thread;
use tauri::AppHandle;
#[tauri::command]
pub fn capture_screenshot(app_handle: AppHandle) -> CommandResult<()> {
    thread::spawn(move || {
        App::with_capturer(|capturer| {
            let frame = match capturer.grab(2000) {
                Ok(f) => f,
                Err(error) => {
                    Log::error(error.to_string()).send_to_app_msg();
                    return;
                }
            };
            match frame.to_base64_png() {
                Ok(base64_data) => emit(&app_handle, "backend:update:frame", base64_data),
                Err(error) => {
                    Log::error(error.to_string()).send_to_app_msg();
                    return;
                }
            }
        });
    });

    Ok(())
}
#[tauri::command]
pub fn get_monitor_size() -> CommandResult<Size> {
    App::with_capturer(|capturer| capturer.get_monitor_size()).map_err(|error| error.into())
}
