fn main() {
    // println!("cargo:rustc-link-search=native=C:\\tools\\opencv\\build\\x64\\vc15\\lib");
    // println!("cargo:rustc-link-lib=static=opencv_world455");
    tauri_build::build()
}
