#[tauri::command]
pub fn current_dir() -> String {
    let dir = std::env::current_dir().unwrap();
    dir.to_str().unwrap().to_string()
}
