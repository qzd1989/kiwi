use std::time::Duration;

use kiwi_lib::grpc::{self, RUN_TIME};

fn main() {
    RUN_TIME.spawn(async move {
        grpc::run().await.unwrap();
    });
    std::thread::sleep(Duration::from_secs(1000));
}
