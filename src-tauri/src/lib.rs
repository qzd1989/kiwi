pub mod capture;
pub mod commands;
pub mod common;
pub mod find;
pub mod input;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::fs::create_dir,
            commands::fs::create_file,
            commands::fs::read_dir,
            commands::fs::rename,
            commands::fs::remove,
            commands::fs::exists,
            commands::fs::write_file,
            commands::fs::read_file,
            commands::capture::snapshot,
            commands::capture::display_size,
            commands::find::find_peak,
            commands::find::find_image,
            commands::find::find_images,
            commands::find::find_locating_color,
            commands::find::find_color,
            commands::find::find_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use pyo3::prelude::*;

// #[pyfunction]
// fn guess_the_number() -> PyResult<i32> {
//     println!("Guess the number!");
//     Ok(5)
// }

// #[pymodule]
// fn guessing_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
//     Ok(())
// }
