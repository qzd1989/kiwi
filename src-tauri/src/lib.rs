pub mod capture;
pub mod commands;
pub mod common;
pub mod find;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
            greet,
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
            commands::capture::get_peak_point,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
