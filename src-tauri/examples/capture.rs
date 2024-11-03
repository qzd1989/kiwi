use image::{ImageBuffer, Rgba};
use kiwi_lib::capture;

#[tokio::main]
async fn main() {
    let display = capture::engine::get_primary_display().await;
    // capture::engine::frame(display, |frame| {
    //     println!("{:?} - {:?}", frame.width, frame.height);
    // })
    // .await;
    let frame = capture::engine::snapshot(display).await;
    println!("{:?} - {:?}", frame.width, frame.height);
    let img =
        ImageBuffer::<Rgba<u8>, _>::from_vec(frame.width, frame.height, frame.buffer).unwrap();
    img.save("b.png").unwrap();
    println!("done");
}
