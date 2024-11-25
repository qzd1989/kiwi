use std::{thread::sleep, time::Duration};

use kiwi_lib::capture::primary_display_listen;
use kiwi_lib::common::PYTHON_EXEC_FILE;
use kiwi_lib::frontend_commands::run;

#[tokio::main]
async fn main() {
    let file = "C:\\Users\\kiwi\\rust\\kiwi\\src-tauri\\main.py".to_string();
    let file = "C:\\main.py".to_string();
    println!("{:?} {:?}", PYTHON_EXEC_FILE.to_string(), file);
    primary_display_listen().await;
    run(file);
    sleep(Duration::from_secs(1000));
}
