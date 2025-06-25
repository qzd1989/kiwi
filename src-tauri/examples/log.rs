use std::{thread, time::Duration};

use kiwi_lib::app;
fn main() {
    app::Log::info("info from fuck you")
        .write_to_file("a.log")
        .unwrap();
    thread::sleep(Duration::from_secs(1));
    app::Log::warn("warn from fuck you")
        .write_to_file("a.log")
        .unwrap();
    thread::sleep(Duration::from_secs(2));
    app::Log::error("error from fuck you")
        .write_to_file("a.log")
        .unwrap();
    thread::sleep(Duration::from_secs(1));
    app::Log::info("info from fuck you222")
        .write_to_file("a.log")
        .unwrap();
}
