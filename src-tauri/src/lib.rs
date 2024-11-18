pub mod capture;
pub mod common;
pub mod find;
pub mod frontend_commands;
pub mod input;
pub mod install;
pub mod python_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    install::install();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            frontend_commands::fs::create_dir,
            frontend_commands::fs::create_file,
            frontend_commands::fs::read_dir,
            frontend_commands::fs::rename,
            frontend_commands::fs::remove,
            frontend_commands::fs::exists,
            frontend_commands::fs::write_file,
            frontend_commands::fs::read_file,
            frontend_commands::capture::snapshot,
            frontend_commands::capture::display_size,
            frontend_commands::find::find_peak,
            frontend_commands::find::find_image,
            frontend_commands::find::find_images,
            frontend_commands::find::find_locating_color,
            frontend_commands::find::find_color,
            frontend_commands::find::find_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// for python
use pyo3::prelude::*;
#[pymodule]
fn kiwi_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_commands::input_module(m)?;
    Ok(())
}
