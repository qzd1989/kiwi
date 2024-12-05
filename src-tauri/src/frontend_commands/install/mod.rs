use crate::utils;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

pub static VERSION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| "1.0.0".to_string());
pub static TESSERACT_INSTALL_FILE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "macos")]
    {
        todo!()
    }
    #[cfg(target_os = "windows")]
    {
        utils::fs::current_dir()
            .join("resources")
            .join("tesseract-windows.exe")
            .to_str()
            .unwrap()
            .to_string()
    }
});
pub static PYTHON_INSTALL_FILE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "macos")]
    {
        #[cfg(debug_assertions)]
        {
            utils::fs::current_dir()
                .join("resources")
                .join("python-macos.pkg")
                .to_str()
                .unwrap()
                .to_string()
        }
        #[cfg(not(debug_assertions))]
        {
            "/Applications/kiwi.app/Contents/Resources/resources/python-macos.pkg".to_string()
        }
    }
    #[cfg(target_os = "windows")]
    {
        utils::fs::current_dir()
            .join("resources")
            .join("python-windows.exe")
            .to_str()
            .unwrap()
            .to_string()
    }
});
pub static WHL_FILE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "macos")]
    {
        #[cfg(debug_assertions)]
        {
            utils::fs::current_dir()
                .join("resources")
                .join(format!(
                    "kiwi-{}-cp310-abi3-macosx_11_0_arm64.whl",
                    *VERSION
                ))
                .to_str()
                .unwrap()
                .to_string()
        }
        #[cfg(not(debug_assertions))]
        {
            format!("/Applications/kiwi.app/Contents/Resources/resources/kiwi-{}-cp310-abi3-macosx_11_0_arm64.whl", *VERSION)
        }
    }
    #[cfg(target_os = "windows")]
    {
        utils::fs::current_dir()
            .join("resources")
            .join(format!("kiwi-{}-cp310-abi3-win_amd64.whl", *VERSION))
            .to_str()
            .unwrap()
            .to_string()
    }
});
