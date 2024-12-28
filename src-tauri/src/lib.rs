pub mod capture;
pub mod common;
pub mod find;
pub mod grpc;
pub mod input;
pub mod utils;

pub mod frontend_commands;
pub mod python_commands;

use pyo3::prelude::*;

//for frontend
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    //response calling from python or other requests
    grpc::run_spawn();

    let frontend = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init());
    #[cfg(all(windows, debug_assertions))]
    frontend
        .invoke_handler(tauri::generate_handler![
            frontend_commands::home_dir,
            frontend_commands::init,
            frontend_commands::fs::create_dir,
            frontend_commands::fs::create_file,
            frontend_commands::fs::read_dir,
            frontend_commands::fs::rename,
            frontend_commands::fs::remove,
            frontend_commands::fs::exists,
            frontend_commands::fs::write_file,
            frontend_commands::fs::read_file,
            frontend_commands::fs::save_base64_image,
            frontend_commands::capture::snapshot,
            frontend_commands::capture::display_size,
            frontend_commands::find::find_peak,
            frontend_commands::find::find_locating_color,
            frontend_commands::find::find_color,
            frontend_commands::find::find_text,
            frontend_commands::install::initialize_projects,
            frontend_commands::install::install_tesseract,
            frontend_commands::install::uninstall_tesseract,
            frontend_commands::install::install_tessdata,
            frontend_commands::install::install_python,
            frontend_commands::install::uninstall_python,
            frontend_commands::install::repair_python,
            frontend_commands::install::install_pip,
            frontend_commands::install::install_whl,
            frontend_commands::install::is_installed,
            frontend_commands::project::run,
            frontend_commands::project::stop,
            frontend_commands::project::set_project,
            frontend_commands::project::get_project_dir,
            frontend_commands::project::code_check,
            frontend_commands::has_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    #[cfg(not(all(windows, debug_assertions)))]
    frontend
        .invoke_handler(tauri::generate_handler![
            frontend_commands::app::has_permission,
            frontend_commands::app::home_dir,
            frontend_commands::app::init,
            frontend_commands::fs::create_dir,
            frontend_commands::fs::create_file,
            frontend_commands::fs::read_dir,
            frontend_commands::fs::rename,
            frontend_commands::fs::remove,
            frontend_commands::fs::exists,
            frontend_commands::fs::write_file,
            frontend_commands::fs::read_file,
            frontend_commands::fs::save_base64_image,
            frontend_commands::capture::snapshot,
            frontend_commands::capture::display_size,
            frontend_commands::find::find_image,
            frontend_commands::find::find_images,
            frontend_commands::find::find_peak,
            frontend_commands::find::find_locating_color,
            frontend_commands::find::find_color,
            frontend_commands::find::find_text,
            frontend_commands::install::initialize_projects,
            frontend_commands::install::install_tesseract,
            frontend_commands::install::uninstall_tesseract,
            frontend_commands::install::install_tessdata,
            frontend_commands::install::install_python,
            frontend_commands::install::uninstall_python,
            frontend_commands::install::repair_python,
            frontend_commands::install::install_pip,
            frontend_commands::install::install_whl,
            frontend_commands::install::is_installed,
            frontend_commands::project::run,
            frontend_commands::project::stop,
            frontend_commands::project::set_project_dir,
            frontend_commands::project::get_project_dir,
            frontend_commands::project::code_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// for python
#[pymodule]
fn kiwi_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_commands::input_module(m)?;
    python_commands::find_moudle(m)?;
    Ok(())
}
