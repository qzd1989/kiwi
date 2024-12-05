use std::time::Instant;

use kiwi_lib::grpc::FindImageRequest;

fn main() {
    let time = Instant::now();
    let path = "payment".to_string();
    let start_x = 15;
    let start_y = 20;
    let end_x = 100;
    let end_y = 100;
    let threshold = 0.957;
    let request = FindImageRequest::new(path, start_x, start_y, end_x, end_y, threshold);
    let result = request.send();
    println!("{:?}", result);
    println!("eslaped: {:?}", time.elapsed());
}
