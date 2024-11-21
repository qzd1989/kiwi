use std::{fs, path::PathBuf};
pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn exists(path: String) -> Result<bool, String> {
    let result = fs::exists(path);
    if let Err(error) = result {
        return Err(format!("{}", error));
    }
    Ok(result.unwrap())
}

pub fn write_file(path: String, contents: String) -> Result<bool, String> {
    if let Err(error) = fs::write(path, contents) {
        return Err(format!("{}", error));
    }
    Ok(true)
}

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
