use kiwi_lib::capture::{listen_primary_display, IS_CAPTURING};
use std::{thread::sleep, time::Duration};

fn main() {
    *IS_CAPTURING.lock().unwrap() = true;
    listen_primary_display();
    sleep(Duration::from_secs(1000));
}
