use std::path::PathBuf;

use kiwi_lib::utils::common::find_matching_file;
fn main() {
    let base_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_path = base_dir
        .join("assets")
        .join("python")
        .join("kiwi-.*-py3-none-any\\.whl");
    let pattern = target_path.to_str().unwrap();
    if let Ok(Some(file)) = find_matching_file(pattern) {
        println!("{:#?}", file);
    }
}
