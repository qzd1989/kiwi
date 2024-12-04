use kiwi_lib::capture::listen_primary_display;
use std::{thread::sleep, time::Duration};

fn main() {
    listen_primary_display();
    sleep(Duration::from_secs(1000));
}
