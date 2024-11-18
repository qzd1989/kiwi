use serde::Serialize;
use std::fs;
#[tauri::command]
pub fn create_dir(path: String) -> Result<bool, String> {
    let exists = fs::exists(path.clone());
    if let Err(error) = exists {
        return Err(format!("{}", error));
    }
    if exists.unwrap() {
        return Err(format!("Directory {:?} already exists", path));
    }
    if let Err(error) = fs::create_dir_all(path) {
        return Err(format!("{}", error));
    }
    Ok(true)
}

#[tauri::command]
pub fn create_file(path: String) -> Result<bool, String> {
    let exists = fs::exists(path.clone());
    if let Err(error) = exists {
        return Err(format!("{}", error));
    }
    if exists.unwrap() {
        return Err(format!("File {:?} already exists", path));
    }
    let file = fs::File::create(path);
    if let Err(error) = file {
        return Err(format!("{}", error));
    }
    Ok(true)
}
#[tauri::command]
pub fn rename(from: String, to: String) -> Result<bool, String> {
    if let Err(error) = fs::rename(from, to) {
        return Err(format!("{}", error));
    }
    Ok(true)
}
#[tauri::command]
pub fn write_file(path: String, contents: String) -> Result<bool, String> {
    if let Err(error) = fs::write(path, contents) {
        return Err(format!("{}", error));
    }
    Ok(true)
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    let contents = fs::read_to_string(path);
    if let Err(error) = contents {
        return Err(format!("{}", error));
    }
    Ok(contents.unwrap())
}

#[tauri::command]
pub fn remove(path: String) -> Result<bool, String> {
    let metadata = fs::metadata(path.clone());
    if let Err(error) = metadata {
        return Err(format!("{}", error));
    }
    let metadata = metadata.unwrap();
    if metadata.is_dir() {
        if let Err(error) = fs::remove_dir_all(path) {
            return Err(format!("{}", error));
        }
    } else {
        if let Err(error) = fs::remove_file(path) {
            return Err(format!("{}", error));
        }
    }
    Ok(true)
}
#[tauri::command]
pub fn exists(path: String) -> Result<bool, String> {
    let result = fs::exists(path);
    if let Err(error) = result {
        return Err(format!("{}", error));
    }
    Ok(result.unwrap())
}
#[tauri::command]
pub fn read_dir(path: String) -> Result<Vec<Entry>, String> {
    let mut files = Vec::new();
    let dir = fs::read_dir(path);
    if let Err(error) = dir {
        return Err(format!("{}", error));
    }
    for entry in dir.unwrap() {
        if let Err(error) = entry {
            return Err(format!("{}", error));
        }
        files.push(Entry::new(entry.unwrap()));
    }
    Ok(files)
}

#[derive(Debug, Serialize, Clone)]
pub struct Entry {
    name: String,
    path: String,
    is_dir: bool,
    is_symlink: bool,
    is_file: bool,
}

impl Entry {
    fn new(dir_entry: fs::DirEntry) -> Self {
        let file_type = dir_entry.file_type().unwrap();
        let name = dir_entry.file_name().into_string().unwrap();
        let path = dir_entry.path().to_str().unwrap().to_string();
        let is_dir = file_type.is_dir();
        let is_symlink = file_type.is_symlink();
        let is_file = file_type.is_file();
        Self {
            name,
            path,
            is_dir,
            is_symlink,
            is_file,
        }
    }
}
