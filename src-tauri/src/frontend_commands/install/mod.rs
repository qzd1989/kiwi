use crate::{common::VERSION, utils};
use lazy_static::lazy_static;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

lazy_static! {
    static ref TESSERACT_INSTALL_FILE: String = {
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
    };
    static ref PYTHON_INSTALL_FILE: String = {
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
    };
    static ref WHL_FILE: String = {
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
    };
}
